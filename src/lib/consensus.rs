
use std::collections::BTreeSet;
use lib::blockchain::{Chain,Block};
use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::{Uri, Client, Response, Chunk};
use tokio_core::reactor::Core;
use url::{Url};

struct Consensus {    }

impl Consensus {

    fn get_neighbour_chains(urls: &[Url]) -> Vec<String> {

        let mut core = Core::new().unwrap();
        let http_client = Client::new(&core.handle());

        let mut chains = Vec::<String>::new();
        for url in urls {
            let uri = url.as_str().parse().unwrap();
            let work = http_client.get(uri).map(|res| {
                println!("RES {:?}", res);
                res.body().concat2().and_then(move |body: Chunk| {
                    println!("BODY {:?}", body);
                    Ok(())
                });
            });
            core.run(work);
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
        let urls = [Url::parse("http://google.com").unwrap()];
        Consensus::get_neighbour_chains(&urls);
    }
}
  