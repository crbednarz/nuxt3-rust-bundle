use viewer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, World!");
}

#[wasm_bindgen]
pub fn run() {
    viewer::run();
}

#[wasm_bindgen]
pub fn test() -> JsValue {
    JsValue::from_str("hello, world")
}
