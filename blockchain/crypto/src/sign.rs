use std::io::{self, Write};

use openpgp::serialize::stream::*;
use openpgp::parse::stream::*;

const MESSAGE: &'static str = "дружба";

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

struct Helper<'a> {
    tpk: &'a openpgp::TPK,
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
                },
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
