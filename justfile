build:
    cargo build

test:
    cargo test

test-debug:
    RUST_LOG=debug

test-int:
    cargo test --no-default-features --features integration

test-mine:
     cargo test --no-default-features --features mining-tests

run port='8000':
    RUST_LOG=debug cargo run -- -p={{port}} 