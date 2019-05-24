use std::sync::{Arc, mpsc};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use easy_hash::{Sha256, HashResult};
use easy_hash::Hasher;
use std::sync::mpsc::RecvError;

// blog.softwaremill.com/multithreading-in-rust-with-mpsc-multi-producer-single-consumer-channels-db0fc91ae3fa

const BASE: usize = 42;
const NTHREADS: usize = 4;
#[allow(dead_code)]
const DIFFICULTY0: &str = "000";
#[allow(dead_code)]
const DIFFICULTY1: &str = "0000";
#[derive(Debug)]
struct Solution(usize, String);


fn main() {
    match async_search(DIFFICULTY1) {
        Ok(Solution(i, hash)) => println!("{}, {}", i, hash),
        Err(e) => println!("An error occured: {}", e),
    }
}
}


fn async_search(difficulty: &'static str) -> Result<Solution, RecvError> {
    unimplemented!()
}

fn search_for_solution(start_at: usize, sender: mpsc::Sender<Solution>, is_solution_found: Arc<AtomicBool>, difficulty: &str) {
    let mut iteration_no = 0;
    for number in (start_at..).step_by(NTHREADS) {
        if let Some(solution) = verify_number(number, difficulty) {
            is_solution_found.store(true, Ordering::Relaxed);
            match sender.send(solution) {
                Ok(_)  => {},
                Err(_) => println!("Receiver has stopped listening, dropping worker number {}.", start_at),
            }
            return;
        } else if iteration_no % 1000 == 0 && is_solution_found.load(Ordering::Relaxed) {
            return;
        }
        iteration_no += 1;
    }
}

fn verify_number(number: usize, difficulty: &str) -> Option<Solution> {
    let hash: String = Sha256::hash((number * BASE).to_string().as_bytes()).hex();
    if hash.ends_with(difficulty) {
        Some(Solution(number, hash))
    } else {
        None
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_search0() {
        assert_eq!(async_search(DIFFICULTY0), Ok(Solution(1574, "bde6977bd9de18c8c87d6da84ef49165be37f904cf9803d8dea237fe5d98d000".to_string())));
    }

    #[test]
    fn test_async_search1() {
        assert_eq!(async_search(DIFFICULTY1), Ok(Solution(3341, "ace3f7f0672635a4b3be1a036c2506d899ade343d6b799a22598c0a088d70000".to_string())));
    }
}
