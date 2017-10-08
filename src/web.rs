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
pub fn new_block(state: State<BlockchainState>) -> Result<String, u32> {
    return blockchain_op(&state, |b|{
         
         return "all good".to_string();
    });
}

//todo: post
#[get("/transaction/new")]
pub fn new_transaction(state: State<BlockchainState>) -> Result<String, u32> {
    return blockchain_op(&state, |b| {
         let num = b.new_transaction("a".to_string(), "b".to_string(), 1);
         return "all good".to_string();
    })
}

fn blockchain_op<F>(state: &State<BlockchainState>, blockchain_op: F) -> Result<String, u32> 
    where F: Fn(&mut Blockchain) -> String {
    let guard = state.blockchain.write();
    if guard.is_ok() {        
        let mut blockchain = guard.unwrap();
        let result = blockchain_op(&mut blockchain);
        return Ok(result);        
    }
    Err(500)
}


