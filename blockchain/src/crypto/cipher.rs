use std::str;
use std::io::{self, Write};

use sequoia_openpgp as openpgp;
use openpgp::crypto::SessionKey;
use openpgp::constants::SymmetricAlgorithm;
use openpgp::serialize::stream::*;
use openpgp::parse::stream::*;

use super::sign::Helper;

//TODO: sign and verify transactions, not bytes
//TODO: encrypt and decrypt messages

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
