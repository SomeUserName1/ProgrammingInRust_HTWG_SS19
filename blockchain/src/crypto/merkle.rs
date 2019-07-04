//! The merkle tree of a block.
//!
//! A Merkle tree summarizes all the transactions in a block by producing a digital fingerprint
//! of the entire set of transactions, thereby enabling a user to verify whether or not a
//! transaction is included in a block.
//! Every transaction will be hashed. Each hash will be again be hashed with another hash,
//! halving the amount of hashes. This continues until only one hash is left. Because of this
//! we need a even amount of hashes to start with.

use super::hash::hash;
use crate::blockchain::transaction::Transaction;


/// Produces the merkle tree as a string.
pub fn get_merkle<T: serde::Serialize + std::fmt::Debug + std::clone::Clone>(curr_trans: Vec<Transaction<T>>) -> String {
    let mut merkle = Vec::new();

    // hash every item
    for t in &curr_trans {
        let hash = hash(t);
        merkle.push(hash);
    }

    // we need an even amount to halve the amount until one is left
    if merkle.len() % 2 == 1 {
        let last = merkle.last().cloned().unwrap();
        merkle.push(last);
    }

    // take two and hash them together
    while merkle.len() > 1 {
        let mut h1 = merkle.remove(0);
        let mut h2 = merkle.remove(0);
        h1.push_str(&mut h2);
        let nh = hash(&h1);
        merkle.push(nh);
    }
    // return the one remaining
    match merkle.pop() {
        Some(a) => a,
        None => {
            println!("Got no merkle as I commented out the reward block123");
            panic!("duh");
        }
    }
}
