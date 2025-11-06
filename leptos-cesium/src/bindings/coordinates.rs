//! Bindings for Cesium coordinate helpers.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Cesium, js_name = Cartesian3)]
    pub type Cartesian3;
}

#[cfg(target_arch = "wasm32")]
pub fn cartesian3_from_degrees(longitude: f64, latitude: f64, height: f64) -> Cartesian3 {
    use js_sys::{global, Function, Reflect};
    use wasm_bindgen::{JsCast, JsValue};

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium"))
        .expect("Cesium global to be available");
    let cartesian3 = Reflect::get(&cesium, &JsValue::from_str("Cartesian3"))
        .expect("Cesium.Cartesian3 to exist");
    let from_degrees = Reflect::get(&cartesian3, &JsValue::from_str("fromDegrees"))
        .expect("Cesium.Cartesian3.fromDegrees to exist");
    let from_degrees_fn: Function = from_degrees
        .dyn_into()
        .expect("Cesium.Cartesian3.fromDegrees to be callable");
    from_degrees_fn
        .call3(
            &cartesian3,
            &JsValue::from_f64(longitude),
            &JsValue::from_f64(latitude),
            &JsValue::from_f64(height),
        )
        .expect("Cesium.Cartesian3.fromDegrees call to succeed")
        .dyn_into()
        .expect("Result of Cesium.Cartesian3.fromDegrees to be a Cartesian3")
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct Cartesian3;

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub fn cartesian3_from_degrees(_longitude: f64, _latitude: f64, _height: f64) -> Cartesian3 {
    Cartesian3
}
