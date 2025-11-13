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

    #[wasm_bindgen(method, getter, js_name = clock)]
    pub fn clock(this: &CzmlDataSource) -> DataSourceClock;

    /// DataSource clock that defines the time range
    #[wasm_bindgen(js_namespace = Cesium, js_name = DataSourceClock)]
    pub type DataSourceClock;
}

// Helper to call CzmlDataSource.load() using reflection
#[cfg(target_arch = "wasm32")]
pub fn czml_data_source_load(url: &str) -> js_sys::Promise {
    use js_sys::{Function, Reflect, global};
    use wasm_bindgen::JsCast;

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium"))
        .expect("Cesium global to be available");
    let czml_data_source = Reflect::get(&cesium, &JsValue::from_str("CzmlDataSource"))
        .expect("Cesium.CzmlDataSource to exist");
    let load_fn = Reflect::get(&czml_data_source, &JsValue::from_str("load"))
        .expect("Cesium.CzmlDataSource.load to exist");
    let load_fn: Function = load_fn
        .dyn_into()
        .expect("Cesium.CzmlDataSource.load to be callable");

    load_fn
        .call1(&czml_data_source, &JsValue::from_str(url))
        .expect("Cesium.CzmlDataSource.load to succeed")
        .unchecked_into::<js_sys::Promise>()
}

// Helper to call CzmlDataSource.load() with options
#[cfg(target_arch = "wasm32")]
pub fn czml_data_source_load_with_options(url: &str, options: &JsValue) -> js_sys::Promise {
    use js_sys::{Function, Reflect, global};
    use wasm_bindgen::JsCast;

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium"))
        .expect("Cesium global to be available");
    let czml_data_source = Reflect::get(&cesium, &JsValue::from_str("CzmlDataSource"))
        .expect("Cesium.CzmlDataSource to exist");
    let load_fn = Reflect::get(&czml_data_source, &JsValue::from_str("load"))
        .expect("Cesium.CzmlDataSource.load to exist");
    let load_fn: Function = load_fn
        .dyn_into()
        .expect("Cesium.CzmlDataSource.load to be callable");

    load_fn
        .call2(&czml_data_source, &JsValue::from_str(url), options)
        .expect("Cesium.CzmlDataSource.load to succeed")
        .unchecked_into::<js_sys::Promise>()
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct DataSourceCollection;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct CzmlDataSource;
