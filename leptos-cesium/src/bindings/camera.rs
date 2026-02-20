//! Bindings for Cesium camera-related types and options.

use crate::bindings::coordinates::Cartesian3;
#[cfg(target_arch = "wasm32")]
use crate::bindings::rectangle::Rectangle;
#[cfg(target_arch = "wasm32")]
use crate::bindings::viewer::Matrix4;
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

/// Destination accepted by Camera.setView/flyTo.
#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub enum CameraDestination {
    Cartesian3(Cartesian3),
    Rectangle(Rectangle),
}

#[cfg(target_arch = "wasm32")]
impl From<Cartesian3> for CameraDestination {
    fn from(value: Cartesian3) -> Self {
        Self::Cartesian3(value)
    }
}

#[cfg(target_arch = "wasm32")]
impl From<Rectangle> for CameraDestination {
    fn from(value: Rectangle) -> Self {
        Self::Rectangle(value)
    }
}

/// Direction/up orientation variant accepted by Camera.setView/flyTo.
#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub struct DirectionUp {
    direction: Cartesian3,
    up: Cartesian3,
}

#[cfg(target_arch = "wasm32")]
impl DirectionUp {
    pub fn new(direction: Cartesian3, up: Cartesian3) -> Self {
        Self { direction, up }
    }
}

/// Orientation accepted by Camera.setView/flyTo.
#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub enum CameraOrientation {
    HeadingPitchRoll(HeadingPitchRoll),
    DirectionUp(DirectionUp),
}

#[cfg(target_arch = "wasm32")]
impl From<HeadingPitchRoll> for CameraOrientation {
    fn from(value: HeadingPitchRoll) -> Self {
        Self::HeadingPitchRoll(value)
    }
}

#[cfg(target_arch = "wasm32")]
impl From<DirectionUp> for CameraOrientation {
    fn from(value: DirectionUp) -> Self {
        Self::DirectionUp(value)
    }
}

/// Builder for Camera.flyTo() options
#[cfg(target_arch = "wasm32")]
pub struct FlyToOptions {
    destination: CameraDestination,
    orientation: Option<CameraOrientation>,
    duration: Option<f64>,
    complete: Option<js_sys::Function>,
    cancel: Option<js_sys::Function>,
    end_transform: Option<Matrix4>,
    convert: Option<bool>,
    maximum_height: Option<f64>,
    pitch_adjust_height: Option<f64>,
    fly_over_longitude: Option<f64>,
    fly_over_longitude_weight: Option<f64>,
    easing_function: Option<js_sys::Function>,
}

#[cfg(target_arch = "wasm32")]
impl FlyToOptions {
    /// Create new FlyToOptions with required destination
    pub fn new(destination: impl Into<CameraDestination>) -> Self {
        Self {
            destination: destination.into(),
            orientation: None,
            duration: None,
            complete: None,
            cancel: None,
            end_transform: None,
            convert: None,
            maximum_height: None,
            pitch_adjust_height: None,
            fly_over_longitude: None,
            fly_over_longitude_weight: None,
            easing_function: None,
        }
    }

    /// Set camera orientation using heading/pitch/roll.
    pub fn orientation_hpr(mut self, orientation: HeadingPitchRoll) -> Self {
        self.orientation = Some(CameraOrientation::HeadingPitchRoll(orientation));
        self
    }

    /// Set camera orientation using direction/up vectors.
    pub fn orientation_direction_up(mut self, direction: Cartesian3, up: Cartesian3) -> Self {
        self.orientation = Some(CameraOrientation::DirectionUp(DirectionUp::new(
            direction, up,
        )));
        self
    }

    /// Set camera orientation with a pre-built orientation enum.
    pub fn orientation(mut self, orientation: impl Into<CameraOrientation>) -> Self {
        self.orientation = Some(orientation.into());
        self
    }

    /// Set flight duration in seconds (default: 3.0)
    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set end transform matrix.
    pub fn end_transform(mut self, transform: Matrix4) -> Self {
        self.end_transform = Some(transform);
        self
    }

    /// Set convert flag (only relevant outside 3D).
    pub fn convert(mut self, convert: bool) -> Self {
        self.convert = Some(convert);
        self
    }

    /// Set maximum height reached during flight.
    pub fn maximum_height(mut self, maximum_height: f64) -> Self {
        self.maximum_height = Some(maximum_height);
        self
    }

    /// Set pitch adjustment height.
    pub fn pitch_adjust_height(mut self, pitch_adjust_height: f64) -> Self {
        self.pitch_adjust_height = Some(pitch_adjust_height);
        self
    }

    /// Set fly-over longitude in radians.
    pub fn fly_over_longitude(mut self, value: f64) -> Self {
        self.fly_over_longitude = Some(value);
        self
    }

    /// Set fly-over longitude weight.
    pub fn fly_over_longitude_weight(mut self, value: f64) -> Self {
        self.fly_over_longitude_weight = Some(value);
        self
    }

    /// Set easing function callback.
    pub fn easing_function(mut self, callback: js_sys::Function) -> Self {
        self.easing_function = Some(callback);
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
        let destination_js = match self.destination {
            CameraDestination::Cartesian3(value) => JsValue::from(value),
            CameraDestination::Rectangle(value) => JsValue::from(value),
        };
        let _ = Reflect::set(&options, &JsValue::from_str("destination"), &destination_js);

        // Optional orientation
        if let Some(orientation) = self.orientation {
            let orientation_obj = Object::new();
            match orientation {
                CameraOrientation::HeadingPitchRoll(orientation) => {
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
                }
                CameraOrientation::DirectionUp(orientation) => {
                    let _ = Reflect::set(
                        &orientation_obj,
                        &JsValue::from_str("direction"),
                        &JsValue::from(orientation.direction),
                    );
                    let _ = Reflect::set(
                        &orientation_obj,
                        &JsValue::from_str("up"),
                        &JsValue::from(orientation.up),
                    );
                }
            }
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
        if let Some(end_transform) = self.end_transform {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("endTransform"),
                &JsValue::from(end_transform),
            );
        }
        if let Some(convert) = self.convert {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("convert"),
                &JsValue::from_bool(convert),
            );
        }
        if let Some(value) = self.maximum_height {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("maximumHeight"),
                &JsValue::from_f64(value),
            );
        }
        if let Some(value) = self.pitch_adjust_height {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("pitchAdjustHeight"),
                &JsValue::from_f64(value),
            );
        }
        if let Some(value) = self.fly_over_longitude {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("flyOverLongitude"),
                &JsValue::from_f64(value),
            );
        }
        if let Some(value) = self.fly_over_longitude_weight {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("flyOverLongitudeWeight"),
                &JsValue::from_f64(value),
            );
        }
        if let Some(easing_function) = self.easing_function {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("easingFunction"),
                &easing_function,
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
    destination: Option<CameraDestination>,
    orientation: Option<CameraOrientation>,
    end_transform: Option<Matrix4>,
    convert: Option<bool>,
}

#[cfg(target_arch = "wasm32")]
impl SetViewOptions {
    /// Create a new empty SetViewOptions builder.
    pub fn new() -> Self {
        Self {
            destination: None,
            orientation: None,
            end_transform: None,
            convert: None,
        }
    }

    /// Set destination.
    pub fn destination(mut self, destination: impl Into<CameraDestination>) -> Self {
        self.destination = Some(destination.into());
        self
    }

    /// Set orientation with heading/pitch/roll.
    pub fn orientation_hpr(mut self, orientation: HeadingPitchRoll) -> Self {
        self.orientation = Some(CameraOrientation::HeadingPitchRoll(orientation));
        self
    }

    /// Set orientation with direction/up vectors.
    pub fn orientation_direction_up(mut self, direction: Cartesian3, up: Cartesian3) -> Self {
        self.orientation = Some(CameraOrientation::DirectionUp(DirectionUp::new(
            direction, up,
        )));
        self
    }

    /// Set orientation with a pre-built orientation enum.
    pub fn orientation(mut self, orientation: impl Into<CameraOrientation>) -> Self {
        self.orientation = Some(orientation.into());
        self
    }

    /// Set end transform matrix.
    pub fn end_transform(mut self, transform: Matrix4) -> Self {
        self.end_transform = Some(transform);
        self
    }

    /// Set convert flag (only relevant outside 3D).
    pub fn convert(mut self, convert: bool) -> Self {
        self.convert = Some(convert);
        self
    }

    /// Build the options object for Camera.setView()
    pub fn build(self) -> JsValue {
        use js_sys::{Object, Reflect};

        let options = Object::new();

        // Optional destination
        if let Some(destination) = self.destination {
            let destination_js = match destination {
                CameraDestination::Cartesian3(value) => JsValue::from(value),
                CameraDestination::Rectangle(value) => JsValue::from(value),
            };
            let _ = Reflect::set(&options, &JsValue::from_str("destination"), &destination_js);
        }

        // Optional orientation
        if let Some(orientation) = self.orientation {
            let orientation_obj = Object::new();
            match orientation {
                CameraOrientation::HeadingPitchRoll(orientation) => {
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
                }
                CameraOrientation::DirectionUp(orientation) => {
                    let _ = Reflect::set(
                        &orientation_obj,
                        &JsValue::from_str("direction"),
                        &JsValue::from(orientation.direction),
                    );
                    let _ = Reflect::set(
                        &orientation_obj,
                        &JsValue::from_str("up"),
                        &JsValue::from(orientation.up),
                    );
                }
            }
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("orientation"),
                &orientation_obj,
            );
        }
        if let Some(end_transform) = self.end_transform {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("endTransform"),
                &JsValue::from(end_transform),
            );
        }
        if let Some(convert) = self.convert {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("convert"),
                &JsValue::from_bool(convert),
            );
        }

        JsValue::from(options)
    }
}

/// Builder for Camera.flyToBoundingSphere() options.
#[cfg(target_arch = "wasm32")]
pub struct FlyToBoundingSphereOptions {
    duration: Option<f64>,
    maximum_height: Option<f64>,
    pitch_adjust_height: Option<f64>,
    offset: Option<HeadingPitchRange>,
    complete: Option<js_sys::Function>,
    cancel: Option<js_sys::Function>,
    easing_function: Option<js_sys::Function>,
}

#[cfg(target_arch = "wasm32")]
impl FlyToBoundingSphereOptions {
    pub fn new() -> Self {
        Self {
            duration: None,
            maximum_height: None,
            pitch_adjust_height: None,
            offset: None,
            complete: None,
            cancel: None,
            easing_function: None,
        }
    }

    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn maximum_height(mut self, value: f64) -> Self {
        self.maximum_height = Some(value);
        self
    }

    pub fn pitch_adjust_height(mut self, value: f64) -> Self {
        self.pitch_adjust_height = Some(value);
        self
    }

    pub fn offset(mut self, offset: HeadingPitchRange) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn on_complete(mut self, callback: js_sys::Function) -> Self {
        self.complete = Some(callback);
        self
    }

    pub fn on_cancel(mut self, callback: js_sys::Function) -> Self {
        self.cancel = Some(callback);
        self
    }

    pub fn easing_function(mut self, callback: js_sys::Function) -> Self {
        self.easing_function = Some(callback);
        self
    }

    pub fn build(self) -> JsValue {
        use js_sys::{Object, Reflect};

        let options = Object::new();

        if let Some(duration) = self.duration {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("duration"),
                &JsValue::from_f64(duration),
            );
        }
        if let Some(value) = self.maximum_height {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("maximumHeight"),
                &JsValue::from_f64(value),
            );
        }
        if let Some(value) = self.pitch_adjust_height {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("pitchAdjustHeight"),
                &JsValue::from_f64(value),
            );
        }
        if let Some(offset) = self.offset {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("offset"),
                &JsValue::from(offset),
            );
        }
        if let Some(complete) = self.complete {
            let _ = Reflect::set(&options, &JsValue::from_str("complete"), &complete);
        }
        if let Some(cancel) = self.cancel {
            let _ = Reflect::set(&options, &JsValue::from_str("cancel"), &cancel);
        }
        if let Some(easing_function) = self.easing_function {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("easingFunction"),
                &easing_function,
            );
        }

        options.into()
    }
}
