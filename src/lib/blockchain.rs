
use chrono;

use lib::hasher::*;
use lib::transaction::Transaction;
use std::collections::BTreeSet;
use std::collections::HashSet;
use self::chrono::offset::Utc;
use url::{Url};

pub type Chain = BTreeSet<Block>;

#[derive(Debug)]
pub struct Blockchain {
    chain: Chain,
    //not a lot of sorted options in stdlib...
    current_transactions: BTreeSet<Transaction>,
    nodes: HashSet<Url>,
    difficulty: u64
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Block {
    pub index: usize,
    pub timestamp: i64,
    pub proof: u64,
    pub previous_hash: String,
    pub transactions: BTreeSet<Transaction>
}

impl Blockchain {

    #[cfg(test)]
    pub fn new() -> Blockchain {
        Self::new_with(3)
    }
    pub fn new_with(difficulty: u64) -> Blockchain {
        let mut blockchain = Blockchain {
            chain: BTreeSet::new(),
            current_transactions: BTreeSet::new(),
            nodes: HashSet::new(),
            difficulty: difficulty
        };
        blockchain.new_block(100, String::from("Genesis block."));
        blockchain
    }
    
    ///
    /// Add a new transaction
    /// 
    /// returns: the index of the block it will be added to
    pub fn new_transaction(&mut self, transaction: Transaction) -> usize {        
        self.current_transactions.insert(transaction);
        //It will be added to the index of the next block
        self.last_block().index + 1
    }

    ///
    /// Mine a new block
    /// 
   pub fn mine(&mut self) -> Result<&Block, String> {
        // We run the proof of work algorithm to get the next proof...    
        let new_block_proof = self.proof_of_work()?;
        //Got it. Give ourselves the new coin (block?)
        //The sender is "0" to signify that this node has mined a new coin.
        self.new_transaction(Transaction::new("0".into(), "my node address".into(), 1));
        let previous_hash = self.hash_last_block()?;
        //Forge the new Block by adding it to the chain
        let mined_block = self.new_block(new_block_proof, previous_hash);
        Ok(&mined_block)
    }

    pub fn chain(&self) -> &Chain {
        &self.chain
    }

    #[cfg(test)]
    pub fn into_chain(self) -> Chain {
        self.chain
    }

    ///
    /// Add a new node
    /// 
    pub fn register_node(&mut self, address: Url) -> bool {
        self.nodes.insert(address)
    }

    pub fn nodes(&self) -> &HashSet<Url> {
        &self.nodes
    }

    pub fn replace(&mut self, new_chain: Chain) {
        self.chain = new_chain;
    }

    pub fn len(&self) -> usize {
        self.chain.len()
    }

    fn create_block(&mut self, proof: u64, previous_hash: String) -> Block {
        //Current transactions get moved to this block and are cleared to start
        //collecting the next block's transactions
        let mut txns = BTreeSet::new();
        txns.append(&mut self.current_transactions);
        Block {
            index: self.chain.len() + 1,
            timestamp: Utc::now().timestamp(),
            proof: proof,
            previous_hash: previous_hash,
            transactions: txns
        }
    }
    
    ///
    ///Create a new Block 
    ///
    fn new_block(&mut self, proof: u64, previous_hash: String) -> &Block {
        let block = self.create_block(proof, previous_hash);
        self.chain.insert(block);
        &self.chain.iter().next_back().expect("invariant: just added element")
    }
  
    fn last_block(&self) -> &Block {
        //it's a double-ended iterator, and it's sorted, so it should be fast
        self.chain.iter().next_back().expect("invariant: Chain empty. Expected genesis block")
    }

    //todo: get away from string errors
    fn hash(block: &Block) -> Result<String, String> {
       self::hash(block)
    }

    ///
    ///Simple Proof of Work Algorithm:
    /// Simple PoW algo:                                                                                                                                
    /// Find a number p' (new proof) s. t. hash(pp'h) contains 4 leading zeroes, where p is the 
    /// previous proof and h is the hash of the previous block.      
    /// 
    fn proof_of_work(&self) -> Result<u64, String> {
        
        let last_block = self.last_block();
        let last_proof = last_block.proof;

        info!("Mining from last_proof {}...", last_proof);
        let mut proof = 0;
        let previous_hash = self.hash_last_block()?;
        while !Self::valid_proof(last_proof, proof, self.difficulty, &previous_hash) {
             proof += 1;
        }
        debug!("Took {} iterations", proof);
        Ok(proof)
    }

    /// Validates the Proof
    /// i.e. does the hash of last_proof and this proof start with 000?
    fn valid_proof(last_proof: u64, proof: u64, difficulty: u64, previous_hash: &String) -> bool {
        
        //todo: don't recalculate every time
        let hash_prefix = "0".repeat(difficulty as usize); //"000"

        let guess = format!("{}{}{}", last_proof, proof, previous_hash);
        let guess_hash =  self::hash_string(guess);
        let is_valid = guess_hash.starts_with(hash_prefix.as_str());
        if is_valid {
            info!("proof {} -> guess_hash: {}", proof, guess_hash);
        } else {
            debug!("proof {} -> guess_hash: {}", proof, guess_hash);
        }
        is_valid
    }

    fn hash_last_block(&self) -> Result<String, String> {
        let last_block = self.last_block();
        Self::hash(last_block)
    }

    ///
    /// Determine if a given blockchain is valid
    /// 
    pub fn valid_chain(&self, chain: &Chain) -> bool {        
        debug!("{} blocks in chain.", chain.len());
        let mut previous_block_opt: Option<&Block> = None;        
        for block in chain {
            if let Some(previous_block) = previous_block_opt {
                //Check the hash and proof
                if !Self::check_hash(previous_block, block) || 
                   !Self::check_proof(previous_block, block, self.difficulty) {
                    return false;
                }               
            }
            previous_block_opt = Some(&block);
        }
        true
    }

    fn check_hash(previous_block: &Block, current_block: &Block) -> bool {
        let previous_block_hash = Self::hash(previous_block).unwrap_or_else(|e| format!("hash failure: {}", e));
        if current_block.previous_hash != previous_block_hash {
            warn!("HASH MISMATCH {} <> {}", current_block.previous_hash, previous_block_hash);
            return false
        }
        true
    }

    fn check_proof(previous_block: &Block, current_block: &Block, difficulty: u64) -> bool {
        let previous_hash = Self::hash(previous_block).unwrap_or_else(|e| format!("hash failure: {}", e));
        if !Self::valid_proof(previous_block.proof, current_block.proof, difficulty, &previous_hash) {                
            warn!("PROOF MISMATCH {} <> {}", previous_block.proof, current_block.proof);
            return false
        }
        true
    }
}

#[cfg(test)]
mod tests {
    //use env_logger;
    use lib::blockchain::Blockchain;
    use lib::transaction::Transaction;
    use url::Url;

    #[test]
    fn new_transaction() {
        let mut blockchain = Blockchain::new();
        let txn = Transaction::new(String::from("a"), String::from("b"), 100);
        let _idx = blockchain.new_transaction(txn);
        let last_txn = blockchain.current_transactions.iter().next_back().expect("expected a txn");
        assert_eq!(last_txn.sender, String::from("a"));
        assert_eq!(last_txn.recipient, String::from("b"));
        assert_eq!(last_txn.amount, 100);
    }

     #[test]
    fn new_block() {
        let mut blockchain = Blockchain::new();
        let txn = Transaction::new(String::from("a"), String::from("b"), 100);
        blockchain.new_transaction(txn);
        
        let a = blockchain.current_transactions.len();
        assert_eq!(1, a , "1 transaction");
    
        blockchain.new_block(2, String::from("abc"));
                 
        let b = blockchain.current_transactions.len();
        assert_eq!(0, b, "New block should clear transactions (which were on the previous block");    
    }
    
    #[test]
    fn hash() {
        let mut blockchain = Blockchain::new();       
        blockchain.new_block(2, String::from("abc"));
        let block = blockchain.last_block();
        let hash = Blockchain::hash(block);
        let hash2 = Blockchain::hash(block);
        println!("{:?}", hash);
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap(), hash2.unwrap(), "Expected same block to hash to the same value");
        //assert!(hash.unwrap().len() > 10, "expected a longer hash");       
    }

    #[test]
    fn valid_proof_false() {
        assert_eq!(Blockchain::valid_proof(100,1, 3, &String::from("some hash")), false);
    }
    
    #[cfg(feature = "mining-tests")]    
    #[test]
    fn proof_of_work() {
        let difficulty = 2;
        let blockchain = Blockchain::new_with(difficulty);     
        println!("Starting proof of work... (long running)");
        let proof = blockchain.proof_of_work().unwrap();
        println!("Finished proof of work: {}", proof);
        assert!(proof > 1, "expected a higher proof");
        let previous_hash = blockchain.hash_last_block().unwrap();
        assert!(Blockchain::valid_proof(100, proof, difficulty, &previous_hash));
        assert!(!Blockchain::valid_proof(100, proof, difficulty, &String::from("invalid hash")));
    }

    #[test]
    fn chain() {
        let mut blockchain = Blockchain::new();     
        
        assert_eq!(blockchain.chain().len(),  1, "Expected 1 block (genesis)");
        blockchain.new_block(100, "abc".into());
        assert_eq!(blockchain.chain().len(),  2, "Expected 2 blocks");
    }

    #[test]
    fn register_node() {
        let mut blockchain = Blockchain::new();     
        let test_local_url = Url::parse("http://localhost:9000").expect("valid url");
        blockchain.register_node(test_local_url.clone());
        assert_eq!(blockchain.nodes().len(),  1, "Expected 1 node");
        blockchain.register_node(test_local_url);
        assert_eq!(blockchain.nodes().len(),  1, "Expected 1 node after dupe add (idempotent)");
    }

    #[test]
    fn valid_chain_invalid_hash() {
        //env_logger::init().unwrap();
        let mut blockchain = Blockchain::new();
        let txn = Transaction::new(String::from("a"), String::from("b"), 100);
        blockchain.new_transaction(txn);
        //invalid hash
        blockchain.new_block(2, String::from("abc"));
        assert!(!blockchain.valid_chain(&blockchain.chain), "blockchain not valid (hash mismatch)");
    }


    #[test]
    fn valid_chain_invalid_proof() {
        let mut blockchain = Blockchain::new();
        let txn = Transaction::new(String::from("a"), String::from("b"), 100);
        blockchain.new_transaction(txn);
        //valid hash, invalid proof
        let hash = blockchain.hash_last_block().unwrap();
        blockchain.new_block(2, hash);

        assert!(!blockchain.valid_chain(&blockchain.chain), "blockchain not valid (proof mismatch)");
    }

    #[test]
    #[cfg(feature = "mining-tests")]    
    fn valid_chain_ok() {
        //env_logger::init().unwrap();
        let mut blockchain = Blockchain::new();
        let txn = Transaction::new(String::from("a"), String::from("b"), 100);
        blockchain.new_transaction(txn);
        //valid hash, invalid proof
        blockchain.mine();
        assert!(blockchain.valid_chain(&blockchain.chain), "blockchain should be valid with a mined block");
    }    
}