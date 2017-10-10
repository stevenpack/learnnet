
# LearnNet
Inspired by [https://hackernoon.com/learn-blockchains-by-building-one-117428612f46](https://hackernoon.com/learn-blockchains-by-building-one-117428612f46) and started as a fork of [https://github.com/dvf/blockchain](https://github.com/dvf/blockchain)

For my own learning purposes, but could be fun to stand it up somewhere publicly and make it a blockchain learning experiment for others... more to come.

### Dependenies
- [Rust nightly](https://rustup.rs/)
- [Just](https://crates.io/crates/just) (rust make-like command runner)  

### Build
`just build`

### Test
`just test`

### Run
`just run`  

Then, use Postman or similar interact.

## TODO

- Custom error types (get rid of map_err(|e| e.to_string()))

## Learnings and Questions

**Genesis**
What is special about it?

**Conensus**
How does this work if two nodes provide chains with different genesis blocks? I think BTC resolves with 51% hashpower? Other options? Or is that the first step of launching a coin? You specify the proof of the genesis block, so every subsequent block must go back to that.... still, if you as node got presented two chains, both the same length, and with the same genesis, but every single other transaction was different, how do you choose?
