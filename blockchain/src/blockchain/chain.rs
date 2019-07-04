use std::fmt::{Debug, Write};
use std::clone::Clone;

use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::crypto::hash;

use super::block::{Block, BlockHeader};
use super::transaction::{Transaction, Transactional};

/// data structure to maintain the chain
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Chain<T> {
    chain: Vec<Block<T>>,
    curr_trans: Vec<Transaction<T>>,
    difficulty: u32,
    miner_addr: String,
    reward: u32,
}

impl<T: Serialize + DeserializeOwned + Debug + Clone + PartialEq + Transactional> Chain<T>
    where T: serde::Serialize + std::fmt::Debug {
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

    pub fn add_transaction(&mut self, transactions: &mut Vec<Transaction<T>>) -> bool {
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

    pub fn update_reward(&mut self, reward: u32) -> bool {
        self.reward = reward;
        true
    }

    pub fn add_new_block(&mut self) -> bool {
        let mut block = Block::<T>::new(
            self.last_hash(), self.difficulty,
            self.miner_addr.clone(), self.reward, &mut self.curr_trans);


        Chain::<T>::proof_of_work(&mut block.header);
        println!("{}", &block.fmt());
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
            write!(&mut str, "{}", trans.fmt()).expect("[Chain fmt()]: Unable to write in Buffer!");
        }

        write!(&mut str, "    ]\n").expect("[Chain fmt()]: Unable to write in Buffer!");
        write!(&mut str, "    Difficulty:    {}\n", &self.difficulty).expect("[Chain fmt()]: Unable to write in Buffer!");
        write!(&mut str, "    Miner address: {}\n", &self.miner_addr).expect("[Chain fmt()]: Unable to write in Buffer!");
        write!(&mut str, "]\n").expect("[Chain fmt()]: Unable to write in Buffer!");

        str
    }
}

#[cfg(test)]
mod tests {
    use crate::blockchain::chain::Chain;
    use crate::blockchain::transaction::{CryptoPayload, Transactional};

    #[test]
    fn create_crypto_block_chain() {
        let miner_addr = String::from("Hans");
        let difficulty = 1;
        let chain = Chain::<CryptoPayload>::new(miner_addr, difficulty);

        assert_eq!(chain.chain.len(), 1);
        assert_eq!(chain.chain.get(0).unwrap().header.difficulty, 1);
        assert_eq!(chain.curr_trans.len(), 0);
        assert_eq!(chain.reward, 100);
    }

    #[test]
    fn add_crypto_transaction_to_block_chain() {
        let miner_addr = String::from("Hans");
        let difficulty = 1;
        let mut chain = Chain::<CryptoPayload>::new(miner_addr.clone(), difficulty);

        let crypto_payload = CryptoPayload {
            receiver: String::from("Peter"),
            amount: 42
        };
        let mut transaction = vec![CryptoPayload::new(miner_addr, crypto_payload)];


        chain.add_transaction(&mut transaction);

        assert_eq!(chain.chain.len(), 1);
        assert_eq!(chain.chain.get(0).unwrap().header.difficulty, 1);
        assert_eq!(chain.curr_trans.len(), 1);
        assert_eq!(chain.reward, 100);
    }
}
