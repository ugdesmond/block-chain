use serde_derive::Serialize;
use sha2::{Digest, Sha256};
use chrono::prelude::*;

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Serialize, Debug)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Serialize, Debug)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

pub struct Chain {
    chain: Vec<Block>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Self {
        let mut chain = Self {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };

        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.curr_trans.push(Transaction { sender, receiver, amount });
        true
    }

    pub fn last_hash(&self) -> String {
        self.chain
            .last()
            .map_or_else(|| "0".repeat(64), |block| Self::hash(&block.header))
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: Utc::now().timestamp_millis(),
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        let reward_trans = Transaction {
            sender: "Root".to_string(),
            receiver: self.miner_addr.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![reward_trans],
        };

        block.transactions.append(&mut self.curr_trans);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Self::get_merkle(&block.transactions);
        Self::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.chain.push(block);
       
        true
    }

    fn get_merkle(transactions: &[Transaction]) -> String {
        let mut merkle: Vec<String> = transactions.iter().map(Self::hash).collect();

        if merkle.len() % 2 == 1 {
            merkle.push(merkle.last().cloned().unwrap());
        }

        while merkle.len() > 1 {
            let mut new_merkle = Vec::with_capacity(merkle.len() / 2);
            for chunk in merkle.chunks(2) {
                let mut combined = chunk[0].clone();
                combined.push_str(&chunk[1]);
                new_merkle.push(Self::hash(&combined));
            }
            merkle = new_merkle;
        }

        merkle.pop().unwrap_or_default()
    }

    pub fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Self::hash(header);
            if hash.starts_with(&"0".repeat(header.difficulty as usize)) {
                println!("Block hash: {}", hash);
                break;
            }
            header.nonce += 1;
        }
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(item).expect("Serialization failed");
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        Self::hex_to_string(&result)
    }

    pub fn hex_to_string(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}
