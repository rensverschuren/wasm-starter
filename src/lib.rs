extern crate stdweb;

use wasm_bindgen::prelude::*;
use web_sys::console;

use stdweb::web::alert;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello");
}