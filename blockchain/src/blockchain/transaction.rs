use std::marker::Sized;
use std::clone::Clone;
use std::fmt::Debug;
use std::fmt::Write;
use std::sync::{Arc, RwLock};

use serde::{Serialize, Deserialize, de::DeserializeOwned};


/// The transaction stored in a block of the blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction<T> {
    /// The sender of the transaction.
    pub sender: String,
    /// The payload of the transaction.
    pub payload: Arc<RwLock<T>>,
}


impl<T> Transaction<T>
    where T: Debug {
    /// Formats a transaction with all information.
    pub fn fmt(&self) -> String {
        let mut str = String::new();

        write!(&mut str, "            Transaction: [\n").expect("[Transaction fmt()]: Unable to write in Buffer!");
        write!(&mut str, "                Sender:   {}\n", self.sender).expect("[Transaction fmt()]: Unable to write in Buffer!");
        write!(&mut str, "                Payload: [\n").expect("[Transaction fmt()]: Unable to write in Buffer!");
        write!(&mut str, "                    {:?}\n", self.payload).expect("[Transaction fmt()]: Unable to write in Buffer!");
        write!(&mut str, "                ]\n").expect("[Transaction fmt()]: Unable to write in Buffer!");
        write!(&mut str, "           ]\n").expect("[Block fmt()]: Unable to write in Buffer!");

        str
    }
}

pub trait Transactional
where Self: Sized + Send + Serialize + DeserializeOwned + PartialEq + Eq + Debug + Clone {
    /// Creates a new transaction with a sender and the specified payload.
    fn new(sender: String, payload: Self) -> Transaction<Self> { // , key: sequoia_openpgp::TPK
        Transaction {
            sender,
            payload: Arc::new(RwLock::new(payload)),
            // FIXME create the sender signature with the key. 
        }
    }

    fn genesis(miner_address: String, reward: u32) -> Transaction<Self>;
}

// Examples: Crypto currency, Code, voting, timestamping of arbitary objects
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// A payload for a cryptographc currency.
pub struct CryptoPayload {
    /// The receiver of the transaction.
    pub receiver: String,
    /// The amount of coins
    pub amount: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// A payload for a voting system.
pub struct VotePayload {
    /// The voted party from the sender.
    pub vote: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// A payload for a version control system.
pub struct CodePayload {
    /// The name of the file.
    pub file_name: String,
    /// The content of the file.
    pub contents: String,
    /// The commit message.
    pub commit_message: String,
}

impl Transactional for CryptoPayload {
    fn genesis(miner_address: String, reward: u32) -> Transaction<CryptoPayload> {
        Transaction {
            sender: String::from("Root"),
            payload: Arc::new(RwLock::new(CryptoPayload {
                receiver: miner_address,
                amount: reward,
            })),
        }
    }
}

impl Transactional for VotePayload {
    fn genesis(_miner_address: String, _reward: u32) -> Transaction<VotePayload> {
        Transaction {
            sender: String::from("Root"),
            payload: Arc::new(RwLock::new(VotePayload {
                vote: String::from("Root"),
            })),
        }
    }
}

impl Transactional for CodePayload {
    fn genesis(_miner_address: String, _reward: u32) -> Transaction<CodePayload> {
        Transaction {
            sender: String::from("Root"),
            payload: Arc::new(RwLock::new(CodePayload {
                file_name: String::from("Readme.md"),
                contents: String::from(""),
                commit_message: String::from("Initialize Repository"),
            })),
        }
    }
}


