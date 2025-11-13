//! Minimal Cesium viewer bindings needed to bootstrap rendering.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlElement;

#[cfg(target_arch = "wasm32")]
use crate::bindings::data_source::DataSourceCollection;
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

    #[wasm_bindgen(method, getter, js_name = dataSources)]
    pub fn data_sources(this: &Viewer) -> DataSourceCollection;

    #[wasm_bindgen(method, getter, js_name = camera)]
    pub fn camera(this: &Viewer) -> Camera;

    #[wasm_bindgen(method, getter, js_name = clock)]
    pub fn clock(this: &Viewer) -> Clock;

    #[wasm_bindgen(method, getter, js_name = scene)]
    pub fn scene(this: &Viewer) -> Scene;

    #[wasm_bindgen(method, js_name = zoomTo)]
    pub fn zoom_to(this: &Viewer, target: &JsValue) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = zoomTo)]
    pub fn zoom_to_with_offset(
        this: &Viewer,
        target: &JsValue,
        offset: &JsValue,
    ) -> js_sys::Promise;

    /// Camera for controlling the view
    #[wasm_bindgen(js_namespace = Cesium, js_name = Camera)]
    pub type Camera;

    #[wasm_bindgen(method, js_name = flyHome)]
    pub fn fly_home(this: &Camera, duration: f64);

    #[wasm_bindgen(method, js_name = flyTo)]
    pub fn fly_to(this: &Camera, options: &JsValue);

    #[wasm_bindgen(method, js_name = setView)]
    pub fn set_view(this: &Camera, options: &JsValue);

    /// Clock for controlling time and animation
    #[wasm_bindgen(js_namespace = Cesium, js_name = Clock)]
    pub type Clock;

    #[wasm_bindgen(method, getter, js_name = shouldAnimate)]
    pub fn should_animate(this: &Clock) -> bool;

    #[wasm_bindgen(method, setter, js_name = shouldAnimate)]
    pub fn set_should_animate(this: &Clock, value: bool);

    #[wasm_bindgen(method, getter, js_name = currentTime)]
    pub fn current_time(this: &Clock) -> JsValue;

    #[wasm_bindgen(method, setter, js_name = currentTime)]
    pub fn set_current_time(this: &Clock, value: &JsValue);

    /// JulianDate for time representation
    #[wasm_bindgen(js_namespace = Cesium, js_name = JulianDate)]
    pub type JulianDate;

    /// Scene contains the primitives and other visual elements
    #[wasm_bindgen(js_namespace = Cesium, js_name = Scene)]
    pub type Scene;

    #[wasm_bindgen(method, getter, js_name = primitives)]
    pub fn primitives(this: &Scene) -> PrimitiveCollection;

    /// Collection of primitives in the scene
    #[wasm_bindgen(js_namespace = Cesium, js_name = PrimitiveCollection)]
    pub type PrimitiveCollection;

    #[wasm_bindgen(method, js_name = add)]
    pub fn add(this: &PrimitiveCollection, primitive: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = remove)]
    pub fn remove(this: &PrimitiveCollection, primitive: &JsValue) -> bool;

    #[wasm_bindgen(method, js_name = removeAll)]
    pub fn remove_all(this: &PrimitiveCollection);
}

// Helper function to get current JulianDate using reflection
#[cfg(target_arch = "wasm32")]
pub fn julian_date_now() -> JulianDate {
    use js_sys::{Function, Reflect, global};
    use wasm_bindgen::{JsCast, JsValue};

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium"))
        .expect("Cesium global to be available");
    let julian_date_class = Reflect::get(&cesium, &JsValue::from_str("JulianDate"))
        .expect("Cesium.JulianDate to exist");
    let now_fn = Reflect::get(&julian_date_class, &JsValue::from_str("now"))
        .expect("Cesium.JulianDate.now to exist");
    let now_fn: Function = now_fn
        .dyn_into()
        .expect("Cesium.JulianDate.now to be callable");

    now_fn
        .call0(&julian_date_class)
        .expect("Cesium.JulianDate.now call to succeed")
        .unchecked_into::<JulianDate>()
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
