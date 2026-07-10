use wasm_bindgen::prelude::*; // example

#[wasm_bindgen]
pub fn rust_has_something_to_say() {
    return "apple".to_string();
}
