extern crate serde;
extern crate chrono;

use serde::{Serialize, Deserialize};

/// one possible kind of transaction data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction<T: serde::Serialize + std::fmt::Debug + std::clone::Clone> {
    pub sender: String,
    pub payload: T,
}

pub trait Transactional<T: serde::Serialize + std::fmt::Debug + std::clone::Clone>
    where Self: std::marker::Sized {
    fn new(sender: String, payload: T) -> Transaction<T> {
        Transaction {
            sender,
            payload
        }
    }

    fn genesis(miner_address: String, reward: f32) -> Transaction<T>;
}

// Examples: Crypto currency, Code, voting, timestamping of arbitary objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoPayload {
    pub receiver: String,
    pub amount: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotePayload {
    pub vote: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodePayload {
    pub file_name: String,
    pub contents: String,
    pub commit_message: String,
}


pub struct TimestampObjectPayload<O> {
    pub object: O,
    pub timestamp: chrono::Utc,
}

impl Transactional<CryptoPayload> for CryptoPayload {
    fn genesis(miner_address: String, reward: f32) -> Transaction<CryptoPayload> {
        Transaction {
            sender: String::from("Root"),
            payload: CryptoPayload {
                    receiver: miner_address,
                    amount: reward
                }
        }
    }
}
impl Transactional<VotePayload> for VotePayload {
    fn genesis(miner_address: String, reward: f32) -> Transaction<VotePayload> {
        Transaction {
            sender: String::from("Root"),
            payload: VotePayload {
                vote: String::from("Root"),
            }
        }
    }
}
impl Transactional<CodePayload> for CodePayload {
    fn genesis(miner_address: String, reward: f32) -> Transaction<CodePayload>  {
        Transaction {
            sender: String::from("Root"),
            payload: CodePayload {
                file_name: String::from("Readme.md"),
                contents: String::from(""),
                commit_message: String::from("Initialize Repository"),
            }
        }
    }
}


