#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate log;
extern crate chrono;
//extern crate env_logger;
extern crate log4rs;
extern crate sha2;
extern crate base64;
extern crate url;
extern crate reqwest;
extern crate clap;
 
mod lib;
mod web;

use clap::{Arg, App};

///
/// Entry point. Starts logger, parses command line args and starts the web api
/// 
/// Note: The impl doesn't really make sense yet. Transactions can be added by anyone
///       and there is no communication between nodes (such as queued transactions),
///       only during consensus. It will be fleshed out in time.
/// 
fn main() {
    //env_logger::init().unwrap_or_else(|e| println!("Failed to init env_logger. {}", e));
    log4rs::init_file("log4rs.yml", Default::default()).unwrap_or_else(|e| println!("Failed to init log4rs. {}", e));
    debug!("Started");
    
    let args = parse_args();

    //The state wrapper that allows Rocket to access the underlying lib::Blockchain
    let blockchain_state = web::BlockchainState::new_with(args.difficulty); 

    //Start the API
    web::init(blockchain_state);
}

///
/// The supported command line arguments
/// 
struct Args {
    difficulty: u64
}

fn parse_args() -> Args {
    debug!("Parsing args...");

    let matches = App::new("learnnet blockchain")
                          .version("0.1")
                          .author("Steven P. <steven.pack.code@gmail.com>")
                          .about("Learn the blockchain! Inspired by https://github.com/dvf/blockchain")                         
                          .arg(Arg::with_name("difficulty")
                               .short("d")
                               .long("difficulty")
                               .help("Proof of work difficulty. 3 would mean a hash starting with 000")
                               .takes_value(true))                         
                          .get_matches();

    let difficulty: u64 = matches.value_of("difficulty").unwrap_or("3").parse().expect("difficulty must be valid integer");

    info!("using difficulty {}", difficulty);

    Args {
        difficulty: difficulty
    }
}