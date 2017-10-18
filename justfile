build:
    cargo build

test:
    RUST_LOG=debug cargo test -- --nocapture

test-int:
    RUST_LOG=debug cargo test --features integration -- --nocapture

test-all:
    RUST_LOG=debug cargo test --features mining-tests -- --nocapture

run port='8000':
    RUST_LOG=debug cargo run -- -p={{port}} 