use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // JavaScript
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = Date)]
    pub fn now() -> u32;    
    // Random
    #[wasm_bindgen(module = "./js/index")]
    pub fn getRandomSeed() -> i32;
    // Display
    #[wasm_bindgen(module = "./js/index")]
    pub fn setPixel(x: u8, y: u8);
    #[wasm_bindgen(module = "./js/index")]
    pub fn clearScreen();
    // Keyboard
    #[wasm_bindgen(module = "./js/index")]
    pub fn isKeyDown(x: i32) -> bool;
    #[wasm_bindgen(module = "./js/index")]
    pub fn getAnyKey() -> i32;
}