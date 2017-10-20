
use lib::blockchain::{Chain,Blockchain};
use serde_json;
use reqwest::{Client, StatusCode};
use std::io::{Read};

#[derive(Deserialize)]
struct ChainResponse {
    chain: Chain
}

pub struct Consensus;
impl Consensus {

    pub fn resolve_conflicts(blockchain: &mut Blockchain) -> bool {
        
        let nodes: Vec<String> = blockchain
                                    .nodes()
                                    .iter()
                                    .cloned()
                                    .map(|node| node.into_string())
                                    .collect();
        
        let neighbour_chains = Self::get(nodes.as_slice());
        Self::take_authoritive(blockchain, neighbour_chains)
    }

    fn take_authoritive(blockchain: &mut Blockchain, chains: Vec<Chain>) -> bool {
        
        let mut is_replaced = false;
        let mut new_chain: Option<Chain> = None;
        let mut max_length = blockchain.len();
        
        for chain in chains {
            if chain.len() > max_length && blockchain.valid_chain(&chain) {
                max_length = chain.len();
                new_chain = Some(chain);
            }
        }
        
        if let Some(longest_chain) = new_chain {
            blockchain.replace(longest_chain);
            is_replaced = true;
        }
        is_replaced
    }
   
    fn get(nodes: &[String]) -> Vec<Chain> {
        let chains_raw = Self::get_neighbour_chains(nodes);
        Self::deserialize(chains_raw)
    }

    fn get_neighbour_chains(nodes: &[String]) -> Vec<String> {
        let mut chains = Vec::<String>::new();
        let client = Client::new();
        //upgrade: rayon or tokio-hyper to request async
        for node in nodes {
            let url = format!("{}/chain", node);

            match client.get(url.as_str()).send() {
                Ok(mut res) => {
                    if res.status() == StatusCode::Ok {
                        let mut buffer = String::new();
                        let bytes_read = res.read_to_string(&mut buffer).unwrap_or_else(|e| {
                            error!("Couldnt' read buffer {}", e);
                            return 0;
                        });
                        if bytes_read > 0 {
                            chains.push(buffer);
                        }                        
                    } else {
                        error!("Failed to get chain from {}. Response was {:?}. Ignoring", url, res)
                    }
                },
                Err(e) => error!("Failed to get chain from {}. Error was {:?}. Ignoring", url, e)
            }
        }
        chains
    }

    fn deserialize(chains_raw: Vec<String>) -> Vec<Chain> {
        let mut chains = Vec::<Chain>::new();
        for raw in chains_raw {
            //upgrade: remove nodes who return invalid chains?
            match serde_json::from_str::<ChainResponse>(raw.as_str()) {
                Ok(chain_res) => chains.push(chain_res.chain),
                Err(e) => error!("Unable to deserialize chain {:?} raw: {}", e, raw)
            }            
        }
        chains
    }
}

#[cfg(test)]
mod tests {    
    use lib::blockchain::Blockchain;
    use lib::consensus::Consensus;
    use env_logger;
    
    #[cfg(feature = "integration")]   
    #[test]
    fn get_neighbour_chains() {
        env_logger::init().unwrap();
        let urls = vec![String::from("http://koalasafe.com")];
        Consensus::get(urls.as_slice());
    }

    #[test]
    fn take_authoritive() {
        //Same or less blocks we keep our own. Longer we replace
        let mut blockchain_1 = Blockchain::new_with(1);
        let mut blockchain_2 = Blockchain::new_with(1);

        blockchain_1.mine();
        assert!(!Consensus::take_authoritive(&mut blockchain_1, vec![blockchain_2.into_chain()]), "1 block vs 0 blocks (don't replace)");
        
        blockchain_1 = Blockchain::new_with(1);
        blockchain_2 = Blockchain::new_with(1);
        blockchain_1.mine();        
        blockchain_2.mine();
        assert!(!Consensus::take_authoritive(&mut blockchain_1, vec![blockchain_2.into_chain()]), "1 block vs 1 blocks (don't replace)");
       
        blockchain_1 = Blockchain::new_with(1);
        blockchain_2 = Blockchain::new_with(1);
        blockchain_1.mine();        
        blockchain_2.mine();
        blockchain_2.mine();
        assert!(Consensus::take_authoritive(&mut blockchain_1, vec![blockchain_2.into_chain()]), "1 block vs 2 blocks (replace)");
    }
}
  