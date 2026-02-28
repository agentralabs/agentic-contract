// AgenticContract WASM — WebAssembly bindings for npm
//
// Provides JavaScript/TypeScript access to AgenticContract policy engine.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
