//! Cesium ImageMaterialProperty bindings and media source helpers.

use palette::Srgba;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{Cartesian2, Color};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Media source accepted by billboard images and image material properties.
#[derive(Clone)]
pub enum MediaSource {
    Url(String),
    DataUrl(String),
    #[cfg(target_arch = "wasm32")]
    HtmlImage(web_sys::HtmlImageElement),
    #[cfg(target_arch = "wasm32")]
    HtmlCanvas(web_sys::HtmlCanvasElement),
    #[cfg(target_arch = "wasm32")]
    HtmlVideo(web_sys::HtmlVideoElement),
}

#[cfg(target_arch = "wasm32")]
impl MediaSource {
    pub fn to_js_value(&self) -> JsValue {
        match self {
            MediaSource::Url(value) => JsValue::from_str(value),
            MediaSource::DataUrl(value) => JsValue::from_str(value),
            MediaSource::HtmlImage(value) => JsValue::from(value.clone()),
            MediaSource::HtmlCanvas(value) => JsValue::from(value.clone()),
            MediaSource::HtmlVideo(value) => JsValue::from(value.clone()),
        }
    }

    pub fn is_dom_backed(&self) -> bool {
        matches!(
            self,
            MediaSource::HtmlImage(_) | MediaSource::HtmlCanvas(_) | MediaSource::HtmlVideo(_)
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl MediaSource {
    pub fn is_dom_backed(&self) -> bool {
        false
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium)]
    pub type ImageMaterialProperty;

    #[wasm_bindgen(constructor, js_namespace = Cesium)]
    pub fn new(options: &JsValue) -> ImageMaterialProperty;
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct ImageMaterialProperty;

#[cfg(not(target_arch = "wasm32"))]
impl ImageMaterialProperty {
    pub fn new(_options: &()) -> Self {
        Self
    }
}

/// Fluent builder for `ImageMaterialProperty`.
#[derive(Default)]
pub struct ImageMaterialPropertyBuilder {
    image: Option<MediaSource>,
    repeat: Option<(f64, f64)>,
    color: Option<Srgba<f32>>,
    transparent: Option<bool>,
}

impl ImageMaterialPropertyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn image(mut self, image: MediaSource) -> Self {
        self.image = Some(image);
        self
    }

    pub fn repeat(mut self, repeat: (f64, f64)) -> Self {
        self.repeat = Some(repeat);
        self
    }

    pub fn color(mut self, color: Srgba<f32>) -> Self {
        self.color = Some(color);
        self
    }

    pub fn transparent(mut self, transparent: bool) -> Self {
        self.transparent = Some(transparent);
        self
    }

    #[cfg(target_arch = "wasm32")]
    pub fn build(self) -> ImageMaterialProperty {
        use js_sys::{Object, Reflect};

        let options = Object::new();

        if let Some(image) = self.image {
            let _ = Reflect::set(&options, &JsValue::from_str("image"), &image.to_js_value());
        }

        if let Some((x, y)) = self.repeat {
            let repeat = Cartesian2::new(x, y);
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("repeat"),
                &JsValue::from(repeat),
            );
        }

        if let Some(color) = self.color {
            let color: Color = color.into();
            let _ = Reflect::set(&options, &JsValue::from_str("color"), &JsValue::from(color));
        }

        if let Some(transparent) = self.transparent {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("transparent"),
                &JsValue::from_bool(transparent),
            );
        }

        ImageMaterialProperty::new(&options.into())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn build(self) -> ImageMaterialProperty {
        let _ = self;
        ImageMaterialProperty
    }
}
