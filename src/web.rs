extern crate rocket;

use serde_json;
use rocket::State;
use blockchain::{Blockchain, Transaction, Block};
use std::sync::{RwLock};
use std::io::Read;
use rocket::{Request, Data, Outcome};
use rocket::data::{self, FromData};
use rocket::http::{Status, ContentType};
use rocket::Outcome::*;

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
         
         return "all good".into();
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

impl FromData for Transaction {
    type Error = String;

    fn from_data(_: &Request, data: Data) -> data::Outcome<Self, String> {
        
        let transaction: Transaction = match serde_json::from_reader(data.open()) {
            Ok(transaction) => transaction,
            Err(e) => {
                error!("Failed to deserialize transaction {:?}", e);
                return Failure((Status::BadRequest, format!("Couldn't parse transaction")));
            }
        };
        debug!("Successfully parsed transaction. {:?}", transaction);
        Success(transaction)
    }
}

///
/// Retrieves the blockchain from state, unlocks and executions the closure
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


