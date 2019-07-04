//! # Overview
//!
//! A locally stored blockchain written in Rust
//!
//! Following commands are available:
//!
//! - ```new transaction``` to create a new transaction and add it to the blockchain.
//! - ```mine block``` to mine a new block and add it to the blockchain.
//! - ```update difficulty``` to update the difficulty to mine a new block.
//! - ```update reward``` to update the reward a miner gets, when a new block is mined.
//! - ```print``` prints the blockchain.
//! - ```help``` prints the available commands.
//! - ```exit``` exits the programm.

/// The core datastructures to create a blockchain
mod blockchain;
mod crypto;

use std::io;
use std::io::{Write, BufRead};

use blockchain::chain::Chain;
use blockchain::transaction::{CryptoPayload, Transactional};
use std::process::exit;

fn main() {
    print!("Please input a miner address: ");
    let miner_addr = read_user_input();

    let mut difficulty: u32;

    loop {
        print!("Please enter the difficulty to mine a new block: ");
        difficulty = match read_user_input().parse() {
            Ok(difficulty) => difficulty,
            Err(_) => {
                0
            }
        };

        if difficulty == 0 {
            println!("Please enter only numbers greater than 0!");
        } else {
            break;
        }
    }

    println!("Generating genesis block...");
    let mut chain = Chain::<CryptoPayload>::new(
        miner_addr.trim().to_string(), difficulty);

    loop {
        print!("Please enter a command:");
        let command = read_user_input();

        match command.as_ref() {
            "new transaction" => new_transaction(&mut chain),
            "help" => print_help_text(),
            "print" => println!("{}", chain.fmt()),
            "mine block" => mine_block(&mut chain),
            "update difficulty" => change_difficulty(&mut chain),
            "update reward" => update_reward(&mut chain),
            "exit" => {
                println!("exiting!");
                exit(0);
            }
            _ => println!("Unknown command: {}! For help type \"help\".", command)
        }
    }
}

/// Prints the available commands.
fn print_help_text() {
    println!("\nFollowing commands are available:\n");
    println!("new transaction   --------------- Creates a new transaction.");
    println!("mine block        --------------- Mines a new block and adds it to the blockchain.");
    println!("update difficulty --------------- Changes the difficulty to mine a new block.");
    println!("update reward     --------------- Changes the reward a miner gets, when a new block is mined.");
    println!("print             --------------- Prints the blockchain.");
    println!("exit              --------------- Exits the programm.\n")
}

/// Reads the input from a user and returns it.
fn read_user_input() -> String {
    let mut user_input = String::new();

    io::stdout().flush().expect("IO Error");
    match io::stdin().lock().read_line(&mut user_input) {
        Ok(_) => user_input.trim().to_string(),
        Err(e) => {
            println!("{}", e);
            exit(-1);
        }
    }
}

/// Adds a new transaction to the blockchain.
fn new_transaction(chain: &mut Chain<CryptoPayload>) {
    print!("Please enter the sender address:");
    let sender = read_user_input();

    print!("Please enter the receiver address:");
    let receiver = read_user_input();

    print!("Please enter the amount to transfer: ");
    let amount = read_user_input();

    let crypto_payload = CryptoPayload {
        receiver: receiver.trim().to_string(),
        amount: amount.trim().parse().unwrap(),
    };
    let mut transaction = vec![CryptoPayload::new(sender, crypto_payload)];

    match chain.add_transaction(&mut transaction) {
        true => println!("Transaction was successfully added."),
        false => println!("Transaction failed!")
    }
}

/// Adds a new block to the blockchain
fn mine_block(chain: &mut Chain<CryptoPayload>) {
    println!("Generating block...");
    match chain.add_new_block() {
        true => println!("Block was generated successfully."),
        false => println!("Block generation failed!"),
    }
}

/// Changes the difficulty to mine a new block.
fn change_difficulty(chain: &mut Chain<CryptoPayload>) {
    let mut new_difficulty: u32;

    loop {
        print!("Please enter the new difficulty: ");
        new_difficulty = match read_user_input().parse() {
            Ok(difficulty) => difficulty,
            Err(_) => {
                0
            }
        };

        if new_difficulty == 0 {
            println!("Please enter only numbers greater than 0!");
        } else {
            break;
        }
    }

    match chain.update_difficulty(new_difficulty) {
        true => println!("Updated difficulty successfully"),
        false => println!("Failed to update difficulty!")
    }
}

/// Updates the reward a miner gets, when a new block is mined.
fn update_reward(chain: &mut Chain<CryptoPayload>) {
    let mut new_reward: u32;

    loop {
        print!("Please enter the new reward: ");
        new_reward = match read_user_input().parse() {
            Ok(difficulty) => difficulty,
            Err(_) => {
                0
            }
        };

        if new_reward == 0 {
            println!("Please enter only numbers greater than 0!");
        } else {
            break;
        }
    }

    match chain.update_reward(new_reward) {
        true => println!("Updated reward successfully."),
        false => println!("Failed to update reward!")
    }
}

