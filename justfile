build:
    cargo build

test:
    cargo test -- --nocapture

run:
    RUST_LOG=debug cargo run