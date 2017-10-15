
use std::collections::BTreeSet;
use lib::blockchain::{Chain,Block};
use url::{Url};
use serde_json;
use serde_json::Value;
use reqwest::{Client, Response};
use std::io::{Read};

struct Consensus;
impl Consensus {

    pub fn get(urls: &[Url]) -> Vec<Chain> {
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
  