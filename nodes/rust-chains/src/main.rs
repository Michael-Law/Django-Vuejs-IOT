use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use blockchainlib::*;
use serde::Deserialize;
use std::collections::HashSet;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CHAIN: Mutex<blockchainlib::Blockchain> = Mutex::new(blockchainlib::Blockchain {
        blocks: vec![],
        unspent_outputs: HashSet::new()
    });
}

lazy_static! {
    static ref TRUNCKHASH: Mutex<Vec<Vec<u8>>> = Mutex::new(vec![]);
}

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

    if unsafe { DEGREE } == 0 {
        let mut block = Block::new(
            unsafe { DEGREE },
            now(),
            vec![0; 32],
            vec![Transaction {
                inputs: vec![],
                outputs: vec![
                    transaction::Output {
                        to_addr: req_body.originator.to_owned(),
                        value: req_body.value,
                    },
                    transaction::Output {
                        to_addr: req_body.destinator.to_owned(),
                        value: req_body.value,
                    },
                ],
            }],
            difficulty,
        );

        block.mine();

        let prev_hash = block.hash.clone();

        TRUNCKHASH.lock().unwrap().push(prev_hash);
        CHAIN
            .lock()
            .unwrap()
            .update_with_block(block)
            .expect("Failed to add block");

        unsafe { increase_degree() };
    } else {
        println!("Index is {:?}", unsafe { DEGREE });
        let indexer: u32 = unsafe { DEGREE } - 1;
        let last_hash = TRUNCKHASH.lock().unwrap()[indexer as usize].clone();
        let mut chains = CHAIN.lock().unwrap();
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
                    inputs: vec![chains.blocks[indexer as usize].transactions[0].outputs[0].clone()],
                    outputs: vec![
                        transaction::Output {
                            to_addr: req_body.originator.to_owned(),
                            value: chains.blocks[0].transactions[0].outputs[0].value
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

        let prev_hash = block.hash.clone();

        TRUNCKHASH.lock().unwrap().push(prev_hash);

        chains
            .update_with_block(block)
            .expect("Failed to add block");
        unsafe { increase_degree() };
    }
    HttpResponse::Ok().body("Transaction Complete")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(peertopeer))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
