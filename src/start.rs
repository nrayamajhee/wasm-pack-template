use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

pub async fn start() -> Result<(), JsValue> {
    alert("Hello from Rust via WASM!");
    Ok(())
}

