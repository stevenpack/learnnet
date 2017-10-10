extern crate rocket;

use web::NodeList;
use lib::transaction::Transaction;
use serde_json;
use rocket::{Request, Data};
use rocket::data::{self, FromData};
use rocket::http::{Status};
use rocket::Outcome::*;

impl FromData for Transaction {
    type Error = String;

    fn from_data(_: &Request, data: Data) -> data::Outcome<Self, String> {
        
        let transaction: Transaction = match serde_json::from_reader(data.open()) {
            Ok(transaction) => transaction,
            Err(e) => {
                error!("Failed to deserialize transaction {:?}", e);
                return Failure((Status::BadRequest, format!("Couldn't parse transaction")));
            }
        };
        debug!("Successfully parsed transaction. {:?}", transaction);
        Success(transaction)
    }
}

impl FromData for NodeList {
    type Error = String;

    fn from_data(_: &Request, data: Data) -> data::Outcome<Self, String> {
        
        //Terser way to express?
        let node_list: NodeList = match serde_json::from_reader(data.open()) {
            Ok(node_list) => node_list,
            Err(e) => {
                error!("Failed to deserialize node_list {:?}", e);
                return Failure((Status::BadRequest, format!("Couldn't parse node_list")));
            }
        };
        debug!("Successfully parsed node_list");
        Success(node_list)
    }
}