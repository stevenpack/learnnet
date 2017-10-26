extern crate rocket;

use web::types::*;
use lib::transaction::Transaction;
use serde_json;
use serde;
use rocket::{Request, Data};
use rocket::data::{self, FromData};
use rocket::http::{Status};
use rocket::Outcome::*;
use std::fmt::Debug;

//Converters so Rocket methods can have strongly typed params. They are deserialized here

fn deserialize<'a, T>(_: &Request, data: Data, type_name: String) -> data::Outcome<T, String>
    where for<'de> T: serde::Deserialize<'de> + Debug {

    match serde_json::from_reader(data.open()) {
        Ok(t) => {
            debug!("Successfully parsed {}. {:?}", type_name, t);
            Success(t)
        },
        Err(e) => {
            error!("Failed to deserialize {} {:?}", type_name, e);
            Failure((Status::BadRequest, format!("Couldn't parse {}", type_name)))
        }
    }
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