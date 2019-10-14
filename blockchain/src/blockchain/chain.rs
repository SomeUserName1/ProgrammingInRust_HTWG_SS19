/// data structure to maintain the chain
use std::fmt::Debug;
use std::clone::Clone;
use std::fmt::Write;

use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::crypto::hash;

use super::block::{Block, BlockHeader};
use super::transaction::{Transaction, Transactional};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chain<T> { 
    chain: Vec<Block<T>>,
    curr_trans: Vec<Transaction<T>>,
    difficulty: u32,
    miner_addr: String,
    reward: u32,
}

impl<T> Chain<T>
where T: Serialize + DeserializeOwned + Debug + Clone + Transactional + Send 
{
    pub fn new(miner_addr: String, difficulty: u32) -> Chain<T> {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100,
         };

        chain.add_new_block();
        chain
    }

    pub fn add_transaction(&mut self, transactions: &mut Vec<Transaction<T>>) ->
    bool {
        self.curr_trans.append(transactions);

        if self.curr_trans.len() > 20 {
            self.add_new_block();
        }
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

    pub fn update_reward(&mut self, reward: u32) -> bool {
        self.reward = reward;
        true
    }

    pub fn add_new_block(&mut self) -> bool  {
        let mut block = Block::<T>::new(
            self.last_hash(), self.difficulty,
            self.miner_addr.clone(), self.reward, &mut self.curr_trans);


        Chain::<T>::proof_of_work(&mut block.header);
        println!("{}", &block.fmt());
        self.chain.push(block);
        self.curr_trans.clear();
        if self.chain.len() % 100 == 0 {
           self.difficulty += 1; 
           self.reward += 1;
        }
        true
    }

    pub fn get_no_curr_trans(&self) -> usize {
        self.curr_trans.len()
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
                }
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            };
        }
    }

    pub fn fmt(&self) -> String {
        let mut str = String::new();

        write!(&mut str, "Chain [\n").expect("[Chain fmt()]: Unable to write in Buffer!");

        for block in &self.chain {
            write!(&mut str, "{}", block.fmt()).expect("[Chain fmt()]: Unable to write in Buffer!");
        }

        write!(&mut str, "    Current Transactions: [\n").expect("[Chain fmt()]: Unable to write in Buffer!");

        for trans in &self.curr_trans {
            write!(&mut str, "{:?}", trans.fmt()).expect("[Chain fmt()]: Unable to write in Buffer!");
        }

        write!(&mut str, "    ]\n").expect("[Chain fmt()]: Unable to write in Buffer!");
        write!(&mut str, "    Difficulty:    {}\n", &self.difficulty).expect("[Chain fmt()]: Unable to write in Buffer!");
        write!(&mut str, "    Miner address: {}\n", &self.miner_addr).expect("[Chain fmt()]: Unable to write in Buffer!");
        write!(&mut str, "]\n").expect("[Chain fmt()]: Unable to write in Buffer!");

        str
    }
}

impl<T> PartialEq for Chain<T>
where T: Serialize + DeserializeOwned + Transactional + Clone + Transactional
{
    fn eq(&self, other: &Self) -> bool {
        self.chain.first().eq(&other.chain.first())
    }
}

impl<T> Eq for Chain<T>
where T: Transactional + DeserializeOwned
{}

