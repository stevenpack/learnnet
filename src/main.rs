#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod blockchain;

use rocket::State;
use blockchain::Blockchain;
use std::sync::{RwLock,Mutex};

// lazy_static! {
//        static ref BLOCKCHAIN: Mutex<Blockchain> = Mutex::new(blockchain::Blockchain::new());
// }

struct MyConfig {
    //think this needs to be lazy_static! with mutex. It's global shared mutable state
    //Otherwise in storage with a connection pool
    //https://stackoverflow.com/questions/36230889/what-is-the-idiomatic-way-to-implement-caching-on-a-function-that-is-not-a-struc
    b: RwLock<Blockchain>
}

#[get("/raw")]
fn raw_config_value(state: State<MyConfig>) -> String {
    // use `inner()` to get a lifetime longer than `deref` gives us
    //state.inner().user_val.as_str()
    let mut blockchain = state.b.write().expect("Unable to get write lock");
    blockchain.new_block()
    // let blockchain = BLOCKCHAIN.lock().unwrap();
    // let msg = blockchain.new_block();
    // format!("raw {}", msg).to_string()
}

#[get("/raw2")]
fn raw_config_value2(state: State<MyConfig>) -> String {
    state.b.write().unwrap().new_transaction()
}

fn main() {
    let config = MyConfig {
        b: RwLock::new(Blockchain::new())
    };
    rocket::ignite()
    .manage(config)
    .mount("/", routes![raw_config_value, raw_config_value2]).launch();
}
