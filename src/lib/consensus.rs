
use lib::blockchain::{Chain,Blockchain};
use url::{Url};
use serde_json;
use reqwest::{Client};
use std::io::{Read};

pub struct Consensus;
impl Consensus {

    pub fn resolve_conflicts(blockchain: &mut Blockchain) -> bool {

        let mut new_chain: Option<Chain> = None;
        let mut max_length = blockchain.len();

        let mut urls = Vec::<Url>::new();
        for node in blockchain.nodes() {
            urls.push(node.clone());
        }
        
        let neighbour_chains = Self::get(urls.as_slice());
        for chain in neighbour_chains {
            if chain.len() > max_length && blockchain.valid_chain(&chain) {
                max_length = chain.len();
                new_chain = Some(chain);
            }
        }
        if let Some(longest_chain) = new_chain {
            blockchain.replace(longest_chain);
            return true;
        }
        false
    }

    fn get(urls: &[Url]) -> Vec<Chain> {
        let chains_raw = Self::get_neighbour_chains(urls);
        Self::deserialize(chains_raw)
    }

    fn get_neighbour_chains(urls: &[Url]) -> Vec<String> {
        let mut chains = Vec::<String>::new();
        let client = Client::new();
        //upgrade_todo: rayon or tokio-hyper to request async
        for url in urls {
            let mut res = client.get(url.as_str()).send().expect("todo: handle");
            let mut buffer = String::new();
            res.read_to_string(&mut buffer).expect("todo: handle");
            chains.push(buffer);
        }
        chains
    }

    fn deserialize(chains_raw: Vec<String>) -> Vec<Chain> {
        let mut chains = Vec::<Chain>::new();
        for raw in chains_raw {
            let chain: Chain = serde_json::from_str(raw.as_str()).unwrap();
            chains.push(chain);
        }
        chains
    }
}

#[cfg(test)]
mod tests {    
    use lib::consensus::Consensus;
    use url::Url;

    #[cfg(feature = "integration")]   
    #[test]
    fn get_neighbour_chains() {
        let urls = [Url::parse("http://koalasafe.com").unwrap()];
        Consensus::get_neighbour_chains(&urls);
    }
}
  