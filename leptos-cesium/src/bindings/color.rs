//! Cesium Color utilities

use wasm_bindgen::prelude::*;

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

impl Color {
    #[cfg(target_arch = "wasm32")]
    pub fn white() -> Color {
        get_color_property("WHITE")
    }

    #[cfg(target_arch = "wasm32")]
    pub fn black() -> Color {
        get_color_property("BLACK")
    }

    #[cfg(target_arch = "wasm32")]
    pub fn red() -> Color {
        get_color_property("RED")
    }

    #[cfg(target_arch = "wasm32")]
    pub fn green() -> Color {
        get_color_property("GREEN")
    }

    #[cfg(target_arch = "wasm32")]
    pub fn blue() -> Color {
        get_color_property("BLUE")
    }

    #[cfg(target_arch = "wasm32")]
    pub fn yellow() -> Color {
        get_color_property("YELLOW")
    }

    #[cfg(target_arch = "wasm32")]
    pub fn cyan() -> Color {
        get_color_property("CYAN")
    }

    #[cfg(target_arch = "wasm32")]
    pub fn magenta() -> Color {
        get_color_property("MAGENTA")
    }

    #[cfg(target_arch = "wasm32")]
    pub fn gray() -> Color {
        get_color_property("GRAY")
    }

    #[cfg(target_arch = "wasm32")]
    pub fn lightgray() -> Color {
        get_color_property("LIGHTGRAY")
    }

    #[cfg(target_arch = "wasm32")]
    pub fn deepskyblue() -> Color {
        get_color_property("DEEPSKYBLUE")
    }

    #[cfg(target_arch = "wasm32")]
    pub fn purple() -> Color {
        get_color_property("PURPLE")
    }
}
