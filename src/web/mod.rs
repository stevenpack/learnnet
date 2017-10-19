mod converters;
mod api;

use rocket;
use rocket::{Config, State};
use lib::blockchain::*;
use lib::transaction::*;
use std::sync::{RwLock};

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

//Requests
#[derive(Debug, Deserialize)]
pub struct NodeList {
    nodes: Vec<String>
}

pub fn init(rocket_config: Config, blockchain_state: BlockchainState) {
    rocket::custom(rocket_config, false)
    //rocket::ignite()
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
    blockchain_op(&state, |b| api::mine(b))
}

#[post("/transaction/new", format = "application/json", data = "<transaction>")]
pub fn new_transaction(transaction: Transaction, state: State<BlockchainState>) -> Result<String, u32> {
    blockchain_op(&state, |b| api::new_transaction(&transaction, b))
}

#[get("/chain", format = "application/json")]
pub fn chain(state: State<BlockchainState>) -> Result<String, u32> {
    blockchain_op(&state, |b| api::chain(b))
}

#[post("/nodes/register", format = "application/json", data="<node_list>")]
pub fn register_node(node_list: NodeList, state: State<BlockchainState>) -> Result<String, u32> {
    blockchain_op(&state, |b| api::register_node(&node_list, b))
}

#[get("/nodes/resolve")]
pub fn consensus(state: State<BlockchainState>) -> Result<String, u32> {
    blockchain_op(&state, |b| api::consensus(b))
}

///
/// Retrieves the blockchain from state, unlocks and executes the closure
/// 
fn blockchain_op<F>(state: &State<BlockchainState>, blockchain_op: F) -> Result<String, u32> 
    where F: Fn(&mut Blockchain) -> Result<String, u32> {
    
    let guard = state.blockchain.write();
    if guard.is_ok() {        
        let mut blockchain = guard.unwrap();
        let result = blockchain_op(&mut blockchain);
        return result;
    }
    error!("Couldn't acquire lock");
    Err(500)
}