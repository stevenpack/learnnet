extern crate rocket;

use lib::transaction::Transaction;
use serde_json;
use rocket::{Request, Data, Outcome};
use rocket::data::{self, FromData};
use rocket::http::{Status, ContentType};
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