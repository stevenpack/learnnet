extern crate rocket;

use rocket::State;
use lib::blockchain::*;
use lib::transaction::*;
use std::sync::{RwLock};

mod converters;

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
#[get("/mine")]
pub fn mine(state: State<BlockchainState>) -> Result<String, u32> {
    return blockchain_op(&state, |b| {

        let mined_block = b.mine();

        return format!("Mined new block with proof {}", mined_block.proof);
    //      response = {
    //     'message': "New Block Forged",
    //     'index': block['index'],
    //     'transactions': block['transactions'],
    //     'proof': block['proof'],
    //     'previous_hash': block['previous_hash'],
    // }
    });
}

//todo: post
#[post("/transaction/new", format = "application/json", data = "<transaction>")]
pub fn new_transaction(transaction: Transaction, state: State<BlockchainState>) -> Result<String, u32> {
    blockchain_op(&state, |b| {
        let index = b.new_transaction(transaction.clone());
        return format!("Transaction added at block {}", index);
    })
}



///
/// Retrieves the blockchain from state, unlocks and executions the closure
/// 
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


