//! Minimal Cesium viewer bindings needed to bootstrap rendering.

use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::bindings::JulianDate;
use crate::bindings::data_source::{DataSource, DataSourceCollection};
use crate::bindings::entity::{Entity, EntityCollection};

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

    /// Selected entity property (get/set)
    #[wasm_bindgen(method, getter, js_name = selectedEntity)]
    pub fn selected_entity(this: &Viewer) -> Option<Entity>;

    #[wasm_bindgen(method, setter, js_name = selectedEntity)]
    pub fn set_selected_entity(this: &Viewer, entity: Option<&Entity>);

    /// Event fired when the selected entity changes
    #[wasm_bindgen(method, getter, js_name = selectedEntityChanged)]
    pub fn selected_entity_changed(this: &Viewer) -> Event;

    /// Tracked entity property (get/set) - the entity currently being tracked by the camera
    #[wasm_bindgen(method, getter, js_name = trackedEntity)]
    pub fn tracked_entity(this: &Viewer) -> Option<Entity>;

    #[wasm_bindgen(method, setter, js_name = trackedEntity)]
    pub fn set_tracked_entity(this: &Viewer, entity: Option<&Entity>);

    /// Event fired when the tracked entity changes
    #[wasm_bindgen(method, getter, js_name = trackedEntityChanged)]
    pub fn tracked_entity_changed(this: &Viewer) -> Event;

    /// Gets or sets whether data sources can suspend animation while assets stream.
    #[wasm_bindgen(method, getter, js_name = allowDataSourcesToSuspendAnimation)]
    pub fn allow_data_sources_to_suspend_animation(this: &Viewer) -> bool;

    #[wasm_bindgen(method, setter, js_name = allowDataSourcesToSuspendAnimation)]
    pub fn set_allow_data_sources_to_suspend_animation(this: &Viewer, value: bool);

    /// Gets or sets the data source tracked by the viewer clock.
    #[wasm_bindgen(method, getter, js_name = clockTrackedDataSource)]
    pub fn clock_tracked_data_source(this: &Viewer) -> Option<DataSource>;

    #[wasm_bindgen(method, setter, js_name = clockTrackedDataSource)]
    pub fn set_clock_tracked_data_source(this: &Viewer, value: Option<&DataSource>);

    /// Cesium Event type for event handling
    #[wasm_bindgen(js_namespace = Cesium, js_name = Event)]
    pub type Event;

    #[wasm_bindgen(method, js_name = addEventListener)]
    pub fn add_event_listener(this: &Event, listener: &js_sys::Function);

    #[wasm_bindgen(method, js_name = removeEventListener)]
    pub fn remove_event_listener(this: &Event, listener: &js_sys::Function);

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

    #[wasm_bindgen(method, getter, js_name = canAnimate)]
    pub fn can_animate(this: &Clock) -> bool;

    #[wasm_bindgen(method, setter, js_name = canAnimate)]
    pub fn set_can_animate(this: &Clock, value: bool);

    #[wasm_bindgen(method, getter, js_name = currentTime)]
    pub fn current_time(this: &Clock) -> JulianDate;

    #[wasm_bindgen(method, setter, js_name = currentTime)]
    pub fn set_current_time(this: &Clock, value: &JulianDate);

    #[wasm_bindgen(method, getter, js_name = startTime)]
    pub fn start_time(this: &Clock) -> JulianDate;

    #[wasm_bindgen(method, setter, js_name = startTime)]
    pub fn set_start_time(this: &Clock, value: &JulianDate);

    #[wasm_bindgen(method, getter, js_name = stopTime)]
    pub fn stop_time(this: &Clock) -> JulianDate;

    #[wasm_bindgen(method, setter, js_name = stopTime)]
    pub fn set_stop_time(this: &Clock, value: &JulianDate);

    #[wasm_bindgen(method, getter, js_name = multiplier)]
    pub fn multiplier(this: &Clock) -> f64;

    #[wasm_bindgen(method, setter, js_name = multiplier)]
    pub fn set_multiplier(this: &Clock, value: f64);

    #[wasm_bindgen(method, getter, js_name = clockRange)]
    pub fn clock_range(this: &Clock) -> i32;

    #[wasm_bindgen(method, setter, js_name = clockRange)]
    pub fn set_clock_range(this: &Clock, value: i32);

    #[wasm_bindgen(method, getter, js_name = clockStep)]
    pub fn clock_step(this: &Clock) -> i32;

    #[wasm_bindgen(method, setter, js_name = clockStep)]
    pub fn set_clock_step(this: &Clock, value: i32);

    #[wasm_bindgen(method, getter, js_name = onTick)]
    pub fn on_tick(this: &Clock) -> Event;

    #[wasm_bindgen(method, getter, js_name = onStop)]
    pub fn on_stop(this: &Clock) -> Event;

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

impl Viewer {
    /// Clears the tracked entity (convenience method)
    pub fn clear_tracked_entity(&self) {
        self.set_tracked_entity(None);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ClockRangeMode {
    #[default]
    Unbounded = 0,
    Clamped = 1,
    LoopStop = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ClockStepMode {
    #[default]
    TickDependent = 0,
    SystemClockMultiplier = 1,
    SystemClock = 2,
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
