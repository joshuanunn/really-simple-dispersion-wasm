#![allow(dead_code)]
#![allow(unused_imports)]

mod rsdm;

use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}
