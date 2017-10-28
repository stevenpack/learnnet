use lib::blockchain::*;
use lib::transaction::*;

use std::collections::BTreeSet;

///
/// Strongly typed list of nodes for /nodes/register
/// 
#[derive(Debug, Deserialize)]
pub struct NodeList {
    pub nodes: Vec<String>
}

///
/// Strongly typed response for mining
/// 
#[derive(Debug,Serialize)]
pub struct MineResult {
    pub message: String,
    pub index: usize,
    pub transactions: BTreeSet<Transaction>,
    pub proof: u64,
    pub previous_hash: String
}

///
/// Strongly typed response for requesting the blockchain
/// 
#[derive(Serialize)]
pub struct ChainResult<'a> {
    pub chain: &'a BTreeSet<Block>,
    pub length: usize
}

///
/// Strongly typed response for registering a node
/// 
#[derive(Serialize)]
pub struct RegisterNodeResponse {
    pub message: String,
    pub total_nodes: usize
}

#[derive(Serialize)]
pub struct ConsensusReponse<'a> {
    pub message: String,
    pub chain: Option<&'a Chain>,
    pub new_chain: Option<&'a Chain>
}