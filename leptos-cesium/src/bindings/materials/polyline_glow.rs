//! Cesium PolylineGlowMaterialProperty

use crate::bindings::Color;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium)]
    pub type PolylineGlowMaterialProperty;

    #[wasm_bindgen(constructor, js_namespace = Cesium)]
    pub fn new(options: &JsValue) -> PolylineGlowMaterialProperty;
}

/// Builder for creating PolylineGlowMaterialProperty with a fluent API
#[derive(Default)]
pub struct PolylineGlowOptions {
    color: Option<Color>,
    glow_power: Option<f64>,
    taper_power: Option<f64>,
}

impl PolylineGlowOptions {
    /// Create a new PolylineGlowOptions builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the color of the polyline
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the glow power (strength of glow effect, 0.0 to 1.0)
    pub fn glow_power(mut self, glow_power: f64) -> Self {
        self.glow_power = Some(glow_power);
        self
    }

    /// Set the taper power (how the glow tapers off, 0.0 to 1.0)
    pub fn taper_power(mut self, taper_power: f64) -> Self {
        self.taper_power = Some(taper_power);
        self
    }

    /// Build the PolylineGlowMaterialProperty
    #[cfg(target_arch = "wasm32")]
    pub fn build(self) -> PolylineGlowMaterialProperty {
        use js_sys::{Object, Reflect};

        let options = Object::new();

        if let Some(color) = self.color {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("color"),
                &JsValue::from(color),
            );
        }
        if let Some(glow_power) = self.glow_power {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("glowPower"),
                &JsValue::from_f64(glow_power),
            );
        }
        if let Some(taper_power) = self.taper_power {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("taperPower"),
                &JsValue::from_f64(taper_power),
            );
        }

        PolylineGlowMaterialProperty::new(&options.into())
    }

    /// Build the PolylineGlowMaterialProperty
    #[cfg(not(target_arch = "wasm32"))]
    pub fn build(self) -> PolylineGlowMaterialProperty {
        PolylineGlowMaterialProperty
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct PolylineGlowMaterialProperty;

#[cfg(not(target_arch = "wasm32"))]
impl PolylineGlowMaterialProperty {
    pub fn new(_options: &()) -> Self {
        PolylineGlowMaterialProperty
    }
}
