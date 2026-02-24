//! Cesium billboard bindings and typed origin enums.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = BillboardGraphics)]
    pub type BillboardGraphics;

    #[wasm_bindgen(constructor, js_namespace = Cesium, js_class = BillboardGraphics)]
    pub fn new(options: &JsValue) -> BillboardGraphics;
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct BillboardGraphics;

#[cfg(not(target_arch = "wasm32"))]
impl BillboardGraphics {
    pub fn new(_options: &()) -> Self {
        Self
    }
}

/// Cesium `HorizontalOrigin` enum values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalOrigin {
    Left,
    Center,
    Right,
}

impl HorizontalOrigin {
    #[cfg(target_arch = "wasm32")]
    fn as_cesium_name(self) -> &'static str {
        match self {
            HorizontalOrigin::Left => "LEFT",
            HorizontalOrigin::Center => "CENTER",
            HorizontalOrigin::Right => "RIGHT",
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn to_js_value(self) -> JsValue {
        cesium_enum_value("HorizontalOrigin", self.as_cesium_name())
    }
}

/// Cesium `VerticalOrigin` enum values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalOrigin {
    Bottom,
    Center,
    Top,
    Baseline,
}

impl VerticalOrigin {
    #[cfg(target_arch = "wasm32")]
    fn as_cesium_name(self) -> &'static str {
        match self {
            VerticalOrigin::Bottom => "BOTTOM",
            VerticalOrigin::Center => "CENTER",
            VerticalOrigin::Top => "TOP",
            VerticalOrigin::Baseline => "BASELINE",
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn to_js_value(self) -> JsValue {
        cesium_enum_value("VerticalOrigin", self.as_cesium_name())
    }
}

#[cfg(target_arch = "wasm32")]
fn cesium_enum_value(enum_name: &str, value_name: &str) -> JsValue {
    use js_sys::{Reflect, global};

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium"))
        .expect("Cesium global to be available");
    let enum_object = Reflect::get(&cesium, &JsValue::from_str(enum_name))
        .expect("Cesium enum object to be available");
    Reflect::get(&enum_object, &JsValue::from_str(value_name))
        .expect("Cesium enum value to be available")
}
