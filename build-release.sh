#!/bin/bash
cargo +nightly build --release --target wasm32-unknown-unknown && wasm-bindgen target/wasm32-unknown-unknown/release/chip8.wasm --out-dir .