use lib::blockchain::*;
use lib::transaction::*;
use lib::consensus::*;
use std::collections::BTreeSet;
use serde_json;
use url::{Url};
use web::NodeList;

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

//tood: to make testable... couldn't create Rocket::State, derived from state crate in tests
pub fn mine(b: &mut Blockchain) -> Result<String, u32> {
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
}

pub fn new_transaction(transaction: &Transaction, b: &mut Blockchain) -> Result<String, u32> {   
    let index = b.new_transaction(transaction.clone());
    Ok(format!("Transaction added at block {}", index))
}

pub fn chain(b: &mut Blockchain) -> Result<String, u32> {
    
    let chain = b.chain();
    let response = ChainResult {
        chain: chain,
        length: chain.len()
    };

    Ok(serde_json::to_string(&response).unwrap_or_else(|e| {
        error!("serialize error: {:?}", e);
        return String::from("Could not serialize chain.")
    }))
     
}

pub fn register_node(node_list: &NodeList, b: &mut Blockchain) -> Result<String, u32> {
   
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
}

pub fn consensus(b: &mut Blockchain) -> Result<String, u32> {

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

}

#[cfg(test)]
mod tests {
    use lib::blockchain::Blockchain;
    use web::api;

    #[test]
    fn mine() {
        let mut blockchain = Blockchain::new_with(1);
        let result = api::mine(&mut blockchain);
        assert!(result.is_ok(), format!("Failed to mine {:?}", result));
        println!("mine response: {}", result.unwrap());
    }
}