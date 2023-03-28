use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[derive(Serialize, Deserialize)]
struct Group {
    vertices: Vec<Vec<f64>>,
}

#[derive(Serialize, Deserialize)]
struct InputJson {
    groups: Vec<Group>,
}

#[wasm_bindgen]
pub fn helloworld() -> Result<(), JsValue> {
    console::log_1(&"Hello from rust - this is run by WASM".into());
    Ok(())
}

#[wasm_bindgen]
pub fn process_json(json_string: &str) {
    let input_json: InputJson = serde_json::from_str(json_string).unwrap();
    let formatted_json = serde_json::to_string_pretty(&input_json).unwrap();
    console::log_1(&JsValue::from_str(&formatted_json));
}
