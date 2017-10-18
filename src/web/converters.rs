extern crate rocket;

use web::NodeList;
use lib::transaction::Transaction;
use serde_json;
use serde;
use serde::Deserialize;
use rocket::{Request, Data};
use rocket::data::{self, FromData};
use rocket::http::{Status};
use rocket::Outcome::*;
use std::fmt::Debug;

fn deserialize<'a, T>(_: &Request, data: Data, type_name: String) -> data::Outcome<T, String>
    where for<'de> T: serde::Deserialize<'de> + Debug {

    let t: T = match serde_json::from_reader(data.open()) {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to deserialize {} {:?}", type_name, e);
            return Failure((Status::BadRequest, format!("Couldn't parse {}", type_name)));
        }
    };
    debug!("Successfully parsed {}. {:?}", type_name, t);
    Success(t)
}


impl FromData for Transaction {
    type Error = String;
    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {        
       deserialize(req, data, String::from("Transaction"))
    }
}

impl FromData for NodeList {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {        
       deserialize(req, data, String::from("NodeList"))
    }
}