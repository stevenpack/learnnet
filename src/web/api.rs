use lib::blockchain::*;
use lib::transaction::*;
use lib::consensus::*;
use url::{Url};
use web::types::*;

///
/// Mine a new block
/// 
pub fn mine(b: &mut Blockchain) -> Result<MineResult, String> {
    match b.mine() {
        Ok(mined_block) => {
            Ok(MineResult {
                message: "New Block Forged".into(),
                index: mined_block.index,
                transactions: mined_block.transactions.clone(),
                proof: mined_block.proof,
                previous_hash: mined_block.previous_hash.clone()
            })
        },
        Err(e) => Err(format!("Failed to mind block. {:?}", e))
    }    
}

///
/// Add a new transaction, which will be added to the next block.
/// 
/// # Returns the index of the next block.
/// 
pub fn new_transaction(transaction: &Transaction, b: &mut Blockchain) -> String {   
    let index = b.new_transaction(transaction.clone());
    format!("Transaction added at block {}", index)
}

///
/// Return the whole blockchain (but not any pending transactions)
/// 
pub fn chain(b: &Blockchain) -> ChainResult {    
    let chain = b.chain();
    ChainResult {
        chain: chain,
        length: chain.len()
    }
}

///
/// Add a new node to be called during conensus (conflict resolution)
/// 
pub fn register_node(node_list: &NodeList, b: &mut Blockchain) -> Result<RegisterNodeResponse, String> {
   
    let mut node_urls = Vec::<Url>::with_capacity(node_list.nodes.len());

    //Validate - all or nothing
    for node in &node_list.nodes {
        let parse_result = Url::parse(node);
        if parse_result.is_err() {
            warn!("Failed to parse {} {:?}", node, parse_result.err());
            return Err(String::from("Failed to parse at least one node. All nodes must be valid"));
        }
        let url = parse_result.expect("validated");
        node_urls.push(url);
    }

    //Add
    for node_url in node_urls {
        b.register_node(node_url);
    }      

    Ok(RegisterNodeResponse {
        message: String::from("New nodes have been added"),
        total_nodes: b.nodes().len(),
    })
}

///
/// Determine which node has the longest blockchain, and replace with that
/// if it's not ours
/// 
pub fn consensus(b: &mut Blockchain) -> ConsensusReponse {

    let replaced = Consensus::resolve_conflicts(b);
    if replaced {
        ConsensusReponse {
            message: String::from("Our chain was replaced"),
            chain: None,
            new_chain: Some(b.chain())
        }
    }
    else
    {
         ConsensusReponse {
            message: String::from("Our chain is authoritative"),
            chain: Some(b.chain()),
            new_chain: None
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
