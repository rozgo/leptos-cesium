//! Cesium DataSource bindings for CZML and other data formats

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    /// Collection of DataSource instances
    #[wasm_bindgen(js_namespace = Cesium, js_name = DataSourceCollection)]
    pub type DataSourceCollection;

    #[wasm_bindgen(method, js_name = add)]
    pub fn add(this: &DataSourceCollection, data_source: js_sys::Promise) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = removeAll)]
    pub fn remove_all(this: &DataSourceCollection);

    /// CZML data source
    #[wasm_bindgen(js_namespace = Cesium, js_name = CzmlDataSource)]
    pub type CzmlDataSource;

    #[wasm_bindgen(static_method_of = CzmlDataSource, js_name = load)]
    pub fn load(url: &str) -> js_sys::Promise;

    #[wasm_bindgen(static_method_of = CzmlDataSource, js_name = load)]
    pub fn load_with_options(url: &str, options: &JsValue) -> js_sys::Promise;
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct DataSourceCollection;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct CzmlDataSource;
