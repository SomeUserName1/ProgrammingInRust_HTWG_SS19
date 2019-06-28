use serde::{Serialize, Deserialize};
use chrono;

/// one possible kind of transaction data structure
#[derive(Debug, Clone, Serialize)]
pub struct Transaction<T> {
    pub sender: String,
    pub payload: T,
}

trait Transactional<T>
    where Self: std::marker::Sized {
    fn new(sender: String, payload: T) -> Transaction<T> {
        Transaction {
            sender,
            payload
        }
    }
}

// Examples: Crypto currency, Code, voting, timestamping of arbitary objects
#[derive(Debug, Clone, Serialize)]
pub struct CryptoPayload {
    receiver: String,
    amount: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct VotePayload {
    vote: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CodePayload {
    code: String,
    commit_message: String,
}


pub struct TimestampObjectPayload<O> {
    object: O,
    timestamp: chrono::Utc,
}

impl Transactional<CryptoPayload> for CryptoPayload {}
impl Transactional<VotePayload> for VotePayload {}
impl Transactional<CodePayload> for CodePayload {}
