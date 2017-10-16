#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate chrono;
extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate sha2;
extern crate base64;
extern crate url;
extern crate reqwest;
extern crate clap;
 
mod lib;
mod web;

use clap::{Arg, App, SubCommand};


fn main() {
    env_logger::init().unwrap_or_else(|e| println!("Failed to init env_logger. {}", e));
    debug!("Started");
    debug!("Parsing args...");

    let matches = App::new("learnnet blockchain")
                          .version("0.1")
                          .author("Steven P. <steven.pack.code@gmail.com>")
                          .about("Learn the blockchain! Inspired by https://github.com/dvf/blockchain")
                          .arg(Arg::with_name("port")
                               .short("p")
                               .long("port")
                               .help("Web server port")
                               .takes_value(true))
                          .arg(Arg::with_name("difficulty")
                               .short("d")
                               .long("difficulty")
                               .help("Proof of work difficult. 3 would mean a hash starting with 000")
                               .takes_value(true))                         
                          .get_matches();

    let port: u32 = matches.value_of("port").unwrap_or("8000").parse().expect("port must be valid integer");
    let difficulty: u32 = matches.value_of("difficulty").unwrap_or("3").parse().expect("difficulty must be valid integer");
    info!("using port {}", port);
    info!("using difficulty {}", difficulty);

    let config = web::BlockchainState::new(); 
    rocket::ignite()
    .manage(config)
    .mount("/", routes![
    
        web::mine, 
        web::new_transaction,
        web::chain,
        web::register_node,
        web::consensus 
        
    ]).launch();
}
