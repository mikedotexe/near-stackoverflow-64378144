!#/bin/bash
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/veggie_example.wasm res