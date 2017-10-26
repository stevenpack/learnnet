mod converters;
mod api;
mod types;

use rocket;
use rocket::{State};
use lib::blockchain::*;
use lib::transaction::*;
use std::sync::{RwLock};
use web::types::*;
///
/// A RwLock around `Blockchain`
/// 
/// It's passed to API methods exposed by Rocket and unlocked for reading or
/// writing as required
/// 
pub struct BlockchainState {
    pub blockchain: RwLock<Blockchain>
}

impl BlockchainState {
    pub fn new_with(difficulty: u64) -> BlockchainState {
        BlockchainState {
            blockchain: RwLock::new(Blockchain::new_with(difficulty))
        }
    }
}

///
/// Start Rocket
/// 
pub fn init(blockchain_state: BlockchainState) {
    rocket::ignite()
        .manage(blockchain_state)
        .mount("/", routes![
    
            mine, 
            new_transaction,
            chain,
            register_node,
            consensus 
            
        ]).launch();
}

//todo: respone as JSON - https://github.com/SergioBenitez/Rocket/blob/v0.3.3/examples/json/src/main.rs
#[get("/mine", format = "application/json")]
pub fn mine(state: State<BlockchainState>) -> Result<String, u32> {
    blockchain_mut_op(&state, |b| api::mine(b))
}

#[post("/transaction/new", format = "application/json", data = "<transaction>")]
pub fn new_transaction(transaction: Transaction, state: State<BlockchainState>) -> Result<String, u32> {
    blockchain_mut_op(&state, |b| api::new_transaction(&transaction, b))
}

#[get("/chain", format = "application/json")]
pub fn chain(state: State<BlockchainState>) -> Result<String, u32> {
    blockchain_op(&state, |b| api::chain(b))
}

#[post("/nodes/register", format = "application/json", data="<node_list>")]
pub fn register_node(node_list: NodeList, state: State<BlockchainState>) -> Result<String, u32> {
    blockchain_mut_op(&state, |b| api::register_node(&node_list, b))
}

#[get("/nodes/resolve")]
pub fn consensus(state: State<BlockchainState>) -> Result<String, u32> {
    blockchain_mut_op(&state, |b| api::consensus(b))
}

///
/// Retrieves the blockchain from state, unlocks for WRITE and executes the closure
/// 
fn blockchain_mut_op<F>(state: &State<BlockchainState>, blockchain_op: F) -> Result<String, u32> 
    where F: Fn(&mut Blockchain) -> Result<String, u32> {
    
    let guard = state.blockchain.write();
    if guard.is_ok() {        
        let mut blockchain = guard.unwrap();
        let result = blockchain_op(&mut blockchain);
        return result;
    }
    error!("Couldn't acquire WRITE lock");
    Err(500)
}

///
/// Retrieves the blockchain from state, unlocks for READ and executes the closure
/// 
fn blockchain_op<F>(state: &State<BlockchainState>, blockchain_op: F) -> Result<String, u32> 
    where F: Fn(&Blockchain) -> Result<String, u32> {
    
    let guard = state.blockchain.read();
    if guard.is_ok() {        
        let blockchain = guard.unwrap();
        let result = blockchain_op(&blockchain);
        return result;
    }
    error!("Couldn't acquire READ lock");
    Err(500)
}