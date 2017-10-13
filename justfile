build:
    cargo build

test:
    RUST_LOG=debug cargo test -- --nocapture

run:
    RUST_LOG=debug cargo run