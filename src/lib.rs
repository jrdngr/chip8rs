#![feature(rust_2018_preview, use_extern_macros, wasm_custom_section, wasm_import_module)]

pub mod cpu;
pub mod compiler;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template() {
        assert_eq!(1, 1);
    }
}
