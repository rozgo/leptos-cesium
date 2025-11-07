//! Cesium Cartesian2 (2D positions)

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
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

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct Cartesian2 {
    pub x: f64,
    pub y: f64,
}

#[cfg(not(target_arch = "wasm32"))]
impl Cartesian2 {
    pub fn new(x: f64, y: f64) -> Self {
        Cartesian2 { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}
