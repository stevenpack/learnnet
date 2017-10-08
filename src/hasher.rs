use serde;
use serde_json;
use base64;
use std::fmt::Debug;
use std::result::Result;
use serde_json::Error;
use sha2::{Sha256, Digest};


pub fn hash<T>(t: &T) -> Result<String, String> where T: serde::Serialize + Debug {
    
    let json = try!(serde_json::to_string(t).map_err(|e| e.to_string()));
    Ok(hash_string(json))
}

pub fn hash_string(string: String) -> String {
    let mut hasher = Sha256::default();
    hasher.input(string.as_bytes());
    let base64_hash = base64::encode(hasher.result().as_slice());
    debug!("struct {:?} -> hash: {:?}", string, base64_hash);
    base64_hash
}