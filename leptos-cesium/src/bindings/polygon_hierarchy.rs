//! Cesium PolygonHierarchy for polygons with holes

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium)]
    pub type PolygonHierarchy;

    #[wasm_bindgen(constructor, js_namespace = Cesium)]
    pub fn new(positions: &JsValue, holes: &JsValue) -> PolygonHierarchy;

    #[wasm_bindgen(constructor, js_namespace = Cesium)]
    pub fn new_simple(positions: &JsValue) -> PolygonHierarchy;
}
