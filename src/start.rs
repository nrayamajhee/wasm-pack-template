use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

pub async fn run() -> Result<(), JsValue> {
    alert!("Hello from Rust via WASM!");
}

