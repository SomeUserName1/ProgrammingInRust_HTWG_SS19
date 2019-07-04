/// Start a node and provide a cli to it
///
/// TODO unify with all other modules
mod blockchain;
mod crypto;

use std::io;
use std::process;
use std::io::Write;

use blockchain::chain;
use blockchain::transaction::{CryptoPayload, Transactional};


fn main() {

    // TODO start node from here

    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("input a miner address: ");
    io::stdout().flush().expect("IO Error");
    io::stdin().read_line(&mut miner_addr).expect("IO Error");
    print!("Difficulty: ");
    io::stdout().flush().expect("IO Error");
    io::stdin().read_line(&mut difficulty).expect("IO Error");
    let diff = difficulty.trim().parse::<u32>().expect("we need an integer");
    println!("generating genesis block! ");
    let mut chain = chain::Chain::<CryptoPayload>::new( diff, "root".to_string());

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("5) Print Blockchain");
        println!("0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush().expect("IO Error");
        choice.clear();
        io::stdin().read_line(&mut choice).expect("IO Error");
        println!();

        match choice.trim().parse().unwrap() {
            0 =>
                {
                    println!("exiting!");
                    process::exit(0);
                },
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("enter sender address:");
                io::stdout().flush().expect("IO Error");
                io::stdin().read_line(&mut sender).expect("IO Error");
                print!("enter receiver address: ");
                io::stdout().flush().expect("IO Error");
                io::stdin().read_line(&mut receiver).expect("IO Error");
                print!("Enter amount: ");
                io::stdout().flush().expect("IO Error");
                io::stdin().read_line(&mut amount).expect("IO Error");

                let res = chain.add_transaction(&mut vec![
                    CryptoPayload::new(
                        sender.trim().to_string(),
                        CryptoPayload {
                            receiver: receiver.trim().to_string(),
                            amount: amount.trim().parse().unwrap(),
                        })
                ]);

                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed"),
                }
            },
            2 =>
                {
                    let mut sender = String::new();
                    print!("enter sender address:");
                    io::stdout().flush().expect("IO Error");
                    io::stdin().read_line(&mut sender).expect("IO Error");
                    println!("Generating block");
                    let res = chain.add_new_block(sender);
                    match res {
                        true => println!("Block generated successfully"),
                        false => println!("Block generation failed"),
                    }
                },
            3 =>
                {
                    let mut new_diff = String::new();
                    print!("enter new difficulty: ");
                    io::stdout().flush().expect("IO Error");
                    io::stdin().read_line(&mut new_diff).expect("IO Error");
                    let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                    match res {
                        true => println!("Updated Difficulty"),
                        false => println!("Failed Update Difficulty"),
                    }
                },
            4 =>{
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush().expect("IO Error");
                io::stdin().read_line(&mut new_reward).expect("IO Error");
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated reward"),
                    false => println!("Failed Update reward"),
                }
            }
            5 => {
                println!("{}", chain.fmt());
            }
            _ => println!("Invalid option please retry"),
        }

    }
}
