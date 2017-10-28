mod converters;
mod api;
mod types;

use std::fmt::{Display};
use rocket;
use rocket::{State};
use rocket::response::content;
use lib::blockchain::*;
use lib::transaction::*;
use std::sync::{RwLock};
use web::types::*;
use serde_json;
use serde::Serialize;

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

///
/// Typical return type of serialized JSON, or an http error code
/// 
type JsonResult = Result<content::Json<String>, u32>;

///
/// Routes. Responsible for getting read/write lock on `BlockchainState`, then
/// converting to a `JsonResult`
/// 

#[get("/mine")]
pub fn mine(state: State<BlockchainState>) -> JsonResult {
     match state.blockchain.write() {
        Ok(mut blockchain) => match api::mine(&mut blockchain) {
            Ok(result) => to_json_result(result),
            Err(_) => Err(500)
        }
        Err(e) => no_read_lock(e)
    }   
}

#[post("/transaction/new", format = "application/json", data = "<transaction>")]
pub fn new_transaction(transaction: Transaction, state: State<BlockchainState>) -> JsonResult {
     match state.blockchain.write() {
        Ok(mut blockchain) => to_json_result(api::new_transaction(&transaction, &mut blockchain)),
        Err(e) => no_read_lock(e)
    }   
}

#[get("/chain")]
pub fn chain(state: State<BlockchainState>) -> JsonResult {
     match state.blockchain.read() {
        Ok(blockchain) => to_json_result(api::chain(&blockchain)),
        Err(e) => no_read_lock(e)
    }   
}

#[post("/nodes/register", format = "application/json", data="<node_list>")]
pub fn register_node(node_list: NodeList, state: State<BlockchainState>) -> JsonResult {
    match state.blockchain.write() {
        Ok(mut blockchain) => {
            match api::register_node(&node_list, &mut blockchain) {
                Ok(response) => to_json_result(response),
                Err(_) => Err(400)
            }
        },
        Err(e) => no_write_lock(e)
    }    
}

#[get("/nodes/resolve")]
pub fn consensus(state: State<BlockchainState>) -> JsonResult  {
    match state.blockchain.write() {
        Ok(mut blockchain) => to_json_result(api::consensus(&mut blockchain)),
        Err(e) => no_write_lock(e)
    }    
}

fn no_read_lock<T, E>(err: E) -> Result<T, u32> where E : Display {
    error!("Failed to get READ lock {}", err);
    Err(500)
}

fn no_write_lock<T, E>(err: E) -> Result<T, u32> where E : Display {
    error!("Failed to get WRITE lock {}", err);
    Err(500)
}

///
/// Given a response, serialize to a Json string, or return 500 if it fails
/// 
fn to_json_result<T>(response: T) -> JsonResult 
    where T: Serialize {
    match serde_json::to_string(&response) {
        Ok(serialized) => Ok(content::Json(serialized)),
        Err(e) => {
            error!("Failed to serialize {}", e);
            Err(500)
        }
    }
}