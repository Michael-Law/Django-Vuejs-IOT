#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

use blockchainlib::*;

fn main() {
    let difficulty = 0x00ffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(
        0,
        0,
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 500,
                },
                transaction::Output {
                    to_addr: "Merle".to_owned(),
                    value: 700,
                },
            ],
        }],
        difficulty,
    );
    genesis_block.mine();

    println!("{:?}", &genesis_block);
    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block");

    let mut block = Block::new(
        1,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 536,
                }],
            },
            Transaction {
                inputs: vec![blockchain.blocks[0].transactions[0].outputs[0].clone()],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 36,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 12,
                    },
                ],
            },
        ],
        difficulty,
    );

    block.mine();

    println!("Mined block {:?}", &block);

    last_hash = block.hash.clone();

    blockchain
        .update_with_block(block)
        .expect("Failed to add block");

    rocket::ignite().mount("/", routes![hello]).launch();
}
