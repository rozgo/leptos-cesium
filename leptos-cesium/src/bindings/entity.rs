//! Minimal bindings for Cesium entities and collections.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Cesium, js_name = Entity)]
    pub type Entity;

    #[wasm_bindgen(constructor, js_namespace = Cesium, js_class = Entity)]
    pub fn new(options: &JsValue) -> Entity;

    /// Entity name property
    #[wasm_bindgen(method, getter, js_name = name)]
    pub fn name(this: &Entity) -> JsValue;

    /// Entity description property
    #[wasm_bindgen(method, getter, js_name = description)]
    pub fn description(this: &Entity) -> JsValue;

    /// Entity id property
    #[wasm_bindgen(method, getter, js_name = id)]
    pub fn id(this: &Entity) -> String;

    /// Entity position property
    #[wasm_bindgen(method, getter, js_name = position)]
    pub fn position(this: &Entity) -> JsValue;

    /// Entity properties (custom properties bag)
    #[wasm_bindgen(method, getter, js_name = properties)]
    pub fn properties(this: &Entity) -> JsValue;

    #[wasm_bindgen(js_namespace = Cesium, js_name = EntityCollection)]
    pub type EntityCollection;

    #[wasm_bindgen(method, js_name = add)]
    pub fn add_with_options(this: &EntityCollection, entity: &JsValue) -> Entity;

    #[wasm_bindgen(method, js_name = remove)]
    pub fn remove(this: &EntityCollection, entity: &Entity) -> bool;

    #[wasm_bindgen(method, js_name = removeAll)]
    pub fn remove_all(this: &EntityCollection);
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct Entity;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct EntityCollection;

#[cfg(not(target_arch = "wasm32"))]
impl EntityCollection {
    #[allow(clippy::unused_io_amount)]
    pub fn add_with_options(&self, _entity: &wasm_bindgen::JsValue) -> Entity {
        Entity
    }

    pub fn remove(&self, _entity: &Entity) -> bool {
        false
    }

    pub fn remove_all(&self) {}
}
