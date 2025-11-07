//! Cesium StripeMaterialProperty

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium)]
    pub type StripeMaterialProperty;

    #[wasm_bindgen(constructor, js_namespace = Cesium)]
    pub fn new(options: &JsValue) -> StripeMaterialProperty;
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct StripeMaterialProperty;

#[cfg(not(target_arch = "wasm32"))]
impl StripeMaterialProperty {
    pub fn new(_options: &()) -> Self {
        StripeMaterialProperty
    }
}
