#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;

#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serde;
extern crate serde_json;


mod blockchain;
mod web;

fn main() {
    let config = web::BlockchainState::new(); 
    rocket::ignite()
    .manage(config)
    .mount("/", routes![
    
        web::new_block, 
        web::new_transaction
        
    ]).launch();
}
