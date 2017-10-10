use serde_json;
use std::collections::BTreeSet;
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

//todo: respone as JSON - https://github.com/SergioBenitez/Rocket/blob/v0.3.3/examples/json/src/main.rs
#[get("/mine", format = "application/json")]
pub fn mine(state: State<BlockchainState>) -> Result<String, u32> {
    return blockchain_op(&state, |b| {

        let mined_block = b.mine();
        let response = MineResult {
            message: "New Block Forged".into(),
            index: mined_block.index,
            transactions: mined_block.transactions.clone(),
            proof: mined_block.proof,
            previous_hash: mined_block.previous_hash.clone()
        };

        serde_json::to_string(&response).unwrap_or_else(|e| {
            error!("serialize error: {:?}", e);
            return String::from("Block mined, but details not available")
        })
    });
}

#[post("/transaction/new", format = "application/json", data = "<transaction>")]
pub fn new_transaction(transaction: Transaction, state: State<BlockchainState>) -> Result<String, u32> {
    blockchain_op(&state, |b| {
        let index = b.new_transaction(transaction.clone());
        return format!("Transaction added at block {}", index);
    })
}

#[get("/chain", format = "application/json")]
pub fn chain(state: State<BlockchainState>) -> Result<String, u32> {
    return blockchain_op(&state, |b| {

            let chain = b.chain();
            let response = ChainResult {
              chain: chain,
              length: chain.len()
            };

            serde_json::to_string(&response).unwrap_or_else(|e| {
                error!("serialize error: {:?}", e);
                return String::from("Could not serialize chain.")
            })
        });
}

#[post("/nodes/register", format = "application/json", data="<node_list>")]
pub fn register_node(node_list: NodeList, state: State<BlockchainState>) -> Result<String, u32> {
    blockchain_op(&state, |b| {
        
        //values = request.get_json()

        //     nodes = values.get('nodes')
        //     if nodes is None:
        //         return "Error: Please supply a valid list of nodes", 400

        //     for node in nodes:
        //         blockchain.register_node(node)

        //     response = {
        //         'message': 'New nodes have been added',
        //         'total_nodes': list(blockchain.nodes),
        //     }
        //     return jsonify(response), 201
        for x in &node_list.nodes {
            debug!("{}", x);
        }
        //let url = Url::parse();
        //let index = b.register_node(url);
        return format!("Added nodes");
    })
}

///
/// Retrieves the blockchain from state, unlocks and executes the closure
/// 
fn blockchain_op<F>(state: &State<BlockchainState>, blockchain_op: F) -> Result<String, u32> 
    where F: Fn(&mut Blockchain) -> String {
    let guard = state.blockchain.write();
    if guard.is_ok() {        
        let mut blockchain = guard.unwrap();
        let result = blockchain_op(&mut blockchain);
        return Ok(result);        
    }
    error!("Couldn't acquire lock");
    Err(500)
}


