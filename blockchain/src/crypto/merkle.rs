use super::hash::hash;
use crate::blockchain::transaction::Transaction;

pub fn get_merkle<T: serde::Serialize + std::fmt::Debug + std::clone::Clone>(curr_trans: Vec<Transaction<T>>) -> String {
    let mut merkle = Vec::new();

    for t in &curr_trans {
        let hash = hash(t);
        merkle.push(hash);
    }

    if merkle.len() % 2 == 1 {
        let last = merkle.last().cloned().unwrap();
        merkle.push(last);
    }

    while merkle.len() > 1 {
        let mut h1 = merkle.remove(0);
        let mut h2 = merkle.remove(0);
        h1.push_str(&mut h2);
        let nh = hash(&h1);
        merkle.push(nh);
    }
    match merkle.pop() {
        Some(a) => a,
        None => {
            println!("Got no merkle as I commented out the reward block123");
            panic!("duh");
        }
    }
}
