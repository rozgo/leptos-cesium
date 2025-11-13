//! Cesium Rectangle utilities

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium)]
    pub type Rectangle;

    /// Gets the west coordinate in radians
    #[wasm_bindgen(method, getter)]
    pub fn west(this: &Rectangle) -> f64;

    /// Gets the south coordinate in radians
    #[wasm_bindgen(method, getter)]
    pub fn south(this: &Rectangle) -> f64;

    /// Gets the east coordinate in radians
    #[wasm_bindgen(method, getter)]
    pub fn east(this: &Rectangle) -> f64;

    /// Gets the north coordinate in radians
    #[wasm_bindgen(method, getter)]
    pub fn north(this: &Rectangle) -> f64;

    /// Gets the width of the rectangle in radians
    #[wasm_bindgen(method, getter)]
    pub fn width(this: &Rectangle) -> f64;

    /// Gets the height of the rectangle in radians
    #[wasm_bindgen(method, getter)]
    pub fn height(this: &Rectangle) -> f64;
}

/// Internal helper using reflection to call Cesium.Rectangle.fromDegrees
#[cfg(target_arch = "wasm32")]
fn from_degrees_impl(west: f64, south: f64, east: f64, north: f64) -> Rectangle {
    use js_sys::{Function, Reflect, global};
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

impl Rectangle {
    /// Create a Rectangle from west, south, east, north coordinates in degrees.
    ///
    /// Calls Cesium.Rectangle.fromDegrees internally.
    #[cfg(target_arch = "wasm32")]
    pub fn from_degrees(west: f64, south: f64, east: f64, north: f64) -> Self {
        from_degrees_impl(west, south, east, north)
    }
}
