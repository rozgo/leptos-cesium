//! Cesium Rectangle utilities

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium)]
    pub type Rectangle;
}

#[cfg(target_arch = "wasm32")]
pub fn from_degrees(west: f64, south: f64, east: f64, north: f64) -> Rectangle {
    use js_sys::{global, Function, Reflect};
    use wasm_bindgen::{JsCast, JsValue};

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium"))
        .expect("Cesium global to be available");
    let rectangle =
        Reflect::get(&cesium, &JsValue::from_str("Rectangle")).expect("Cesium.Rectangle to exist");
    let from_degrees_fn = Reflect::get(&rectangle, &JsValue::from_str("fromDegrees"))
        .expect("Cesium.Rectangle.fromDegrees to exist");
    let from_degrees_fn: Function = from_degrees_fn
        .dyn_into()
        .expect("Cesium.Rectangle.fromDegrees to be callable");
    from_degrees_fn
        .call4(
            &rectangle,
            &JsValue::from_f64(west),
            &JsValue::from_f64(south),
            &JsValue::from_f64(east),
            &JsValue::from_f64(north),
        )
        .expect("Cesium.Rectangle.fromDegrees to succeed")
        .unchecked_into::<Rectangle>()
}

#[cfg(target_arch = "wasm32")]
impl Rectangle {
    pub fn from_degrees(west: f64, south: f64, east: f64, north: f64) -> Self {
        from_degrees(west, south, east, north)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct Rectangle;

#[cfg(not(target_arch = "wasm32"))]
impl Rectangle {
    pub fn from_degrees(_west: f64, _south: f64, _east: f64, _north: f64) -> Self {
        Rectangle
    }
}
