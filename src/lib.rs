extern crate wasm_bindgen;

pub mod cpu;

use wasm_bindgen::prelude::*;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
