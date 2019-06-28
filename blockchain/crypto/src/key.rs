use std::io::{self, Write};


use openpgp::crypto::SessionKey;
use openpgp::constants::SymmetricAlgorithm;
use openpgp::serialize::stream::*;
use openpgp::parse::stream::*;

const MESSAGE: &'static str = "дружба";

/// Generates an encryption-capable key.
fn generate() -> openpgp::Result<openpgp::TPK> {
    let (tpk, _revocation) = openpgp::tpk::TPKBuilder::new()
        .add_userid("someone@example.org")
        .add_encryption_subkey()
        .generate()?;

    // Save the revocation certificate somewhere.

    Ok(tpk)
}



