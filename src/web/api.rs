use lib::blockchain::*;
use lib::transaction::*;
use lib::consensus::*;
use serde_json;
use serde::Serialize;
use url::{Url};
use web::types::*;

///
/// Mine a new block
/// 
pub fn mine(b: &mut Blockchain) -> Result<String, u32> {
    match b.mine() {
        Ok(mined_block) => {
            let response = MineResult {
                message: "New Block Forged".into(),
                index: mined_block.index,
                transactions: mined_block.transactions.clone(),
                proof: mined_block.proof,
                previous_hash: mined_block.previous_hash.clone()
            };
            serialize(&response)
        },
        Err(e) => {
            error!("Failed to mind block. {:?}", e);
            Err(500)
        }
    }    
}

///
/// Add a new transaction, which will be added to the next block.
/// 
/// # Returns the index of the next block.
/// 
pub fn new_transaction(transaction: &Transaction, b: &mut Blockchain) -> Result<String, u32> {   
    let index = b.new_transaction(transaction.clone());
    Ok(format!("Transaction added at block {}", index))
}

///
/// Return the whole blockchain (but not any pending transactions)
/// 
pub fn chain(b: &Blockchain) -> Result<String, u32> {
    
    let chain = b.chain();
    let response = ChainResult {
        chain: chain,
        length: chain.len()
    };
    serialize(&response)     
}

///
/// Add a new node to be called during conensus (conflict resolution)
/// 
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

    serialize(&response)
}

///
/// Determine which node has the longest blockchain, and replace with that
/// if it's not ours
/// 
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

fn serialize<T>(response: &T) -> Result<String, u32> where T: Serialize {
    match serde_json::to_string(&response) {
        Ok(serialized) => Ok(serialized),
        Err(e) => {
            error!("serialize error: {:?}", e);
            return Err(500); //include reason?
        }
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
