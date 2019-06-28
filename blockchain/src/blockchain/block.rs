/// basic block data structure
use hash;
use transaction::Transaction;
use std::borrow::BorrowMut;

#[derive(Serialize, Debug)]
pub struct BlockHeader {
    timestamp: i64,
    pub nonce: u32, 
    pre_hash: String,  
    merkle: String,  
    pub difficulty: u32,
}


#[derive(Serialize, Debug)]
pub struct Block {
    pub header: BlockHeader,
    count: u32,
    transactions: Vec<Transaction>
}

impl Block {
    pub fn new(
        hash: String, 
        difficulty: u32, 
        miner_address: String,
        reward: f32,
        transactions: &mut Vec<Transaction> 
                                        ) -> Self {
        let header = BlockHeader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: hash,
            merkle: String::new(),
            difficulty
        };

        let reward_trans = Transaction {
            sender: String::from("Root"),
            receiver: miner_address,
            amount: reward
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![]
        };

        block.transactions.push(reward_trans);
        block.transactions.append(transactions.borrow_mut());
        block.count = block.transactions.len() as u32;
        block.header.merkle = Block::get_merkle(block.transactions.clone());
        block
    }
}
