//! Minimal bindings for Cesium entities and collections.

use crate::bindings::{PositionProperty, Property, PropertyBag};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Cesium, js_name = Entity)]
    pub type Entity;

    #[wasm_bindgen(constructor, js_namespace = Cesium, js_class = Entity)]
    pub fn new(options: &JsValue) -> Entity;

    /// Gets the unique ID associated with this object
    #[wasm_bindgen(method, getter, js_name = id)]
    pub fn id(this: &Entity) -> String;

    /// Gets or sets the name of the object (plain string, not a Property)
    #[wasm_bindgen(method, getter, js_name = name)]
    pub fn name(this: &Entity) -> Option<String>;

    /// Gets or sets the description (Property type)
    #[wasm_bindgen(method, getter, js_name = description)]
    pub fn description(this: &Entity) -> Option<Property>;

    /// Gets or sets the position (PositionProperty type)
    #[wasm_bindgen(method, getter, js_name = position)]
    pub fn position(this: &Entity) -> Option<PositionProperty>;

    /// Gets or sets the bag of arbitrary properties associated with this entity
    #[wasm_bindgen(method, getter, js_name = properties)]
    pub fn properties(this: &Entity) -> Option<PropertyBag>;

    /// Gets or sets whether this entity should be displayed
    #[wasm_bindgen(method, getter, js_name = show)]
    pub fn show(this: &Entity) -> bool;

    #[wasm_bindgen(js_namespace = Cesium, js_name = EntityCollection)]
    pub type EntityCollection;

    #[wasm_bindgen(method, js_name = add)]
    pub fn add_with_options(this: &EntityCollection, entity: &JsValue) -> Entity;

    #[wasm_bindgen(method, js_name = remove)]
    pub fn remove(this: &EntityCollection, entity: &Entity) -> bool;

    #[wasm_bindgen(method, js_name = removeAll)]
    pub fn remove_all(this: &EntityCollection);
}
