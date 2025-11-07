//! Cesium Material bindings

pub mod stripe;

pub use stripe::*;

use crate::bindings::Color;
use crate::core::ThreadSafeJsValue;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

/// Material types for entity graphics
pub enum Material {
    /// Solid color material
    Color(ThreadSafeJsValue<Color>),
    /// Stripe pattern material
    Stripe(ThreadSafeJsValue<StripeMaterialProperty>),
}

impl Clone for Material {
    fn clone(&self) -> Self {
        match self {
            Material::Color(c) => Material::Color(c.clone()),
            Material::Stripe(s) => Material::Stripe(s.clone()),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Material {
    /// Create a color material
    pub fn color(color: Color) -> Self {
        Material::Color(ThreadSafeJsValue::new(color))
    }

    /// Create a stripe material
    pub fn stripe(stripe: StripeMaterialProperty) -> Self {
        Material::Stripe(ThreadSafeJsValue::new(stripe))
    }

    /// Convert to JsValue for Cesium API
    pub fn to_js_value(&self) -> JsValue {
        match self {
            Material::Color(color) => JsValue::from(color.value().clone()),
            Material::Stripe(stripe) => JsValue::from(stripe.value().clone()),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Material {
    pub fn color(_color: Color) -> Self {
        Material::Color(ThreadSafeJsValue::new(_color))
    }

    pub fn stripe(_stripe: StripeMaterialProperty) -> Self {
        Material::Stripe(ThreadSafeJsValue::new(_stripe))
    }
}
