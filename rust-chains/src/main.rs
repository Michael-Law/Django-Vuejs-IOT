// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use] extern crate rocket;

// mod lib;

// #[get("/")]
// fn external() -> &'static str {
//     "Hello, world!"
// }

// fn main() {

//     rocket::ignite().mount("/", routes![external]).launch();
// }
use blockchainlib::*;

fn main(){
    let difficulty = 0x00ffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(0, 0, vec![0; 32],vec![
        Transaction{
            inputs:vec![],
            outputs:vec![
                transaction::Output{
                    to_addr:"Alice".to_owned(),
                    value:50,
                },

                transaction::Output{
                    to_addr:"Merle".to_owned(),
                    value:7,
                },
            ]
        }
    ], difficulty);
    
    genesis_block.mine();

    println!("{:?}",&genesis_block);
    
    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");

    
    

}