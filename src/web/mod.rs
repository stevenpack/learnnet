mod converters;
mod api;
mod types;

use rocket;
use rocket::{State};
use rocket::response::content;
use rocket_contrib::Json;
use lib::blockchain::*;
use lib::transaction::*;
use std::sync::{RwLock};
use web::types::*;
use serde_json;

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
            // chain,
            register_node,
            consensus 
            
        ]).launch();
}

#[get("/mine")]
pub fn mine(state: State<BlockchainState>) -> Result<Json<MineResult>, u32> {
    to_json(blockchain_mut_op(&state, |b| api::mine(b)))
}

#[post("/transaction/new", format = "application/json", data = "<transaction>")]
pub fn new_transaction(transaction: Transaction, state: State<BlockchainState>) -> Result<Json<String>, u32> {
    to_json(blockchain_mut_op(&state, |b| Ok(api::new_transaction(&transaction, b))))
}

// #[get("/chain")]
// pub fn chain<'a>(state: State<BlockchainState>) -> Result<Json<ChainResult>, u32> {
//     convert(blockchain_op(&state, |b| Ok(api::chain(b))))
// }

#[post("/nodes/register", format = "application/json", data="<node_list>")]
pub fn register_node(node_list: NodeList, state: State<BlockchainState>) -> Result<Json<RegisterNodeResponse>, u32> {
    to_json(blockchain_mut_op(&state, |b| api::register_node(&node_list, b)))
}

#[get("/nodes/resolve")]
pub fn consensus(state: State<BlockchainState>) -> Result<content::Json<String>, u32>  {
   
    let guard = state.blockchain.write();
    if guard.is_ok() {        
        let mut blockchain = guard.unwrap();
        let x = api::consensus(&mut blockchain);
        return Ok(content::Json(serde_json::to_string(&x).unwrap()));
    }
    return Err(500);
//         return Ok(Json(&));
//     }
    //Err(500)    
}

///
/// Take a Result<T> and return Json<T> if successful, otherwise
/// log the error and return Err(500)
/// 
fn to_json<T>(result: Result<T, String>) -> Result<Json<T>, u32> {
    match result {
        Ok(expr) => Ok(Json(expr)),
        Err(e) => {
            error!("{}", e);
            Err(500)
        }
    }
}

///
/// Retrieves the blockchain from state, unlocks for WRITE and executes the closure
/// 
fn blockchain_mut_op<F, T>(state: &State<BlockchainState>, blockchain_op: F) -> Result<T, String> 
    where F: Fn(&mut Blockchain) -> Result<T, String> {
    
    let guard = state.blockchain.write();
    if guard.is_ok() {        
        let mut blockchain = guard.unwrap();
        let result = blockchain_op(&mut blockchain);
        return result;
    }
    Err(format!("Couldn't acquire WRITE lock"))
}

///
/// Retrieves the blockchain from state, unlocks for READ and executes the closure
/// 
fn blockchain_op<F, T>(state: &State<BlockchainState>, blockchain_op: F) -> Result<T, String> 
    where F: Fn(&Blockchain) -> Result<T, String> {
    
    let guard = state.blockchain.read();
    if guard.is_ok() {        
        let blockchain = guard.unwrap();
        let result = blockchain_op(&blockchain);
        return result;
    }
    Err(format!("Couldn't acquire READ lock"))
}