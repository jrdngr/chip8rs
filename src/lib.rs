extern crate wasm_bindgen;

pub mod cpu;
pub mod opcode;
pub mod graphics;
pub mod keyboard;

use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;
    use super::*;

    #[test]
    fn breakout() {
        let mut f = File::open("breakout.ch8").expect("File not found");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("Error reading file");
        println!("buffer: {}", buffer.len());

        let mut cpu = cpu::Cpu::new();
        cpu.load(&buffer);
        cpu.start();
    }
}
