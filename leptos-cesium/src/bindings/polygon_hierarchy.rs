//! Cesium PolygonHierarchy for polygons with holes

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
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

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct PolygonHierarchy;

#[cfg(not(target_arch = "wasm32"))]
impl PolygonHierarchy {
    pub fn new(_positions: &(), _holes: &()) -> Self {
        PolygonHierarchy
    }

    pub fn new_simple(_positions: &()) -> Self {
        PolygonHierarchy
    }
}
