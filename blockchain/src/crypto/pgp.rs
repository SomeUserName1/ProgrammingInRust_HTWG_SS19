use sequoia_openpgp as openpgp;
use openpgp::TPK;
use openpgp::serialize::{SerializeInto, Serialize};
use openpgp::parse::Parse;
use openpgp::crypto::SessionKey;
use openpgp::constants::SymmetricAlgorithm;
use openpgp::serialize::stream::*;
use openpgp::parse::stream::*;
use bytes::{BytesMut, BufMut};
use tokio::codec::{Encoder, Decoder};
use serde::{Serialize, de::DeserializeOwned};
use serde_json;
use uuid::Uuid;
use std::str;
use std::marker::PhantomData;
use std::io::{self, Write};
use std::collections::HashMap;

use super::messages::Messages;
use crate::blockchain::transaction::Transactional;

//TODO: sign and verify transactions, not bytes
//TODO: encrypt and decrypt messages

/// Simple Keyring
/// Just maps known ids to TPKs
struct Keyring {
    peers: HashMap<Uuid, TPK>,
    key: TPK,
    user: Uuid,
}

/// Generates an encryption-capable key.
/// The key's primary key is certification- and signature-capable.
/// The key has one subkey, an encryption-capable subkey.
pub fn generate(node_uuid: Uuid) -> openpgp::Result<(openpgp::TPK, openpgp::packet::Signature)> {
    let (tpk, revocation) = openpgp::tpk::TPKBuilder::general_purpose(
        openpgp::tpk::CipherSuite::RSA3k, Some(node_uuid.to_string()))
        .generate()?;

    Ok((tpk, revocation))
}

/// Imports a TPK from a file.
/// Just delegates to [`TPK::from_file`], nothing special.
pub fn import_key(path: &str) -> Result<TPK, failure::Error> {
    openpgp::TPK::from_file(path)
}

/// Exports a TPK´s public key to a file with filename equal to user´s id.
/// Can be used to exchange keys on a secure channel.
/// Returns the name of the created file.
/// Note: this will not store any attached secret keys,
/// because they should be encrypted beforehand!
pub fn export_key(tpk: &openpgp::TPK) -> Result<String, failure::Error> {
    /// Extracts the generic userid from a TPK.
    /// The TPK´s userid´s value is a str, except we didnt create the key
    let id = match std::str::from_utf8(
        tpk.userids().next()
            .map(|opt| opt.userid())
            .map(|uid| uid.value())
            .expect("no uid present")) {
        Ok(string) => string,
        Err(_) => "unkown_uid",
    };
    let file_name = format!("{}.pgp", id);
    let mut file = std::fs::File::create(file_name.as_str())?;
    /// Serializes the TPK to ascii armored file
    openpgp::tpk::armor::Encoder::new(&tpk).serialize(&mut file)?;
    Ok(file_name)
}


/// Signs the given message.
fn sign(sink: &mut Write, plaintext: &str, tsk: &openpgp::TPK)
        -> openpgp::Result<()> {
    // Get the keypair to do the signing from the TPK.
    let mut keypair = tsk.keys_valid().signing_capable().nth(0).unwrap().2
        .clone().into_keypair()?;

    // Start streaming an OpenPGP message.
    let message = Message::new(sink);

    // We want to sign a literal data packet.
    let signer = Signer::new(message, vec![&mut keypair], None)?;

    // Emit a literal data packet.
    let mut literal_writer = LiteralWriter::new(
        signer, openpgp::constants::DataFormat::Binary, None, None)?;

    // Sign the data.
    literal_writer.write_all(plaintext.as_bytes())?;

    // Finalize the OpenPGP message to make sure that all data is
    // written.
    literal_writer.finalize()?;

    Ok(())
}

/// Verifies the given message.
fn verify(sink: &mut Write, signed_message: &[u8], sender: &openpgp::TPK)
          -> openpgp::Result<()> {
    // Make a helper that that feeds the sender's public key to the
    // verifier.
    let helper = Helper {
        tpk: sender,
    };

    // Now, create a verifier with a helper using the given TPKs.
    let mut verifier = Verifier::from_bytes(signed_message, helper, None)?;

    // Verify the data.
    io::copy(&mut verifier, sink)?;

    Ok(())
}

pub struct Helper<'a> {
    pub tpk: &'a openpgp::TPK,
}

impl<'a> VerificationHelper for Helper<'a> {
    fn get_public_keys(&mut self, _ids: &[openpgp::KeyID])
                       -> openpgp::Result<Vec<openpgp::TPK>> {
        // Return public keys for signature verification here.
        Ok(vec![self.tpk.clone()])
    }

    fn check(&mut self, structure: &MessageStructure)
             -> openpgp::Result<()> {
        // In this function, we implement our signature verification
        // policy.

        let mut good = false;
        for (i, layer) in structure.iter().enumerate() {
            match (i, layer) {
                // First, we are interested in signatures over the
                // data, i.e. level 0 signatures.
                (0, MessageLayer::SignatureGroup { ref results }) => {
                    // Finally, given a VerificationResult, which only says
                    // whether the signature checks out mathematically, we apply
                    // our policy.
                    match results.get(0) {
                        Some(VerificationResult::GoodChecksum(..)) =>
                            good = true,
                        Some(VerificationResult::MissingKey(_)) =>
                            return Err(failure::err_msg(
                                "Missing key to verify signature")),
                        Some(VerificationResult::BadChecksum(_)) =>
                            return Err(failure::err_msg("Bad signature")),
                        None =>
                            return Err(failure::err_msg("No signature")),
                    }
                }
                _ => return Err(failure::err_msg(
                    "Unexpected message structure")),
            }
        }

        if good {
            Ok(()) // Good signature.
        } else {
            Err(failure::err_msg("Signature verification failed"))
        }
    }
}

/// Encrypts the given message.
fn encrypt(sink: &mut Write, plaintext: &str, recipient: &openpgp::TPK)
           -> openpgp::Result<()> {
    // Start streaming an OpenPGP message.
    let message = Message::new(sink);

    // We want to encrypt a literal data packet.
    let encryptor = Encryptor::new(message,
                                   &[], // No symmetric encryption.
                                   &[recipient],
                                   EncryptionMode::ForTransport,
                                   None)?;

    // Emit a literal data packet.
    let mut literal_writer = LiteralWriter::new(
        encryptor, openpgp::constants::DataFormat::Binary, None, None)?;

    // Encrypt the data.
    literal_writer.write_all(plaintext.as_bytes())?;

    // Finalize the OpenPGP message to make sure that all data is
    // written.
    literal_writer.finalize()?;

    Ok(())
}

/// Decrypts the given message.
fn decrypt(sink: &mut Write, ciphertext: &[u8], recipient: &openpgp::TPK)
           -> openpgp::Result<()> {
    // Make a helper that that feeds the recipient's secret key to the
    // decryptor.
    let helper = Helper {
        tpk: recipient,
    };

    // Now, create a decryptor with a helper using the given TPKs.
    let mut decryptor = Decryptor::from_bytes(ciphertext, helper, None)?;

    // Decrypt the data.
    io::copy(&mut decryptor, sink)?;

    Ok(())
}


impl<'a> DecryptionHelper for Helper<'a> {
    fn decrypt<D>(&mut self,
                  pkesks: &[openpgp::packet::PKESK],
                  _skesks: &[openpgp::packet::SKESK],
                  mut decrypt: D)
                  -> openpgp::Result<Option<openpgp::Fingerprint>>
        where D: FnMut(SymmetricAlgorithm, &SessionKey) -> openpgp::Result<()>
    {
        // The encryption key is the first and only subkey.
        let key = self.tpk.subkeys().nth(0)
            .map(|binding| binding.subkey().clone())
            .unwrap();

        // The secret key is not encrypted.
        let mut pair = key.into_keypair().unwrap();

        pkesks[0].decrypt(&mut pair)
            .and_then(|(algo, session_key)| decrypt(algo, &session_key))
            .map(|_| None)
        // XXX: In production code, return the Fingerprint of the
        // recipient's TPK here
    }
}
