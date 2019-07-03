use sequoia_openpgp as openpgp;
use uuid::Uuid;
use openpgp::serialize::{SerializeInto, Serialize};
use openpgp::parse::Parse;
use std::io::Write;
use openpgp::TPK;

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