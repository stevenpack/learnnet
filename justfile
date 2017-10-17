build:
    cargo build

test:
    RUST_LOG=debug cargo test -- --nocapture

test-int:
    cargo test --features integration -- --nocapture

test-all:
    RUST_LOG=debug cargo test --features mining-tests -- --nocapture

run:
    RUST_LOG=debug cargo run