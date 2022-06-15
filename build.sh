#!/bin/bash
set -e &&
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release &&
mkdir -p artifacts &&
cp target/wasm32-unknown-unknown/release/*.wasm artifacts