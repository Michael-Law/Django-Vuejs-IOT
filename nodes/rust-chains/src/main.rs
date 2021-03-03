use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use blockchainlib::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;

static mut DEGREE: u32 = 0;

unsafe fn increase_degree() -> u32 {
    let re = DEGREE;
    DEGREE += 1;
    return re;
}

#[derive(Deserialize, Debug)]
struct Request {
    originator: String,
    destinator: String,
    value: u64,
}

#[post("/transaction")]
async fn peertopeer(req_body: web::Form<Request>) -> impl Responder {
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Precursor".to_owned(),
                    value: 100,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                },
            ],
        }],
        difficulty,
    );

    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block");

    unsafe { increase_degree() };
    let indexer: u32 = unsafe { DEGREE } - 1;

    let mut block = Block::new(
        unsafe { DEGREE },
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![transaction::Output {
                    to_addr: "Miner".to_owned(),
                    value: 5,
                }],
            },
            Transaction {
                inputs: vec![
                    blockchain.blocks[indexer as usize].transactions[0].outputs[0].clone(),
                ],
                outputs: vec![
                    transaction::Output {
                        to_addr: req_body.originator.to_owned(),
                        value: blockchain.blocks[0].transactions[0].outputs[0].value
                            - req_body.value,
                    },
                    transaction::Output {
                        to_addr: req_body.destinator.to_owned(),
                        value: req_body.value,
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

    HttpResponse::Ok().body("Transaction Complete")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(peertopeer))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
