extern crate wasm_bindgen;

pub mod cpu;
pub mod interpreter;
pub mod opcode;

use wasm_bindgen::prelude::*;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
