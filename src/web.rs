#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;

use rocket::State;
use blockchain::Blockchain;
use std::sync::{RwLock};

pub struct BlockchainState {
    pub blockchain: RwLock<Blockchain>
}

impl BlockchainState {
    pub fn new() -> BlockchainState {
        BlockchainState {
            blockchain: RwLock::new(Blockchain::new())
        }
    }
}

//todo: post
#[get("/new-block")]
pub fn new_block(state: State<BlockchainState>) -> String {
    //let mut blockchain = state.blockchain.write().expect("Unable to get write lock");
    let guard = state.blockchain.write();
    if guard.is_ok() {
        return guard.unwrap().new_block()
    }
    "nope".to_string()
}

//todo: post
#[get("/new-transaction")]
pub fn new_transaction(state: State<BlockchainState>) -> String {
    state.blockchain.write().unwrap().new_transaction()
}
