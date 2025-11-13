//! Bindings for Cesium coordinate helpers.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = Cartesian3)]
    pub type Cartesian3;

    #[wasm_bindgen(constructor, js_namespace = Cesium, js_class = Cartesian3)]
    pub fn new(x: f64, y: f64, z: f64) -> Cartesian3;
}

#[cfg(target_arch = "wasm32")]
pub fn cartesian3_from_degrees(longitude: f64, latitude: f64, height: f64) -> Cartesian3 {
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

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct Cartesian3;

#[cfg(not(target_arch = "wasm32"))]
impl Cartesian3 {
    pub fn new(_x: f64, _y: f64, _z: f64) -> Self {
        Cartesian3
    }
}

#[cfg(target_arch = "wasm32")]
pub fn cartesian3_from_degrees_array(degrees: &[f64]) -> js_sys::Array {
    use js_sys::{Array, Function, Reflect, global};
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

    let degrees_array = Array::new();
    for &value in degrees {
        degrees_array.push(&JsValue::from_f64(value));
    }

    from_degrees_array_fn
        .call1(&cartesian3, &degrees_array)
        .expect("Cesium.Cartesian3.fromDegreesArray call to succeed")
        .dyn_into()
        .expect("Result of Cesium.Cartesian3.fromDegreesArray to be an Array")
}

#[cfg(target_arch = "wasm32")]
pub fn cartesian3_from_degrees_array_heights(degrees: &[f64]) -> js_sys::Array {
    use js_sys::{Array, Function, Reflect, global};
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

    let degrees_array = Array::new();
    for &value in degrees {
        degrees_array.push(&JsValue::from_f64(value));
    }

    from_degrees_array_heights_fn
        .call1(&cartesian3, &degrees_array)
        .expect("Cesium.Cartesian3.fromDegreesArrayHeights call to succeed")
        .dyn_into()
        .expect("Result of Cesium.Cartesian3.fromDegreesArrayHeights to be an Array")
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub fn cartesian3_from_degrees(_longitude: f64, _latitude: f64, _height: f64) -> Cartesian3 {
    Cartesian3
}

#[cfg(not(target_arch = "wasm32"))]
pub fn cartesian3_from_degrees_array(_degrees: &[f64]) -> Vec<Cartesian3> {
    vec![]
}

#[cfg(not(target_arch = "wasm32"))]
pub fn cartesian3_from_degrees_array_heights(_degrees: &[f64]) -> Vec<Cartesian3> {
    vec![]
}
