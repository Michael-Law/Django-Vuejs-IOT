#[derive(Debug)]
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
    nonce: &u64,
            ) ->String {
                let mut bytes = vec![];
                use std::io::Read
                bytes.extend(&timestamp.to_ne_bytes());
                bytes.extend(
                    transactions
                        .iter()
                        .flat_map(|transaction| transaction.bytes())
                        .collect::<Vec<u8>>(),
                );
                bytes.extend(pre_hash.as_bytes());
        
        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
    }
