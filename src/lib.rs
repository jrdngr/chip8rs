#![feature(rust_2018_preview, use_extern_macros, wasm_custom_section, wasm_import_module)]

pub mod cpu;
pub mod compiler;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;
    use super::*;

    #[test]
    #[ignore]
    fn template() {
        let mut f = File::open("breakout.src").expect("File not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Error reading file");
    }
}
