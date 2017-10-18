

// //tood: to make testable... couldn't create Rocket::State, derived from state crate in tests
// pub fn mine(b: &mut Blockchain) -> Result<String, u32> {
//      let mined_block = b.mine();
//     let response = MineResult {
//         message: "New Block Forged".into(),
//         index: mined_block.index,
//         transactions: mined_block.transactions.clone(),
//         proof: mined_block.proof,
//         previous_hash: mined_block.previous_hash.clone()
//     };

//     Ok(serde_json::to_string(&response).unwrap_or_else(|e| {
//         error!("serialize error: {:?}", e);
//         return String::from("Block mined, but details not available")
//     }))
// }
