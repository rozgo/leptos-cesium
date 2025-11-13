//! Cesium Color utilities

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium)]
    pub type Color;

    #[wasm_bindgen(constructor, js_namespace = Cesium)]
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Color;

    #[wasm_bindgen(static_method_of = Color, js_name = fromRandom)]
    pub fn from_random(options: &JsValue) -> Color;

    #[wasm_bindgen(method, js_name = withAlpha)]
    pub fn with_alpha(this: &Color, alpha: f64) -> Color;
}

// Helper to get static color properties
#[cfg(target_arch = "wasm32")]
fn get_color_property(name: &str) -> Color {
    use js_sys::{Reflect, global};
    use wasm_bindgen::JsCast;

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium"))
        .expect("Cesium global to be available");
    let color_class =
        Reflect::get(&cesium, &JsValue::from_str("Color")).expect("Cesium.Color to exist");
    Reflect::get(&color_class, &JsValue::from_str(name))
        .unwrap_or_else(|_| panic!("Cesium.Color.{} to exist", name))
        .unchecked_into::<Color>()
}

#[cfg(target_arch = "wasm32")]
impl Color {
    pub fn white() -> Color {
        get_color_property("WHITE")
    }

    pub fn black() -> Color {
        get_color_property("BLACK")
    }

    pub fn red() -> Color {
        get_color_property("RED")
    }

    pub fn green() -> Color {
        get_color_property("GREEN")
    }

    pub fn blue() -> Color {
        get_color_property("BLUE")
    }

    pub fn yellow() -> Color {
        get_color_property("YELLOW")
    }

    pub fn cyan() -> Color {
        get_color_property("CYAN")
    }

    pub fn magenta() -> Color {
        get_color_property("MAGENTA")
    }

    pub fn gray() -> Color {
        get_color_property("GRAY")
    }

    pub fn lightgray() -> Color {
        get_color_property("LIGHTGRAY")
    }

    pub fn deepskyblue() -> Color {
        get_color_property("DEEPSKYBLUE")
    }

    pub fn purple() -> Color {
        get_color_property("PURPLE")
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct Color;

#[cfg(not(target_arch = "wasm32"))]
impl Color {
    pub fn new(_red: f64, _green: f64, _blue: f64, _alpha: f64) -> Self {
        Color
    }

    pub fn from_random(_options: &()) -> Self {
        Color
    }

    pub fn with_alpha(&self, _alpha: f64) -> Self {
        Color
    }

    pub fn white() -> Self {
        Color
    }
    pub fn black() -> Self {
        Color
    }
    pub fn red() -> Self {
        Color
    }
    pub fn green() -> Self {
        Color
    }
    pub fn blue() -> Self {
        Color
    }
    pub fn yellow() -> Self {
        Color
    }
    pub fn cyan() -> Self {
        Color
    }
    pub fn magenta() -> Self {
        Color
    }
    pub fn gray() -> Self {
        Color
    }
    pub fn lightgray() -> Self {
        Color
    }
    pub fn deepskyblue() -> Self {
        Color
    }
    pub fn purple() -> Self {
        Color
    }
}
