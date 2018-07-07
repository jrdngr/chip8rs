#!/bin/bash 

if ["$1" == "release"]; then 
    cargo +nightly build --target wasm32-unknown-unknown
else 
    cargo +nightly build --release --target wasm32-unknown-unknown
fi