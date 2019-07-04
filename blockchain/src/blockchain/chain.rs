//! Data structure to maintain the chain.

////////////////////////////////////////////////////////////////////////////////

use std::fmt::{Debug, Write};
use std::clone::Clone;

use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::crypto::hash;

use super::block::{Block, BlockHeader};
use super::transaction::{Transaction, Transactional};
use std::sync::mpsc::RecvError;
use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

const NTHREADS: usize = 4;

#[derive(Debug)]
struct Solution(usize, String);

/// The Chain struct contains the actual chain, consisting of a vector of Blocks.
/// It also hold the current transactions which are not yet contained in a Block.
/// Using proof of work as consensus mechanism, the struct also hold the difficulty
/// and the reward for mining new blocks.
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
    /// Create a new chain. Difficulty can be chosen by the creator.
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

    /// Setter for adding a new transaction to the chain.
    pub fn add_transaction(&mut self, transactions: &mut Vec<Transaction<T>>) ->
    bool {
        self.curr_trans.append(transactions);
        true
    }

    /// Getter for the hash of the last block. If none is present, it will return a specific hash.
    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap()
        };
        hash::hash(&block.header)
    }

    /// Setter for the difficulty for finding a new block.
    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    /// Setter for the reward for finding a new block.
    pub fn update_reward(&mut self, reward: u32) -> bool {
        self.reward = reward;
        true
    }

    /// Creates a new block. All necessary information is taken from the chain. That is the previous
    /// blocks hash, difficulty, the miners address, the reward and all current transactions.
    /// Uses proof of work to mine, so depending on the difficulty it will take a while.
    pub fn add_new_block(&mut self) -> bool {
        let mut block = Block::<T>::new(
            self.last_hash(), self.difficulty,
            self.miner_addr.clone(), self.reward, &mut self.curr_trans);


        Chain::<T>::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    /// Primary function for our proof of work consensus mechanism. Wraps around `async_search`
    /// and waits for a solution.
    pub fn proof_of_work(header: &mut BlockHeader) -> Result<(), RecvError> {
        let difficulty = header.difficulty;
        match async_search(header.clone()) {
            Ok(sol) => {
                header.nonce = sol.0 as u32;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Formatter for the chain. Will print out every field of the Chain.
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

/// Kicks off the asynchronous search by cloning the header and dividing the space to search though.
/// For each division, it spawns a thread and waits for a worker to return.
fn async_search(header: BlockHeader) -> Result<Solution, RecvError> {
    let found = Arc::new(AtomicBool::new(false));
    let (sender, receiver) = mpsc::channel();

    for i in 0..NTHREADS {
        let found = found.clone();
        let sender_n = sender.clone();
        let header_clone = header.clone();
        thread::spawn(move || {
            search_for_solution(i, sender_n, found, header_clone);
        });
    }

    match receiver.recv() {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    }
}

/// A search worker. When mining, there are as many workers as defined in `NTHREADS`. For not searching
/// though the same space, each worker will skip the next `NTHREADS` numbers. So besides some overhead,
/// this will be n times faster than using a single thread.
fn search_for_solution(start_at: usize, sender: mpsc::Sender<Solution>, is_solution_found: Arc<AtomicBool>, header: BlockHeader) {
    let mut iteration_no = 0;
    for number in (start_at..).step_by(NTHREADS) {
        if let Some(solution) = verify_number(number, header.clone()) {
            is_solution_found.store(true, Ordering::Relaxed);
            match sender.send(solution) {
                Ok(_) => {}
                Err(_) => println!("Receiver has stopped listening, dropping worker number {}.", start_at),
            }
            return;
        } else if iteration_no % 1000 == 0 && is_solution_found.load(Ordering::Relaxed) {
            return;
        }
        iteration_no += 1;
    }
}

/// Hashes the given header and checks whether the result ends with defined difficulty or not.
fn verify_number(number: usize, header: BlockHeader) -> Option<Solution> {
    let hash: String = hash::hash(&header);
    if hash.ends_with(header.difficulty.to_string().as_str()) {
        Some(Solution(number, hash))
    } else {
        None
    }
}
