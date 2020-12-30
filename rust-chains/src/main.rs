#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod lib;

#[get("/")]
fn external() -> &'static str {
    "Hello, world!"
}

fn main() {

    rocket::ignite().mount("/", routes![external]).launch();
}