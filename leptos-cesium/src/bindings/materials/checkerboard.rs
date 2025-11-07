//! Cesium CheckerboardMaterialProperty

use crate::bindings::{Cartesian2, Color};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium)]
    pub type CheckerboardMaterialProperty;

    #[wasm_bindgen(constructor, js_namespace = Cesium)]
    pub fn new(options: &JsValue) -> CheckerboardMaterialProperty;
}

/// Builder for creating CheckerboardMaterialProperty with a fluent API
#[derive(Default)]
pub struct CheckerboardOptions {
    even_color: Option<Color>,
    odd_color: Option<Color>,
    repeat: Option<Cartesian2>,
}

impl CheckerboardOptions {
    /// Create a new CheckerboardOptions builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the even (light) square color
    pub fn even_color(mut self, color: Color) -> Self {
        self.even_color = Some(color);
        self
    }

    /// Set the odd (dark) square color
    pub fn odd_color(mut self, color: Color) -> Self {
        self.odd_color = Some(color);
        self
    }

    /// Set the number of times the checkerboard pattern repeats in each direction
    pub fn repeat(mut self, repeat: Cartesian2) -> Self {
        self.repeat = Some(repeat);
        self
    }

    /// Build the CheckerboardMaterialProperty
    #[cfg(target_arch = "wasm32")]
    pub fn build(self) -> CheckerboardMaterialProperty {
        use js_sys::{Object, Reflect};

        let options = Object::new();

        if let Some(color) = self.even_color {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("evenColor"),
                &JsValue::from(color),
            );
        }
        if let Some(color) = self.odd_color {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("oddColor"),
                &JsValue::from(color),
            );
        }
        if let Some(repeat) = self.repeat {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("repeat"),
                &JsValue::from(repeat),
            );
        }

        CheckerboardMaterialProperty::new(&options.into())
    }

    /// Build the CheckerboardMaterialProperty
    #[cfg(not(target_arch = "wasm32"))]
    pub fn build(self) -> CheckerboardMaterialProperty {
        CheckerboardMaterialProperty
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct CheckerboardMaterialProperty;

#[cfg(not(target_arch = "wasm32"))]
impl CheckerboardMaterialProperty {
    pub fn new(_options: &()) -> Self {
        CheckerboardMaterialProperty
    }
}
