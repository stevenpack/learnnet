
use serde_json;
use chrono;

use std::cmp::Ord;
use std::cmp::Ordering;

use std::collections::BTreeSet;
use std::collections::HashSet;
use self::chrono::offset::Utc;

type Address = String;
type Amount = i64;

pub struct Blockchain {
    chain: BTreeSet<Block>,
    current_transactions: BTreeSet<Transaction>,
    nodes: HashSet<String>
}

#[derive(Serialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Transaction {
    sender: Address,
    recipient: Address,
    amount: Amount
}

#[derive(Serialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Block {
    index: usize,
    timestamp: i64,
    proof: i64,
    previous_hash: String,
    transactions: BTreeSet<Transaction>
}

// impl Ord for Block {
//     fn cmp(&self, other: &Block) -> Ordering {
//         self.index.cmp(&other.index)
//     }
// }

// impl PartialEq for Block {
//     fn eq(&self, other: &Block) -> bool {
//         self.index == other.index
//     }
// }

impl Transaction {
    pub fn new(sender: Address, recipient: Address, amount: Amount) -> Transaction {
        Transaction {
            sender: sender,
            recipient: recipient,
            amount: amount
        }
    }
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            chain: BTreeSet::new(),
            current_transactions: BTreeSet::new(),
            nodes: HashSet::new()
        };
        //todo: how to do genesis?
        blockchain.new_block(100, String::from("some hash..."));
        blockchain
    }
    
      
    fn create_block(&self, proof: i64, previous_hash: String) -> Block {
        Block {
            index: self.chain.len() + 1,
            timestamp: Utc::now().timestamp(),
            proof: proof,
            previous_hash: previous_hash, //or self.hash(self.chain[-1]),
            transactions: BTreeSet::new()
        }
    }

    ///
    ///Create a new Block 
    ///
    pub fn new_block(&mut self, proof: i64, previous_hash: String) -> &Block {

        let block = self.create_block(proof, previous_hash);

        self.current_transactions = BTreeSet::new();
        self.chain.insert(block);
        &self.chain.iter().next_back().expect("Just added element")
    }

    pub fn new_transaction(&mut self, sender: Address, recipient: Address, amount: Amount) -> usize {        
        let txn = Transaction::new(sender, recipient, amount);        
        self.current_transactions.insert(txn);
        self.last_block().index
    }

    pub fn last_block(&self) -> &Block {
        //for set, it's self.chain.iter().next_back()
        //it's a double-ended iterator, and it's sorted, so it should be fast
        self.chain.iter().next_back().expect("chain empty. expected genesis block")
    }

    pub fn hash(&self, block: &Block) -> String {
        //let j = serde_json::to_string(block)?;
        let j = serde_json::to_string(block);
        println!("{:?}", j);
        String::from("a")
    }

}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use blockchain::Blockchain;

    #[test]
    fn new_transaction() {
        let mut blockchain = Blockchain::new();
        let idx = blockchain.new_transaction(String::from("a"), String::from("b"), 100);
        let last_txn = blockchain.current_transactions.iter().next_back().expect("expected a txn");
        assert_eq!(last_txn.sender, String::from("a"));
        assert_eq!(last_txn.recipient, String::from("b"));
        assert_eq!(last_txn.amount, 100);
    }

     #[test]
    fn new_block() {
        let mut blockchain = Blockchain::new();
        blockchain.new_transaction(String::from("a"), String::from("b"), 100);
        
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
        let hash = blockchain.hash(block);
        println!("{:?}", hash);
    }
}

// import hashlib
// import json
// from time import time
// from urllib.parse import urlparse
// from uuid import uuid4

// import requests
// from flask import Flask, jsonify, request


// class Blockchain(object):
//     def __init__(self):
//         self.current_transactions = []
//         self.chain = []
//         self.nodes = set()

//         # Create the genesis block
//         self.new_block(previous_hash=1, proof=100)

//     def register_node(self, address):
//         """
//         Add a new node to the list of nodes

//         :param address: <str> Address of node. Eg. 'http://192.168.0.5:5000'
//         :return: None
//         """

//         parsed_url = urlparse(address)
//         self.nodes.add(parsed_url.netloc)

//     def valid_chain(self, chain):
//         """
//         Determine if a given blockchain is valid

//         :param chain: <list> A blockchain
//         :return: <bool> True if valid, False if not
//         """

//         last_block = chain[0]
//         current_index = 1

//         while current_index < len(chain):
//             block = chain[current_index]
//             print(f'{last_block}')
//             print(f'{block}')
//             print("\n-----------\n")
//             # Check that the hash of the block is correct
//             if block['previous_hash'] != self.hash(last_block):
//                 return False

//             # Check that the Proof of Work is correct
//             if not self.valid_proof(last_block['proof'], block['proof']):
//                 return False

//             last_block = block
//             current_index += 1

//         return True

//     def resolve_conflicts(self):
//         """
//         This is our consensus algorithm, it resolves conflicts
//         by replacing our chain with the longest one in the network.

//         :return: <bool> True if our chain was replaced, False if not
//         """

//         neighbours = self.nodes
//         new_chain = None

//         # We're only looking for chains longer than ours
//         max_length = len(self.chain)

//         # Grab and verify the chains from all the nodes in our network
//         for node in neighbours:
//             response = requests.get(f'http://{node}/chain')

//             if response.status_code == 200:
//                 length = response.json()['length']
//                 chain = response.json()['chain']

//                 # Check if the length is longer and the chain is valid
//                 if length > max_length and self.valid_chain(chain):
//                     max_length = length
//                     new_chain = chain

//         # Replace our chain if we discovered a new, valid chain longer than ours
//         if new_chain:
//             self.chain = new_chain
//             return True

//         return False

//     def new_block(self, proof, previous_hash=None):
//         """
//         Create a new Block in the Blockchain

//         :param proof: <int> The proof given by the Proof of Work algorithm
//         :param previous_hash: (Optional) <str> Hash of previous Block
//         :return: <dict> New Block
//         """

//         block = {
//             'index': len(self.chain) + 1,
//             'timestamp': time(),
//             'transactions': self.current_transactions,
//             'proof': proof,
//             'previous_hash': previous_hash or self.hash(self.chain[-1]),
//         }

//         # Reset the current list of transactions
//         self.current_transactions = []

//         self.chain.append(block)
//         return block

//     def new_transaction(self, sender, recipient, amount):
//         """
//         Creates a new transaction to go into the next mined Block

//         :param sender: <str> Address of the Sender
//         :param recipient: <str> Address of the Recipient
//         :param amount: <int> Amount
//         :return: <int> The index of the Block that will hold this transaction
//         """
//         self.current_transactions.append({
//             'sender': sender,
//             'recipient': recipient,
//             'amount': amount,
//         })

//         return self.last_block['index'] + 1

//     @property
//     def last_block(self):
//         return self.chain[-1]

//     @staticmethod
//     def hash(block):
//         """
//         Creates a SHA-256 hash of a Block

//         :param block: <dict> Block
//         :return: <str>
//         """

//         # We must make sure that the Dictionary is Ordered, or we'll have inconsistent hashes
//         block_string = json.dumps(block, sort_keys=True).encode()
//         return hashlib.sha256(block_string).hexdigest()

//     def proof_of_work(self, last_proof):
//         """
//         Simple Proof of Work Algorithm:
//          - Find a number p' such that hash(pp') contains leading 4 zeroes, where p is the previous p'
//          - p is the previous proof, and p' is the new proof

//         :param last_proof: <int>
//         :return: <int>
//         """

//         proof = 0
//         while self.valid_proof(last_proof, proof) is False:
//             proof += 1

//         return proof

//     @staticmethod
//     def valid_proof(last_proof, proof):
//         """
//         Validates the Proof

//         :param last_proof: <int> Previous Proof
//         :param proof: <int> Current Proof
//         :return: <bool> True if correct, False if not.
//         """

//         guess = f'{last_proof}{proof}'.encode()
//         guess_hash = hashlib.sha256(guess).hexdigest()
//         return guess_hash[:4] == "0000"


// # Instantiate the Node
// app = Flask(__name__)

// # Generate a globally unique address for this node
// node_identifier = str(uuid4()).replace('-', '')

// # Instantiate the Blockchain
// blockchain = Blockchain()


// @app.route('/mine', methods=['GET'])
// def mine():
//     # We run the proof of work algorithm to get the next proof...
//     last_block = blockchain.last_block
//     last_proof = last_block['proof']
//     proof = blockchain.proof_of_work(last_proof)

//     # We must receive a reward for finding the proof.
//     # The sender is "0" to signify that this node has mined a new coin.
//     blockchain.new_transaction(
//         sender="0",
//         recipient=node_identifier,
//         amount=1,
//     )

//     # Forge the new Block by adding it to the chain
//     block = blockchain.new_block(proof)

//     response = {
//         'message': "New Block Forged",
//         'index': block['index'],
//         'transactions': block['transactions'],
//         'proof': block['proof'],
//         'previous_hash': block['previous_hash'],
//     }
//     return jsonify(response), 200


// @app.route('/transactions/new', methods=['POST'])
// def new_transaction():
//     values = request.get_json()

//     # Check that the required fields are in the POST'ed data
//     required = ['sender', 'recipient', 'amount']
//     if not all(k in values for k in required):
//         return 'Missing values', 400

//     # Create a new Transaction
//     index = blockchain.new_transaction(values['sender'], values['recipient'], values['amount'])

//     response = {'message': f'Transaction will be added to Block {index}'}
//     return jsonify(response), 201


// @app.route('/chain', methods=['GET'])
// def full_chain():
//     response = {
//         'chain': blockchain.chain,
//         'length': len(blockchain.chain),
//     }
//     return jsonify(response), 200


// @app.route('/nodes/register', methods=['POST'])
// def register_nodes():
//     values = request.get_json()

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


// @app.route('/nodes/resolve', methods=['GET'])
// def consensus():
//     replaced = blockchain.resolve_conflicts()

//     if replaced:
//         response = {
//             'message': 'Our chain was replaced',
//             'new_chain': blockchain.chain
//         }
//     else:
//         response = {
//             'message': 'Our chain is authoritative',
//             'chain': blockchain.chain
//         }

//     return jsonify(response), 200


// if __name__ == '__main__':
//     app.run(host='0.0.0.0', port=5000)