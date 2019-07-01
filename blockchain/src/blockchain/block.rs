/// basic block data structure
use std::borrow::BorrowMut;

use serde::Serialize;

use crate::blockchain::transaction::{Transaction, Transactional};
use crate::crypto::merkle;

#[derive(Serialize, Debug, Clone)]
pub struct BlockHeader {
    timestamp: i64,
    pub nonce: u32, 
    pre_hash: String,  
    merkle: String,  
    pub difficulty: u32,
}


#[derive(Serialize, Debug, Clone)]
pub struct Block<T: serde::Serialize + std::fmt::Debug + std::clone::Clone + Transactional> {
    pub header: BlockHeader,
    count: u32,
    transactions: Vec<Transaction<T>>
}

impl<T: serde::Serialize + std::fmt::Debug + std::clone::Clone + Transactional> Block<T> {
    pub fn new(
        hash: String, 
        difficulty: u32, 
        miner_address: String,
        reward: f32,
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
