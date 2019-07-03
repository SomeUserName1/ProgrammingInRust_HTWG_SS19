use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::clone::Clone;

use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::blockchain::transaction::{Transaction, Transactional};
use crate::crypto::merkle;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
    timestamp: i64,
    pub nonce: u32, 
    pre_hash: String,  
    merkle: String,  
    pub difficulty: u32,
}

impl PartialEq for BlockHeader {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp.eq(&other.timestamp) && self.pre_hash.eq(&other.pre_hash)
            && self.merkle.eq(&other.merkle)
    }
}

impl Eq for BlockHeader {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block<T> {
    pub header: BlockHeader,
    count: u32,
    transactions: Vec<Transaction<T>>
}

impl<T> PartialEq for Block<T> {
    fn eq(&self, other: &Self) -> bool {
        self.header.eq(&other.header)
    }
}

impl<T> Eq for Block<T> {}

impl<T: Serialize + DeserializeOwned + Debug + Clone + Transactional + PartialEq> Block<T> {
    pub fn new(
        hash: String, 
        difficulty: u32, 
        miner_address: String,
        reward: u32,
        transactions: &mut Vec<Transaction<T>>
                                        ) -> Self {
        let header = BlockHeader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: hash,
            merkle: String::new(),
            difficulty
        };

        let reward_trans = T::genesis(miner_address, reward);

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![]
        };

        block.transactions.push(reward_trans);
        block.transactions.append(transactions.borrow_mut());
        block.count = block.transactions.len() as u32;
        block.header.merkle = merkle::get_merkle(block.transactions.clone());
        block
    }
}
