use serde::{Deserialize, Serialize};


#[derive(Debug,Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f32,
}

pub struct Block {
    pub timestamp: i64,
    pub hash: String,
    pub pre_hash: String,
    pub transaction: Vec<Transaction>,
}

pub fn hash_suffix(
    pre_hash: &str,
    transactions: &Vec<Transaction>,
    timestamp: &u64,
            ) ->String {
                let mut bytes = vec![];
                bytes.extend(&timestamp.to_ne_bytes());
                bytes.extend(
                    bincode::serialize(&transactions).unwrap()
                );
                bytes.extend(pre_hash.as_bytes());
        
        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
    }
