//! Minimal Cesium viewer bindings needed to bootstrap rendering.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlElement;

#[cfg(target_arch = "wasm32")]
use crate::bindings::entity::EntityCollection;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Cesium, js_name = Viewer)]
    pub type Viewer;

    #[wasm_bindgen(constructor, js_namespace = Cesium, js_class = Viewer)]
    pub fn new(container: &HtmlElement, options: &JsValue) -> Viewer;

    #[wasm_bindgen(method, js_name = destroy)]
    pub fn destroy(this: &Viewer) -> bool;

    #[wasm_bindgen(method, getter, js_name = entities)]
    pub fn entities(this: &Viewer) -> EntityCollection;

    #[wasm_bindgen(method, js_name = zoomTo)]
    pub fn zoom_to(this: &Viewer, target: &JsValue) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = zoomTo)]
    pub fn zoom_to_with_offset(
        this: &Viewer,
        target: &JsValue,
        offset: &JsValue,
    ) -> js_sys::Promise;
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct Viewer;

#[cfg(not(target_arch = "wasm32"))]
impl Viewer {
    #[allow(dead_code)]
    pub fn entities(&self) -> crate::bindings::entity::EntityCollection {
        crate::bindings::entity::EntityCollection
    }
}
