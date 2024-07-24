use std::io::{self, Write};
use std::process;

mod blockchain;

fn main() {
    let miner_addr = prompt("input a miner address: ");
    let difficulty = prompt("Difficulty: ")
        .trim()
        .parse::<u32>()
        .expect("we need an integer");
    println!("generating genesis block! ");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), difficulty);

    loop {
        println!(
            "Menu\n\
            1) New Transaction\n\
            2) Mine block\n\
            3) Change Difficulty\n\
            4) Change Reward\n\
            0) Exit"
        );
        let choice = prompt("Enter your choice: ")
            .trim()
            .parse()
            .unwrap_or_else(|_| {
                println!("Invalid choice, please enter a number.");
                99 // Invalid choice placeholder
            });

        match choice {
            0 => {
                println!("exiting!");
                process::exit(0);
            }
            1 => {
                let sender = prompt("enter sender address: ");
                let receiver = prompt("enter receiver address: ");
                let amount: f32 = prompt("Enter amount: ")
                    .trim()
                    .parse()
                    .expect("we need a valid number");
                if chain.new_transaction(sender.trim().to_string(), receiver.trim().to_string(), amount) {
                    println!("transaction added");
                } else {
                    println!("transaction failed");
                }
            }
            2 => {
                println!("Generating block");
                if chain.generate_new_block() {
                    println!("Block generated successfully");
                } else {
                    println!("Block generation failed");
                }
            }
            3 => {
                let new_diff: u32 = prompt("enter new difficulty: ")
                    .trim()
                    .parse()
                    .expect("we need a valid integer");
                if chain.update_difficulty(new_diff) {
                    println!("Updated Difficulty");
                } else {
                    println!("Failed to update Difficulty");
                }
            }
            4 => {
                let new_reward: f32 = prompt("Enter new reward: ")
                    .trim()
                    .parse()
                    .expect("we need a valid number");
                if chain.update_reward(new_reward) {
                    println!("Updated reward");
                } else {
                    println!("Failed to update reward");
                }
            }
            _ => println!("Invalid option please retry"),
        }
    }
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().expect("failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    input
}
