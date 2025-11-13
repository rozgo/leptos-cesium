//! Bindings for Cesium camera-related types and options.

use crate::bindings::coordinates::Cartesian3;
use wasm_bindgen::prelude::*;

// ============================================================================
// HeadingPitchRoll
// ============================================================================

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = HeadingPitchRoll)]
    pub type HeadingPitchRoll;

    #[wasm_bindgen(constructor, js_namespace = Cesium, js_class = HeadingPitchRoll)]
    pub fn new(heading: f64, pitch: f64, roll: f64) -> HeadingPitchRoll;

    #[wasm_bindgen(method, getter)]
    pub fn heading(this: &HeadingPitchRoll) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn pitch(this: &HeadingPitchRoll) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn roll(this: &HeadingPitchRoll) -> f64;
}

#[cfg(target_arch = "wasm32")]
impl HeadingPitchRoll {
    /// Top-down view looking straight down
    pub fn top_down() -> Self {
        Self::new(0.0, -std::f64::consts::FRAC_PI_2, 0.0)
    }

    /// North-facing oblique view
    pub fn north_facing() -> Self {
        Self::new(0.0, -std::f64::consts::FRAC_PI_4, 0.0)
    }

    /// Default view (north-facing, slight angle)
    pub fn default_view() -> Self {
        Self::new(0.0, -std::f64::consts::FRAC_PI_6, 0.0)
    }
}

// ============================================================================
// HeadingPitchRange
// ============================================================================

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = HeadingPitchRange)]
    pub type HeadingPitchRange;

    #[wasm_bindgen(constructor, js_namespace = Cesium, js_class = HeadingPitchRange)]
    pub fn new(heading: f64, pitch: f64, range: f64) -> HeadingPitchRange;

    #[wasm_bindgen(method, getter)]
    pub fn heading(this: &HeadingPitchRange) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn pitch(this: &HeadingPitchRange) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn range(this: &HeadingPitchRange) -> f64;
}

// ============================================================================
// BoundingSphere
// ============================================================================

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = BoundingSphere)]
    pub type BoundingSphere;

    #[wasm_bindgen(constructor, js_namespace = Cesium, js_class = BoundingSphere)]
    pub fn new(center: &Cartesian3, radius: f64) -> BoundingSphere;

    #[wasm_bindgen(method, getter)]
    pub fn center(this: &BoundingSphere) -> Cartesian3;

    #[wasm_bindgen(method, getter)]
    pub fn radius(this: &BoundingSphere) -> f64;
}

// ============================================================================
// Camera Options Builders
// ============================================================================

/// Builder for Camera.flyTo() options
#[cfg(target_arch = "wasm32")]
pub struct FlyToOptions {
    destination: Cartesian3,
    orientation: Option<HeadingPitchRoll>,
    duration: Option<f64>,
    complete: Option<js_sys::Function>,
    cancel: Option<js_sys::Function>,
    offset: Option<HeadingPitchRange>,
}

#[cfg(target_arch = "wasm32")]
impl FlyToOptions {
    /// Create new FlyToOptions with required destination
    pub fn new(destination: Cartesian3) -> Self {
        Self {
            destination,
            orientation: None,
            duration: None,
            complete: None,
            cancel: None,
            offset: None,
        }
    }

    /// Set camera orientation (heading, pitch, roll)
    pub fn orientation(mut self, orientation: HeadingPitchRoll) -> Self {
        self.orientation = Some(orientation);
        self
    }

    /// Set flight duration in seconds (default: 3.0)
    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set offset from destination
    pub fn offset(mut self, offset: HeadingPitchRange) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set callback to execute when flight completes
    pub fn on_complete(mut self, callback: js_sys::Function) -> Self {
        self.complete = Some(callback);
        self
    }

    /// Set callback to execute if flight is cancelled
    pub fn on_cancel(mut self, callback: js_sys::Function) -> Self {
        self.cancel = Some(callback);
        self
    }

    /// Build the options object for Camera.flyTo()
    pub fn build(self) -> JsValue {
        use js_sys::{Object, Reflect};

        let options = Object::new();

        // Required destination
        let _ = Reflect::set(
            &options,
            &JsValue::from_str("destination"),
            &JsValue::from(self.destination),
        );

        // Optional orientation
        if let Some(orientation) = self.orientation {
            let orientation_obj = Object::new();
            let _ = Reflect::set(
                &orientation_obj,
                &JsValue::from_str("heading"),
                &JsValue::from_f64(orientation.heading()),
            );
            let _ = Reflect::set(
                &orientation_obj,
                &JsValue::from_str("pitch"),
                &JsValue::from_f64(orientation.pitch()),
            );
            let _ = Reflect::set(
                &orientation_obj,
                &JsValue::from_str("roll"),
                &JsValue::from_f64(orientation.roll()),
            );
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("orientation"),
                &orientation_obj,
            );
        }

        // Optional duration
        if let Some(duration) = self.duration {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("duration"),
                &JsValue::from_f64(duration),
            );
        }

        // Optional offset
        if let Some(offset) = self.offset {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("offset"),
                &JsValue::from(offset),
            );
        }

        // Optional complete callback
        if let Some(complete) = self.complete {
            let _ = Reflect::set(&options, &JsValue::from_str("complete"), &complete);
        }

        // Optional cancel callback
        if let Some(cancel) = self.cancel {
            let _ = Reflect::set(&options, &JsValue::from_str("cancel"), &cancel);
        }

        JsValue::from(options)
    }
}

/// Builder for Camera.setView() options
#[cfg(target_arch = "wasm32")]
pub struct SetViewOptions {
    destination: Cartesian3,
    orientation: Option<HeadingPitchRoll>,
}

#[cfg(target_arch = "wasm32")]
impl SetViewOptions {
    /// Create new SetViewOptions with required destination
    pub fn new(destination: Cartesian3) -> Self {
        Self {
            destination,
            orientation: None,
        }
    }

    /// Set camera orientation (heading, pitch, roll)
    pub fn orientation(mut self, orientation: HeadingPitchRoll) -> Self {
        self.orientation = Some(orientation);
        self
    }

    /// Build the options object for Camera.setView()
    pub fn build(self) -> JsValue {
        use js_sys::{Object, Reflect};

        let options = Object::new();

        // Required destination
        let _ = Reflect::set(
            &options,
            &JsValue::from_str("destination"),
            &JsValue::from(self.destination),
        );

        // Optional orientation
        if let Some(orientation) = self.orientation {
            let orientation_obj = Object::new();
            let _ = Reflect::set(
                &orientation_obj,
                &JsValue::from_str("heading"),
                &JsValue::from_f64(orientation.heading()),
            );
            let _ = Reflect::set(
                &orientation_obj,
                &JsValue::from_str("pitch"),
                &JsValue::from_f64(orientation.pitch()),
            );
            let _ = Reflect::set(
                &orientation_obj,
                &JsValue::from_str("roll"),
                &JsValue::from_f64(orientation.roll()),
            );
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("orientation"),
                &orientation_obj,
            );
        }

        JsValue::from(options)
    }
}
