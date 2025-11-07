//! Cesium StripeMaterialProperty

use crate::bindings::Color;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium)]
    pub type StripeMaterialProperty;

    #[wasm_bindgen(constructor, js_namespace = Cesium)]
    pub fn new(options: &JsValue) -> StripeMaterialProperty;
}

/// Builder for creating StripeMaterialProperty with a fluent API
#[derive(Default)]
pub struct StripeOptions {
    even_color: Option<Color>,
    odd_color: Option<Color>,
    repeat: Option<f64>,
    offset: Option<f64>,
}

impl StripeOptions {
    /// Create a new StripeOptions builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the even stripe color
    pub fn even_color(mut self, color: Color) -> Self {
        self.even_color = Some(color);
        self
    }

    /// Set the odd stripe color
    pub fn odd_color(mut self, color: Color) -> Self {
        self.odd_color = Some(color);
        self
    }

    /// Set the number of times the stripes repeat
    pub fn repeat(mut self, repeat: f64) -> Self {
        self.repeat = Some(repeat);
        self
    }

    /// Set the offset of the stripes
    pub fn offset(mut self, offset: f64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Build the StripeMaterialProperty
    #[cfg(target_arch = "wasm32")]
    pub fn build(self) -> StripeMaterialProperty {
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
                &JsValue::from_f64(repeat),
            );
        }
        if let Some(offset) = self.offset {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("offset"),
                &JsValue::from_f64(offset),
            );
        }

        StripeMaterialProperty::new(&options.into())
    }

    /// Build the StripeMaterialProperty
    #[cfg(not(target_arch = "wasm32"))]
    pub fn build(self) -> StripeMaterialProperty {
        StripeMaterialProperty
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct StripeMaterialProperty;

#[cfg(not(target_arch = "wasm32"))]
impl StripeMaterialProperty {
    pub fn new(_options: &()) -> Self {
        StripeMaterialProperty
    }
}
