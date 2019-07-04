use std::marker::Sized;
use std::clone::Clone;
use std::fmt::Debug;

use serde::{Serialize, Deserialize};


/// one possible kind of transaction data structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction<T> {
    pub sender: String,
    pub payload: T,
}

pub trait Transactional
    where Self: Sized + Serialize + Debug + Clone {
    fn new(sender: String, payload: Self) -> Transaction<Self> {
        Transaction {
            sender,
            payload
        }
    }

    fn genesis(miner_address: String, reward: f32) -> Transaction<Self>;
}

// Examples: Crypto currency, Code, voting, timestamping of arbitary objects
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CryptoPayload {
    pub receiver: String,
    pub amount: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VotePayload {
    pub vote: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodePayload {
    pub file_name: String,
    pub contents: String,
    pub commit_message: String,
}

impl Transactional for CryptoPayload {
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
impl Transactional for VotePayload {
    fn genesis(_miner_address: String, _reward: f32) -> Transaction<VotePayload> {
        Transaction {
            sender: String::from("Root"),
            payload: VotePayload {
                vote: String::from("Root"),
            }
        }
    }
}
impl Transactional for CodePayload {
    fn genesis(_miner_address: String, _reward: f32) -> Transaction<CodePayload>  {
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


