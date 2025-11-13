//! Bindings for Cesium coordinate helpers.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = Cartesian3)]
    pub type Cartesian3;

    #[wasm_bindgen(constructor, js_namespace = Cesium, js_class = Cartesian3)]
    pub fn new(x: f64, y: f64, z: f64) -> Cartesian3;
}

/// Internal helper using reflection to call Cesium.Cartesian3.fromDegrees
#[cfg(target_arch = "wasm32")]
fn cartesian3_from_degrees_impl(longitude: f64, latitude: f64, height: f64) -> Cartesian3 {
    use js_sys::{Function, Reflect, global};
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

impl Cartesian3 {
    /// Create a Cartesian3 from longitude, latitude, and height in degrees.
    ///
    /// Calls Cesium.Cartesian3.fromDegrees internally.
    #[cfg(target_arch = "wasm32")]
    pub fn from_degrees(longitude: f64, latitude: f64, height: f64) -> Self {
        cartesian3_from_degrees_impl(longitude, latitude, height)
    }

    /// Create an array of Cartesian3 positions from a flat array of longitude, latitude pairs.
    ///
    /// Calls Cesium.Cartesian3.fromDegreesArray internally.
    #[cfg(target_arch = "wasm32")]
    pub fn from_degrees_array(degrees: &[f64]) -> js_sys::Array {
        cartesian3_from_degrees_array_impl(degrees)
    }

    /// Create an array of Cartesian3 positions from a flat array of longitude, latitude, height triples.
    ///
    /// Calls Cesium.Cartesian3.fromDegreesArrayHeights internally.
    #[cfg(target_arch = "wasm32")]
    pub fn from_degrees_array_heights(degrees: &[f64]) -> js_sys::Array {
        cartesian3_from_degrees_array_heights_impl(degrees)
    }
}

/// Internal helper using reflection to call Cesium.Cartesian3.fromDegreesArray
#[cfg(target_arch = "wasm32")]
fn cartesian3_from_degrees_array_impl(degrees: &[f64]) -> js_sys::Array {
    use js_sys::{Function, Reflect, global};
    use wasm_bindgen::{JsCast, JsValue};

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium"))
        .expect("Cesium global to be available");
    let cartesian3 = Reflect::get(&cesium, &JsValue::from_str("Cartesian3"))
        .expect("Cesium.Cartesian3 to exist");
    let from_degrees_array = Reflect::get(&cartesian3, &JsValue::from_str("fromDegreesArray"))
        .expect("Cesium.Cartesian3.fromDegreesArray to exist");
    let from_degrees_array_fn: Function = from_degrees_array
        .dyn_into()
        .expect("Cesium.Cartesian3.fromDegreesArray to be callable");

    // Use serde_wasm_bindgen to convert the slice to a JavaScript array
    let degrees_array =
        serde_wasm_bindgen::to_value(&degrees).expect("Failed to serialize degrees array");

    from_degrees_array_fn
        .call1(&JsValue::undefined(), &degrees_array)
        .expect("Cesium.Cartesian3.fromDegreesArray call to succeed")
        .dyn_into()
        .expect("Result of Cesium.Cartesian3.fromDegreesArray to be an Array")
}

/// Internal helper using reflection to call Cesium.Cartesian3.fromDegreesArrayHeights
#[cfg(target_arch = "wasm32")]
fn cartesian3_from_degrees_array_heights_impl(degrees: &[f64]) -> js_sys::Array {
    use js_sys::{Function, Reflect, global};
    use wasm_bindgen::{JsCast, JsValue};

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium"))
        .expect("Cesium global to be available");
    let cartesian3 = Reflect::get(&cesium, &JsValue::from_str("Cartesian3"))
        .expect("Cesium.Cartesian3 to exist");
    let from_degrees_array_heights =
        Reflect::get(&cartesian3, &JsValue::from_str("fromDegreesArrayHeights"))
            .expect("Cesium.Cartesian3.fromDegreesArrayHeights to exist");
    let from_degrees_array_heights_fn: Function = from_degrees_array_heights
        .dyn_into()
        .expect("Cesium.Cartesian3.fromDegreesArrayHeights to be callable");

    // Use serde_wasm_bindgen to convert the slice to a JavaScript array
    let degrees_array =
        serde_wasm_bindgen::to_value(&degrees).expect("Failed to serialize degrees array");

    from_degrees_array_heights_fn
        .call1(&JsValue::undefined(), &degrees_array)
        .expect("Cesium.Cartesian3.fromDegreesArrayHeights call to succeed")
        .dyn_into()
        .expect("Result of Cesium.Cartesian3.fromDegreesArrayHeights to be an Array")
}
