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

use clap::{Arg, App};
use rocket::config::{Config, Environment};

fn main() {
    env_logger::init().unwrap_or_else(|e| println!("Failed to init env_logger. {}", e));
    debug!("Started");
    
    let args = parse_args();

    //The state wrapper that allows Rocket to access the underlying lib::Blockchain
    let blockchain_state = web::BlockchainState::new_with(args.difficulty); 

    //The web server setup
    let rocket_config = Config::build(Environment::Production)
        .address("localhost")
        .port(args.port)
        .expect("Rocket config");

    //Start the API
    web::init(rocket_config, blockchain_state);
}

struct Args {
    port: u16,
    difficulty: u64
}

fn parse_args() -> Args {
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
                               .help("Proof of work difficulty. 3 would mean a hash starting with 000")
                               .takes_value(true))                         
                          .get_matches();

    let port: u16 = matches.value_of("port").unwrap_or("8000").parse().expect("port must be valid integer");
    let difficulty: u64 = matches.value_of("difficulty").unwrap_or("3").parse().expect("difficulty must be valid integer");

    info!("using port {}", port);
    info!("using difficulty {}", difficulty);

    Args {
        port: port,
        difficulty: difficulty
    }
}