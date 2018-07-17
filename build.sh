#!/bin/bash
cargo +nightly build --target wasm32-unknown-unknown && wasm-bindgen target/wasm32-unknown-unknown/debug/chip8.wasm --out-dir ./web