//! Cesium Property types for time-dynamic values

use crate::bindings::Cartesian3;
use wasm_bindgen::prelude::*;

/// Base Property interface - all Cesium properties implement this
#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium)]
    pub type Property;

    /// Gets a value indicating if this property is constant
    #[wasm_bindgen(method, getter, js_name = isConstant)]
    pub fn is_constant(this: &Property) -> bool;

    /// Gets the value of the property at the provided time
    #[wasm_bindgen(method, js_name = getValue)]
    pub fn get_value(this: &Property, time: Option<&JulianDate>) -> JsValue;

    /// Compares this property to another
    #[wasm_bindgen(method, js_name = equals)]
    pub fn equals(this: &Property, other: &Property) -> bool;
}

/// PositionProperty - a Property that returns Cartesian3 positions
#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium)]
    pub type PositionProperty;

    /// Gets a value indicating if this property is constant
    #[wasm_bindgen(method, getter, js_name = isConstant)]
    pub fn is_constant(this: &PositionProperty) -> bool;

    /// Gets the value of the property at the provided time in the fixed frame
    #[wasm_bindgen(method, js_name = getValue)]
    pub fn get_value(this: &PositionProperty, time: Option<&JulianDate>) -> Option<Cartesian3>;

    /// Gets the value in a specific reference frame
    #[wasm_bindgen(method, js_name = getValueInReferenceFrame)]
    pub fn get_value_in_reference_frame(
        this: &PositionProperty,
        time: &JulianDate,
        reference_frame: &ReferenceFrame,
    ) -> Option<Cartesian3>;

    /// Compares this property to another
    #[wasm_bindgen(method, js_name = equals)]
    pub fn equals(this: &PositionProperty, other: &Property) -> bool;
}

/// JulianDate - Cesium's time representation
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Cesium)]
    pub type JulianDate;

    /// Get the current system time as a JulianDate
    #[wasm_bindgen(static_method_of = JulianDate, js_name = now)]
    pub fn now() -> JulianDate;
}

/// ReferenceFrame - coordinate reference frames
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Cesium)]
    pub type ReferenceFrame;
}

/// PropertyBag - a dynamic collection of properties
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Cesium)]
    pub type PropertyBag;
}

impl Property {
    /// Get the current value of the property (using current time)
    pub fn value(&self) -> JsValue {
        self.get_value(None)
    }

    /// Get the value as a string if possible
    pub fn as_string(&self) -> Option<String> {
        self.value().as_string()
    }
}

impl PositionProperty {
    /// Get the current position (using current time)
    pub fn value(&self) -> Option<Cartesian3> {
        self.get_value(None)
    }
}
