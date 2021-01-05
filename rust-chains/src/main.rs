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

    let mut block = Block::new(0, 0, vec![0; 32],0,"First Block".to_owned(), difficulty);
    
    block.mine();

    println!("{:?}",&block);
    
    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain{
        blocks:vec![block],
    };

    for i in 1..=10{
        let mut block = Block::new(i, 0, last_hash ,0,"Next Block".to_owned(), difficulty);
        block.mine();
        println!("{:?}",&block);
        
        last_hash = block.hash.clone();
        
        blockchain.blocks.push(block);
    }
}