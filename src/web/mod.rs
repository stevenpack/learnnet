use serde_json;
use std::collections::BTreeSet;
use rocket::State;
use lib::blockchain::*;
use lib::consensus::Consensus;
use lib::transaction::*;
use std::sync::{RwLock};
use url::{Url};
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

//Requests
#[derive(Deserialize)]
pub struct NodeList {
    nodes: Vec<String>
}

//Responses
#[derive(Serialize)]
struct MineResult {
    message: String,
    index: usize,
    transactions: BTreeSet<Transaction>,
    proof: u64,
    previous_hash: String
}

#[derive(Serialize)]
struct ChainResult<'a> {
    chain: &'a BTreeSet<Block>,
    length: usize
}

#[derive(Serialize)]
struct RegisterNodeResponse {
    message: String,
    total_nodes: usize
}

//todo: respone as JSON - https://github.com/SergioBenitez/Rocket/blob/v0.3.3/examples/json/src/main.rs
#[get("/mine", format = "application/json")]
pub fn mine(state: State<BlockchainState>) -> Result<String, u32> {
    blockchain_op(&state, |b| {

        let mined_block = b.mine();
        let response = MineResult {
            message: "New Block Forged".into(),
            index: mined_block.index,
            transactions: mined_block.transactions.clone(),
            proof: mined_block.proof,
            previous_hash: mined_block.previous_hash.clone()
        };

        Ok(serde_json::to_string(&response).unwrap_or_else(|e| {
            error!("serialize error: {:?}", e);
            return String::from("Block mined, but details not available")
        }))
    })
}

#[post("/transaction/new", format = "application/json", data = "<transaction>")]
pub fn new_transaction(transaction: Transaction, state: State<BlockchainState>) -> Result<String, u32> {
    blockchain_op(&state, |b| {
        let index = b.new_transaction(transaction.clone());
        return Ok(format!("Transaction added at block {}", index));
    })
}

#[get("/chain", format = "application/json")]
pub fn chain(state: State<BlockchainState>) -> Result<String, u32> {
    blockchain_op(&state, |b| {

            let chain = b.chain();
            let response = ChainResult {
              chain: chain,
              length: chain.len()
            };

            Ok(serde_json::to_string(&response).unwrap_or_else(|e| {
                error!("serialize error: {:?}", e);
                return String::from("Could not serialize chain.")
            }))
        })
}

#[post("/nodes/register", format = "application/json", data="<node_list>")]
pub fn register_node(node_list: NodeList, state: State<BlockchainState>) -> Result<String, u32> {
    return blockchain_op(&state, |b| {

        let mut node_urls = Vec::<Url>::with_capacity(node_list.nodes.len());

        //Validate - all or nothing
        for node in &node_list.nodes {
           let parse_result = Url::parse(node);
           if parse_result.is_err() {
               warn!("Failed to parse {} {:?}", node, parse_result.err());
               return Err(400, /* all nodes must be valid */)
           }
           let url = parse_result.expect("validated");
           node_urls.push(url);
        }

        //Add
        for node_url in node_urls {
            b.register_node(node_url);
        }      

        let response = RegisterNodeResponse {
            message: String::from("New nodes have been added"),
            total_nodes: b.nodes().len(),
        };

        Ok(serde_json::to_string(&response).unwrap_or_else(|e| {
                error!("serialize error: {:?}", e);
                return String::from("Could not serialize chain.")
        }))       
    })
}

#[get("/nodes/resolve")]
pub fn consensus(state: State<BlockchainState>) -> Result<String, u32> {
    return blockchain_op(&state, |b| {
        let replaced = Consensus::resolve_conflicts(b);
        if replaced {
            return Ok(json!({
                "message": "Our chain was replaced",
                "new_chain": b.chain()
            }).to_string());
        }
        else
        {
            return Ok(json!({
                "message": "Our chain is authoritative",
                "chain": b.chain()
            }).to_string());
        }
    });
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


