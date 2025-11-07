//! Cesium Math utilities

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["Cesium", "Math"], js_name = toRadians)]
    pub fn to_radians(degrees: f64) -> f64;

    #[wasm_bindgen(js_namespace = ["Cesium", "Math"], js_name = toDegrees)]
    pub fn to_degrees(radians: f64) -> f64;

    #[wasm_bindgen(js_namespace = ["Cesium", "Math"], js_name = setRandomNumberSeed)]
    pub fn set_random_number_seed(seed: u32);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn to_radians(degrees: f64) -> f64 {
    degrees.to_radians()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn to_degrees(radians: f64) -> f64 {
    radians.to_degrees()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn set_random_number_seed(_seed: u32) {
    // No-op for SSR
}
