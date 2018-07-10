extern crate wasm_bindgen;

pub mod cpu;
pub mod opcode;
pub mod graphics;
pub mod keyboard;

use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
