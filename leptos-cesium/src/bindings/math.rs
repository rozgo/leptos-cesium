//! Cesium Math utilities

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["Cesium", "Math"], js_name = toRadians)]
    pub fn to_radians(degrees: f64) -> f64;

    #[wasm_bindgen(js_namespace = ["Cesium", "Math"], js_name = toDegrees)]
    pub fn to_degrees(radians: f64) -> f64;

    #[wasm_bindgen(js_namespace = ["Cesium", "Math"], js_name = setRandomNumberSeed)]
    pub fn set_random_number_seed(seed: u32);
}
