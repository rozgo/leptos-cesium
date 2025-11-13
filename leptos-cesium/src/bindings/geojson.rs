//! GeoJSON data source bindings and load options builder

use crate::bindings::Color;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    /// GeoJSON data source for loading GeoJSON and TopoJSON data
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = GeoJsonDataSource)]
    pub type GeoJsonDataSource;

    #[wasm_bindgen(method, getter, js_name = name)]
    pub fn name(this: &GeoJsonDataSource) -> String;

    #[wasm_bindgen(method, setter, js_name = name)]
    pub fn set_name(this: &GeoJsonDataSource, name: &str);

    #[wasm_bindgen(method, getter, js_name = show)]
    pub fn show(this: &GeoJsonDataSource) -> bool;

    #[wasm_bindgen(method, setter, js_name = show)]
    pub fn set_show(this: &GeoJsonDataSource, show: bool);

    #[wasm_bindgen(method, getter, js_name = entities)]
    pub fn entities(this: &GeoJsonDataSource) -> crate::bindings::EntityCollection;
}

/// Helper to call GeoJsonDataSource.load() using reflection
#[cfg(target_arch = "wasm32")]
pub fn geojson_data_source_load(url: &str) -> js_sys::Promise {
    use js_sys::{Function, Reflect, global};
    use wasm_bindgen::JsCast;

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium"))
        .expect("Cesium global to be available");
    let geojson_data_source = Reflect::get(&cesium, &JsValue::from_str("GeoJsonDataSource"))
        .expect("Cesium.GeoJsonDataSource to exist");
    let load_fn = Reflect::get(&geojson_data_source, &JsValue::from_str("load"))
        .expect("Cesium.GeoJsonDataSource.load to exist");
    let load_fn: Function = load_fn
        .dyn_into()
        .expect("Cesium.GeoJsonDataSource.load to be callable");

    load_fn
        .call1(&geojson_data_source, &JsValue::from_str(url))
        .expect("Cesium.GeoJsonDataSource.load to succeed")
        .unchecked_into::<js_sys::Promise>()
}

/// Helper to call GeoJsonDataSource.load() with options
#[cfg(target_arch = "wasm32")]
pub fn geojson_data_source_load_with_options(url: &str, options: &JsValue) -> js_sys::Promise {
    use js_sys::{Function, Reflect, global};
    use wasm_bindgen::JsCast;

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium"))
        .expect("Cesium global to be available");
    let geojson_data_source = Reflect::get(&cesium, &JsValue::from_str("GeoJsonDataSource"))
        .expect("Cesium.GeoJsonDataSource to exist");
    let load_fn = Reflect::get(&geojson_data_source, &JsValue::from_str("load"))
        .expect("Cesium.GeoJsonDataSource.load to exist");
    let load_fn: Function = load_fn
        .dyn_into()
        .expect("Cesium.GeoJsonDataSource.load to be callable");

    load_fn
        .call2(&geojson_data_source, &JsValue::from_str(url), options)
        .expect("Cesium.GeoJsonDataSource.load to succeed")
        .unchecked_into::<js_sys::Promise>()
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct GeoJsonDataSource;

#[cfg(not(target_arch = "wasm32"))]
impl GeoJsonDataSource {
    pub fn name(&self) -> String {
        String::new()
    }
    pub fn set_name(&self, _name: &str) {}
    pub fn show(&self) -> bool {
        true
    }
    pub fn set_show(&self, _show: bool) {}
}

/// Builder for creating GeoJsonDataSource.LoadOptions with a fluent API
///
/// This builder provides a type-safe Rust interface for configuring how GeoJSON
/// data is styled when loaded into Cesium.
///
/// # Example
///
/// ```rust,ignore
/// let options = GeoJsonLoadOptions::new()
///     .stroke(Color::blue())
///     .stroke_width(3.0)
///     .fill(Color::red().with_alpha(0.5))
///     .marker_color(Color::green())
///     .marker_size(64.0)
///     .clamp_to_ground(true)
///     .build();
/// ```
#[derive(Default)]
pub struct GeoJsonLoadOptions {
    stroke: Option<Color>,
    stroke_width: Option<f64>,
    fill: Option<Color>,
    marker_color: Option<Color>,
    marker_size: Option<f64>,
    marker_symbol: Option<String>,
    clamp_to_ground: Option<bool>,
    credit: Option<String>,
}

impl GeoJsonLoadOptions {
    /// Create a new GeoJsonLoadOptions builder with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the stroke color for polylines and polygon outlines (default: Cesium.Color.BLACK)
    pub fn stroke(mut self, color: Color) -> Self {
        self.stroke = Some(color);
        self
    }

    /// Set the stroke width for polylines and polygon outlines (default: 2.0)
    pub fn stroke_width(mut self, width: f64) -> Self {
        self.stroke_width = Some(width);
        self
    }

    /// Set the fill color for polygons (default: Cesium.Color.YELLOW)
    pub fn fill(mut self, color: Color) -> Self {
        self.fill = Some(color);
        self
    }

    /// Set the marker color for point features (default: Cesium.Color.ROYALBLUE)
    pub fn marker_color(mut self, color: Color) -> Self {
        self.marker_color = Some(color);
        self
    }

    /// Set the marker size for point features in pixels (default: 48)
    pub fn marker_size(mut self, size: f64) -> Self {
        self.marker_size = Some(size);
        self
    }

    /// Set the marker symbol for point features (Maki identifier or single character)
    pub fn marker_symbol(mut self, symbol: impl Into<String>) -> Self {
        self.marker_symbol = Some(symbol.into());
        self
    }

    /// Set whether to clamp features to the ground (default: false)
    pub fn clamp_to_ground(mut self, clamp: bool) -> Self {
        self.clamp_to_ground = Some(clamp);
        self
    }

    /// Set the credit/attribution for the data
    pub fn credit(mut self, credit: impl Into<String>) -> Self {
        self.credit = Some(credit.into());
        self
    }

    /// Build the options object for use with GeoJsonDataSource.load()
    #[cfg(target_arch = "wasm32")]
    pub fn build(self) -> JsValue {
        use js_sys::{Object, Reflect};

        let options = Object::new();

        if let Some(color) = self.stroke {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("stroke"),
                &JsValue::from(color),
            );
        }
        if let Some(width) = self.stroke_width {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("strokeWidth"),
                &JsValue::from_f64(width),
            );
        }
        if let Some(color) = self.fill {
            let _ = Reflect::set(&options, &JsValue::from_str("fill"), &JsValue::from(color));
        }
        if let Some(color) = self.marker_color {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("markerColor"),
                &JsValue::from(color),
            );
        }
        if let Some(size) = self.marker_size {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("markerSize"),
                &JsValue::from_f64(size),
            );
        }
        if let Some(symbol) = self.marker_symbol {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("markerSymbol"),
                &JsValue::from_str(&symbol),
            );
        }
        if let Some(clamp) = self.clamp_to_ground {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("clampToGround"),
                &JsValue::from_bool(clamp),
            );
        }
        if let Some(credit) = self.credit {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("credit"),
                &JsValue::from_str(&credit),
            );
        }

        options.into()
    }

    /// Build the options object (SSR stub)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn build(self) -> () {
        ()
    }
}
