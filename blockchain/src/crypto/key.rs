use sequoia_openpgp as openpgp;
use uuid::Uuid;

/// Generates an encryption-capable key.
pub fn generate(node_uuid: Uuid) -> openpgp::Result<(openpgp::TPK, openpgp::packet::Signature)> {
    let (tpk, revocation) = openpgp::tpk::TPKBuilder::general_purpose(
        openpgp::tpk::CipherSuite::RSA3k, Some(node_uuid.to_string()))
        .generate()?;

    Ok((tpk, revocation))
}



