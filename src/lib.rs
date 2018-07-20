#![feature(rust_2018_preview, use_extern_macros, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

pub mod opcode;
pub mod hardware;
pub mod rng;

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;
    use super::*;

    #[test]
    fn template() {
        assert_eq!(1, 1);
    }
}
