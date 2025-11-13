//! Cesium Cartesian2 (2D positions)

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Cesium)]
    pub type Cartesian2;

    #[wasm_bindgen(constructor, js_namespace = Cesium)]
    pub fn new(x: f64, y: f64) -> Cartesian2;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Cartesian2) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Cartesian2) -> f64;
}
