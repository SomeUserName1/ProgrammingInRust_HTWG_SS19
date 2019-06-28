/// data structure to maintain the chain
use crate::crypto::hash;
use super::block::{Block, BlockHeader};
use super::transaction::{Transaction, Transactional};

pub struct Chain<T: serde::Serialize + std::fmt::Debug + std::clone::Clone + Transactional<T>> {
    chain: Vec<Block<T>>,
    curr_trans: Vec<Transaction<T>>,
    difficulty: u32,
    miner_addr: String, 
    reward: f32,
}

impl<T: serde::Serialize + std::fmt::Debug + std::clone::Clone + Transactional<T>> Chain<T>
    where T: serde::Serialize + std::fmt::Debug {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain<T> {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
         };

        chain.add_new_block();
        chain

    } 

    pub fn add_transaction(&mut self, transactions: &mut Vec<Transaction<T>>) ->
    bool {
        self.curr_trans.append(transactions);
        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap()
        };
        hash::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    pub fn add_new_block(&mut self) -> bool {
        let mut block = Block::<T>::new(
            self.last_hash(), self.difficulty,
                               self.miner_addr.clone(), self.reward, &mut self.curr_trans);


        //Chain::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

        pub fn proof_of_work(header: &mut BlockHeader) {
        loop {
            let hash = hash::hash(header);
            let slice = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                },
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            };
        }
    }
}
