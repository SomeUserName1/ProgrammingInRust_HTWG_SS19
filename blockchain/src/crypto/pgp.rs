//! OpenPGP implementations. Can be used to generate, sign, verify, encrypt, decrypt, import
//! and export PGP keys. Not used since there is no networking.

use std::collections::HashMap;
use std::io::{self, Write, Read};
use uuid::Uuid;
use sequoia_openpgp::TPK;
use sequoia_openpgp::serialize::stream::*;
use sequoia_openpgp::parse::{stream::*, Parse};
use sequoia_openpgp::constants::SymmetricAlgorithm;
use sequoia_openpgp::crypto::{self, SessionKey};
use sequoia_openpgp::tpk::armor::Encoder;
use failure;
use sequoia_openpgp::serialize::Serialize;
use sequoia_openpgp::packet::Signature;

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
pub fn generate(node_uuid: Uuid) -> sequoia_openpgp::Result<(sequoia_openpgp::TPK, sequoia_openpgp::packet::Signature)> {
    let (tpk, revocation) = sequoia_openpgp::tpk::TPKBuilder::general_purpose(
        sequoia_openpgp::tpk::CipherSuite::RSA3k, Some(node_uuid.to_string()))
        .generate()?;

    Ok((tpk, revocation))
}

/// Generates only an encryption-capable key.
fn generate_crypt(node_uuid: Uuid) -> sequoia_openpgp::Result<(TPK, Signature)> {
    sequoia_openpgp::tpk::TPKBuilder::new()
        .add_userid(node_uuid.to_string())
        .add_encryption_subkey()
        .generate()
}

/// Imports a TPK from a file.
/// Just delegates to [`TPK::from_file`], nothing special.
pub fn import_key(path: &str) -> Result<TPK, failure::Error> {
    sequoia_openpgp::TPK::from_file(path)
}

/// Exports a TPK´s public key to a file with filename equal to user´s id.
/// Can be used to exchange keys via trusted services.
/// Returns the name of the created file.
/// Note: secret keys not attached
pub fn export_key(tpk: &sequoia_openpgp::TPK) -> Result<String, failure::Error> {
    // Extracts the generic user id from a TPK.
    // The TPK´s user id´s value is a str, except we didnt create the key
    let id = std::str::from_utf8(tpk.userids().next()
        .map(|opt| opt.userid())
        .map(|uid| uid.value()).get_or_insert(b"unknown_uid")).unwrap();
    let file_name = format!("{}.pgp", id);
    let mut file = std::fs::File::create(file_name.as_str())?;
    // Serializes the TPK to ascii armored file
    let encoder = Encoder::new(tpk);
    encoder.serialize(&mut file)?;
    Ok(file_name)
}


/// Signs a source of data and writes the result into destination with the given key.
/// Will prompt for a password if the tsk is encrypted.
fn sign_detached<R: Read, W: Write>(tsk: TPK, mut src: &mut R, dest: &mut W) -> Result<(), failure::Error> {
    let mut keys = Vec::new();
    for (_, _, key) in tsk.keys_valid().signing_capable().secret(true) {
        keys.push({
            let mut key = key.clone();
            if key.secret().expect("filtered").is_encrypted() {
                let password = rpassword::read_password_from_tty(
                    Some(&format!("Please enter password to decrypt \
                                       {}/{}: ", tsk, key))).unwrap();
                let algo = key.pk_algo();
                key.secret_mut()
                    .expect("")
                    .decrypt_in_place(algo, &password.into())?;
            }
            key.into_keypair()?
        });
    }

    let sink = sequoia_openpgp::armor::Writer::new(dest, sequoia_openpgp::armor::Kind::Signature, &[])?;
    let message = Message::new(sink);

    let mut signer = Signer::detached(
        message,
        keys.iter_mut().map(|s| -> &mut dyn crypto::Signer { s }).collect(),
        None)?;

    io::copy(&mut src, &mut signer)?;
    signer.finalize()?;

    Ok(())
}

/// Signs the given message.
fn sign(sink: &mut Write, data: &[u8], tsk: &TPK)
        -> sequoia_openpgp::Result<()> {
    let mut keypair = tsk.keys_valid().signing_capable().nth(0).unwrap().2
        .clone().into_keypair()?;

    let message = Message::new(sink);
    let signer = Signer::new(message, vec![&mut keypair], None)?;
    let mut literal_writer = LiteralWriter::new(
        signer, sequoia_openpgp::constants::DataFormat::Binary, None, None)?;
    literal_writer.write_all(data)?;
    literal_writer.finalize()?;

    Ok(())
}

/// Verifies the given message.
/// Returns:
/// Ok(()) on successful verification, Err(_) on failure.
fn verify(sink: &mut Write, signed_message: &[u8], sender: &TPK)
          -> sequoia_openpgp::Result<()> {

    let helper = SignHelper {
        tpk: sender,
    };
    let mut verifier = Verifier::from_bytes(signed_message, helper, None)?;
    io::copy(&mut verifier, sink)?;

    Ok(())
}

/// Encrypts the given message and writes the result to sink.
fn encrypt(sink: &mut Write, plaintext: &str, recipient: &sequoia_openpgp::TPK)
           -> sequoia_openpgp::Result<()> {

    let message = Message::new(sink);
    let encryptor = Encryptor::new(message,
                                   &[], // No symmetric encryption.
                                   &[recipient],
                                   EncryptionMode::ForTransport,
                                   None)?;

    let mut literal_writer = LiteralWriter::new(
        encryptor, sequoia_openpgp::constants::DataFormat::Binary, None, None)?;

    literal_writer.write_all(plaintext.as_bytes())?;
    literal_writer.finalize()?;

    Ok(())
}

/// Decrypts the given message.
fn decrypt(sink: &mut Write, ciphertext: &[u8], recipient: &sequoia_openpgp::TPK)
           -> sequoia_openpgp::Result<()> {

    let helper = CryptHelper {
        secret: recipient,
    };
    let mut decryptor = Decryptor::from_bytes(ciphertext, helper, None)?;
    io::copy(&mut decryptor, sink)?;
    Ok(())
}

/// A wrapper holding a TPK.
/// Required by Verifier.
pub struct SignHelper<'a> {
    tpk: &'a TPK,
}

/// A wrapper holding a TSK.
/// Required by Decryptor.
struct CryptHelper<'a> {
    secret: &'a TPK,
}

/// Implementation of our signature verification policy ala sequoia examples.
impl<'a> VerificationHelper for SignHelper<'a> {
    /// Returns:
    /// Public keys for signature verification
    fn get_public_keys(&mut self, _ids: &[sequoia_openpgp::KeyID])
                       -> sequoia_openpgp::Result<Vec<TPK>> {
        Ok(vec![self.tpk.clone()])
    }

    /// Actual implementation for our signature check.
    /// Returns:
    /// Ok(()) for successful verification
    /// Err(_) for bad checksum / bad signature / no signature
    fn check(&mut self, structure: &MessageStructure)
             -> sequoia_openpgp::Result<()> {

        let mut good = false;
        for (i, layer) in structure.iter().enumerate() {
            match (i, layer) {
                (0, MessageLayer::SignatureGroup { ref results }) => {
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


impl<'a> VerificationHelper for CryptHelper<'a> {
    fn get_public_keys(&mut self, _ids: &[sequoia_openpgp::KeyID])
                       -> sequoia_openpgp::Result<Vec<sequoia_openpgp::TPK>> {
        Ok(Vec::new())
    }

    fn check(&mut self, _structure: &MessageStructure)
             -> sequoia_openpgp::Result<()> {
        Ok(())
    }
}

/// Implementation of our decryption policy ala sequoia examples.
impl<'a> DecryptionHelper for CryptHelper<'a> {
    /// Actual method used for decryption.
    /// Expects the decryption key to be the second subkey.
    /// Expects the secret key to not be encrypted.
    /// Returns:
    /// Ok(()) for successful verification
    /// Err(_) for bad checksum / bad signature / no signature
    fn decrypt<D>(&mut self,
                  pkesks: &[sequoia_openpgp::packet::PKESK],
                  _skesks: &[sequoia_openpgp::packet::SKESK],
                  mut decrypt: D)
                  -> sequoia_openpgp::Result<Option<sequoia_openpgp::Fingerprint>>
        where D: FnMut(SymmetricAlgorithm, &SessionKey) -> sequoia_openpgp::Result<()>
    {
        let key = self.secret.subkeys().nth(0)
            .map(|binding| binding.subkey().clone())
            .unwrap();

        let mut pair = key.into_keypair().unwrap();

        pkesks[0].decrypt(&mut pair)
            .and_then(|(algo, session_key)| decrypt(algo, &session_key))
            .map(|_| None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MESSAGE: &'static str = "дружба";

    #[test]
    fn test_generation() {
        let user = Uuid::new_v4();
        let (tpk, _) = generate(user)
            .unwrap();
        assert!(tpk.keys_valid().count() > 0);
        let id = std::str::from_utf8(tpk.userids().next()
            .map(|opt| opt.userid())
            .map(|uid| uid.value()).get_or_insert(b"unknown_uid")).unwrap();
        assert_eq!(id, user.to_string());
    }

    #[test]
    fn test_import_export() {
        let user = Uuid::new_v4();
        let (key, _) = generate(user).unwrap();
        let file_name = export_key(&key).unwrap();
        let imported_key = import_key(file_name.as_str()).unwrap();
        std::fs::remove_file(file_name).unwrap();
        assert_ne!(key, imported_key); // keys should not be equal, exporting strips off attached secret keys
    }


    #[test]
    fn test_crypt() {
        let user = Uuid::new_v4();
        let (key, _) = generate_crypt(user).unwrap();

        let mut ciphertext = Vec::new();

        encrypt(&mut ciphertext, MESSAGE, &key).unwrap();

        let mut plaintext = Vec::new();
        decrypt(&mut plaintext, &ciphertext, &key).unwrap();

        assert_eq!(MESSAGE.as_bytes(), &plaintext[..]);
    }

    #[test]
    fn test_sign() {
        let user = Uuid::new_v4();
        let (key, _) = generate(user).unwrap();

        let mut signed_message = Vec::new();
        sign(&mut signed_message, MESSAGE.as_bytes(), &key).unwrap();

        let mut plaintext = Vec::new();
        verify(&mut plaintext, &signed_message, &key).unwrap();

        assert_eq!(MESSAGE.as_bytes(), &plaintext[..]);
    }
}
