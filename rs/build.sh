#!/bin/sh

cargo build --target wasm32-unknown-unknown --release

cp target/wasm32-unknown-unknown/release/render_lib.wasm lib.wasm

# Strip debug symbols
wasm-strip lib.wasm

# Even smaller
wasm-opt -o lib.wasm -Oz lib.wasm

sz=`du -h ./lib.wasm`
echo "wasm binary: ${sz}"
