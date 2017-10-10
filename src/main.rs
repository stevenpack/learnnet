#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;

#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use] extern crate log;
extern crate env_logger;
extern crate sha2;
extern crate base64;

mod lib;
mod web;

fn main() {
    env_logger::init().unwrap_or_else(|e| println!("Failed to init env_logger. {}", e));
    debug!("Started");

    let config = web::BlockchainState::new(); 
    rocket::ignite()
    .manage(config)
    .mount("/", routes![
    
        web::mine, 
        web::new_transaction,
        web::chain
        
    ]).launch();
}
