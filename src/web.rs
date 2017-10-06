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
    //let mut blockchain = state.blockchain.write().expect("Unable to get write lock");    
    // let guard = state.blockchain.write();
    // if guard.is_ok() {
    //     return Ok(guard.unwrap().new_block())
    // }
    return x(&state,  &mut |b| b.new_block() );
}

fn x<F>(state: &State<BlockchainState>, blockchain_op: &mut F) -> Result<String, u32> 
    where F: Fn(&mut Blockchain) -> String {
    let guard = state.blockchain.write();
    if guard.is_ok() {        
        let mut blockchain = guard.unwrap();
        let result = blockchain_op(&mut blockchain);
        return Ok(result);        
    }
    Err(500)
}

//todo: post
#[get("/new-transaction")]
pub fn new_transaction(state: State<BlockchainState>) -> String {
    state.blockchain.write().unwrap().new_transaction()
}
