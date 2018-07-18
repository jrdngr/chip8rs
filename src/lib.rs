#![feature(use_extern_macros, proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

pub mod cpu;
pub mod opcode;
pub mod keyboard;
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
