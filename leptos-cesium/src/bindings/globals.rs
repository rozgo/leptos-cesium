//! Bindings for Cesium global helpers.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Cesium::buildModuleUrl, js_name = setBaseUrl)]
    pub fn set_base_url(base_url: &str);
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub fn set_base_url(_base_url: &str) {}
