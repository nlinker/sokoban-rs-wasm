use wasm_bindgen::prelude::*;

// start here

#[wasm_bindgen]
pub fn add_one(x: u32) -> u32 {
    x + 1
}
