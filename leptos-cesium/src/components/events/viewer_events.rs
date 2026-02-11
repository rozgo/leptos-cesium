use crate::bindings::Viewer;
use crate::cesium_events;
use wasm_bindgen::JsValue;

cesium_events!(
    (ViewerEvents, Viewer),
    (selected_entity_changed, selected_entity_changed, JsValue),
    (tracked_entity_changed, tracked_entity_changed, JsValue),
);
