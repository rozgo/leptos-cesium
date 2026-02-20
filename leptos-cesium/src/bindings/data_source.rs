//! Cesium DataSource bindings for CZML and other data formats.

use wasm_bindgen::prelude::*;

use crate::bindings::{EntityCollection, JulianDate, viewer::Event};

#[wasm_bindgen]
extern "C" {
    /// DataSource interface type.
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = DataSource)]
    pub type DataSource;

    /// Credit displayed in Cesium's credit area.
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = Credit)]
    pub type Credit;

    /// Clustering options for data sources.
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = EntityCluster)]
    pub type EntityCluster;

    /// Collection of DataSource instances.
    #[wasm_bindgen(js_namespace = Cesium, js_name = DataSourceCollection)]
    pub type DataSourceCollection;

    /// Adds a data source promise to the collection.
    #[wasm_bindgen(method, js_name = add)]
    pub fn add(this: &DataSourceCollection, data_source: js_sys::Promise) -> js_sys::Promise;

    /// Adds a concrete data source instance to the collection.
    #[wasm_bindgen(method, js_name = add)]
    pub fn add_value(this: &DataSourceCollection, data_source: &JsValue) -> js_sys::Promise;

    /// Removes a data source from the collection.
    #[wasm_bindgen(method, js_name = remove)]
    pub fn remove(this: &DataSourceCollection, data_source: &JsValue) -> bool;

    /// Removes a data source from the collection and optionally destroys it.
    #[wasm_bindgen(method, js_name = remove)]
    pub fn remove_with_destroy(
        this: &DataSourceCollection,
        data_source: &JsValue,
        destroy: bool,
    ) -> bool;

    /// Removes all data sources.
    #[wasm_bindgen(method, js_name = removeAll)]
    pub fn remove_all(this: &DataSourceCollection);

    /// Removes all data sources and optionally destroys them.
    #[wasm_bindgen(method, js_name = removeAll)]
    pub fn remove_all_with_destroy(this: &DataSourceCollection, destroy: bool);

    /// Number of data sources in this collection.
    #[wasm_bindgen(method, getter, js_name = length)]
    pub fn length(this: &DataSourceCollection) -> u32;

    /// Returns true if this collection contains the data source.
    #[wasm_bindgen(method, js_name = contains)]
    pub fn contains(this: &DataSourceCollection, data_source: &JsValue) -> bool;

    /// Returns index of a data source, or -1.
    #[wasm_bindgen(method, js_name = indexOf)]
    pub fn index_of(this: &DataSourceCollection, data_source: &JsValue) -> i32;

    /// Returns data source by index.
    #[wasm_bindgen(method, js_name = get)]
    pub fn get(this: &DataSourceCollection, index: u32) -> JsValue;

    /// Returns data sources by name.
    #[wasm_bindgen(method, js_name = getByName)]
    pub fn get_by_name(this: &DataSourceCollection, name: &str) -> js_sys::Array;

    /// Raises a data source one position.
    #[wasm_bindgen(method, js_name = raise)]
    pub fn raise(this: &DataSourceCollection, data_source: &JsValue);

    /// Lowers a data source one position.
    #[wasm_bindgen(method, js_name = lower)]
    pub fn lower(this: &DataSourceCollection, data_source: &JsValue);

    /// Raises a data source to top.
    #[wasm_bindgen(method, js_name = raiseToTop)]
    pub fn raise_to_top(this: &DataSourceCollection, data_source: &JsValue);

    /// Lowers a data source to bottom.
    #[wasm_bindgen(method, js_name = lowerToBottom)]
    pub fn lower_to_bottom(this: &DataSourceCollection, data_source: &JsValue);

    /// Event raised when a data source is added.
    #[wasm_bindgen(method, getter, js_name = dataSourceAdded)]
    pub fn data_source_added(this: &DataSourceCollection) -> Event;

    /// Event raised when a data source is removed.
    #[wasm_bindgen(method, getter, js_name = dataSourceRemoved)]
    pub fn data_source_removed(this: &DataSourceCollection) -> Event;

    /// Event raised when a data source moves within the collection.
    #[wasm_bindgen(method, getter, js_name = dataSourceMoved)]
    pub fn data_source_moved(this: &DataSourceCollection) -> Event;

    /// CZML data source.
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = CzmlDataSource)]
    pub type CzmlDataSource;

    #[wasm_bindgen(constructor, js_namespace = Cesium, js_class = CzmlDataSource)]
    pub fn new(name: &str) -> CzmlDataSource;

    /// Human-readable name.
    #[wasm_bindgen(method, getter, js_name = name)]
    pub fn name(this: &CzmlDataSource) -> String;

    #[wasm_bindgen(method, setter, js_name = name)]
    pub fn set_name(this: &CzmlDataSource, value: &str);

    /// Visibility of all entities in this data source.
    #[wasm_bindgen(method, getter, js_name = show)]
    pub fn show(this: &CzmlDataSource) -> bool;

    #[wasm_bindgen(method, setter, js_name = show)]
    pub fn set_show(this: &CzmlDataSource, value: bool);

    /// Clock settings from CZML (undefined when static data only).
    #[wasm_bindgen(method, getter, js_name = clock)]
    pub fn clock(this: &CzmlDataSource) -> Option<DataSourceClock>;

    /// Entity collection loaded by CZML.
    #[wasm_bindgen(method, getter, js_name = entities)]
    pub fn entities(this: &CzmlDataSource) -> EntityCollection;

    /// Current loading state.
    #[wasm_bindgen(method, getter, js_name = isLoading)]
    pub fn is_loading(this: &CzmlDataSource) -> bool;

    /// Changed event.
    #[wasm_bindgen(method, getter, js_name = changedEvent)]
    pub fn changed_event(this: &CzmlDataSource) -> Event;

    /// Error event.
    #[wasm_bindgen(method, getter, js_name = errorEvent)]
    pub fn error_event(this: &CzmlDataSource) -> Event;

    /// Loading event.
    #[wasm_bindgen(method, getter, js_name = loadingEvent)]
    pub fn loading_event(this: &CzmlDataSource) -> Event;

    /// Clustering options.
    #[wasm_bindgen(method, getter, js_name = clustering)]
    pub fn clustering(this: &CzmlDataSource) -> EntityCluster;

    #[wasm_bindgen(method, setter, js_name = clustering)]
    pub fn set_clustering(this: &CzmlDataSource, value: &EntityCluster);

    /// Credit displayed for this data source.
    #[wasm_bindgen(method, getter, js_name = credit)]
    pub fn credit(this: &CzmlDataSource) -> Credit;

    /// Load CZML replacing existing entities.
    #[wasm_bindgen(method, js_name = load)]
    pub fn load(this: &CzmlDataSource, czml: &JsValue) -> js_sys::Promise;

    /// Load CZML replacing existing entities with options.
    #[wasm_bindgen(method, js_name = load)]
    pub fn load_with_options(
        this: &CzmlDataSource,
        czml: &JsValue,
        options: &JsValue,
    ) -> js_sys::Promise;

    /// Process CZML appending to existing entities.
    #[wasm_bindgen(method, js_name = process)]
    pub fn process(this: &CzmlDataSource, czml: &JsValue) -> js_sys::Promise;

    /// Process CZML appending to existing entities with options.
    #[wasm_bindgen(method, js_name = process)]
    pub fn process_with_options(
        this: &CzmlDataSource,
        czml: &JsValue,
        options: &JsValue,
    ) -> js_sys::Promise;

    /// Update this data source for simulation time.
    #[wasm_bindgen(method, js_name = update)]
    pub fn update(this: &CzmlDataSource, time: &JulianDate) -> bool;

    /// DataSource clock that defines the time range.
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = DataSourceClock)]
    pub type DataSourceClock;

    #[wasm_bindgen(method, getter, js_name = definitionChanged)]
    pub fn definition_changed(this: &DataSourceClock) -> Event;

    #[wasm_bindgen(method, getter, js_name = startTime)]
    pub fn start_time(this: &DataSourceClock) -> JulianDate;

    #[wasm_bindgen(method, setter, js_name = startTime)]
    pub fn set_start_time(this: &DataSourceClock, value: &JulianDate);

    #[wasm_bindgen(method, getter, js_name = stopTime)]
    pub fn stop_time(this: &DataSourceClock) -> JulianDate;

    #[wasm_bindgen(method, setter, js_name = stopTime)]
    pub fn set_stop_time(this: &DataSourceClock, value: &JulianDate);

    #[wasm_bindgen(method, getter, js_name = currentTime)]
    pub fn current_time(this: &DataSourceClock) -> JulianDate;

    #[wasm_bindgen(method, setter, js_name = currentTime)]
    pub fn set_current_time(this: &DataSourceClock, value: &JulianDate);

    #[wasm_bindgen(method, getter, js_name = clockRange)]
    pub fn clock_range(this: &DataSourceClock) -> i32;

    #[wasm_bindgen(method, setter, js_name = clockRange)]
    pub fn set_clock_range(this: &DataSourceClock, value: i32);

    #[wasm_bindgen(method, getter, js_name = clockStep)]
    pub fn clock_step(this: &DataSourceClock) -> i32;

    #[wasm_bindgen(method, setter, js_name = clockStep)]
    pub fn set_clock_step(this: &DataSourceClock, value: i32);

    #[wasm_bindgen(method, getter, js_name = multiplier)]
    pub fn multiplier(this: &DataSourceClock) -> f64;

    #[wasm_bindgen(method, setter, js_name = multiplier)]
    pub fn set_multiplier(this: &DataSourceClock, value: f64);

    #[wasm_bindgen(method, js_name = getValue)]
    pub fn get_value(this: &DataSourceClock) -> crate::bindings::Clock;
}

#[derive(Default)]
pub struct CzmlLoadOptions {
    source_uri: Option<String>,
    credit: Option<String>,
}

impl CzmlLoadOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn source_uri(mut self, source_uri: impl Into<String>) -> Self {
        self.source_uri = Some(source_uri.into());
        self
    }

    pub fn credit(mut self, credit: impl Into<String>) -> Self {
        self.credit = Some(credit.into());
        self
    }

    pub fn is_empty(&self) -> bool {
        self.source_uri.is_none() && self.credit.is_none()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn build(self) -> JsValue {
        use js_sys::{Object, Reflect};

        let options = Object::new();

        if let Some(source_uri) = self.source_uri {
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("sourceUri"),
                &JsValue::from_str(&source_uri),
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
}

#[cfg(target_arch = "wasm32")]
pub fn parse_czml_json(input: &str) -> Result<JsValue, JsValue> {
    js_sys::JSON::parse(input)
}

#[cfg(target_arch = "wasm32")]
fn czml_data_source_static_call(
    method: &str,
    czml: &JsValue,
    options: Option<&JsValue>,
) -> js_sys::Promise {
    use js_sys::{Function, Reflect, global};
    use wasm_bindgen::JsCast;

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium"))
        .expect("Cesium global to be available");
    let czml_data_source = Reflect::get(&cesium, &JsValue::from_str("CzmlDataSource"))
        .expect("Cesium.CzmlDataSource to exist");
    let method_fn = Reflect::get(&czml_data_source, &JsValue::from_str(method))
        .expect("Cesium.CzmlDataSource method to exist");
    let method_fn: Function = method_fn
        .dyn_into()
        .expect("Cesium.CzmlDataSource method to be callable");

    let promise = match options {
        Some(options) => method_fn
            .call2(&czml_data_source, czml, options)
            .expect("Cesium.CzmlDataSource static call to succeed"),
        None => method_fn
            .call1(&czml_data_source, czml)
            .expect("Cesium.CzmlDataSource static call to succeed"),
    };

    promise.unchecked_into::<js_sys::Promise>()
}

/// Helper to call `CzmlDataSource.load(czml)` using reflection.
#[cfg(target_arch = "wasm32")]
pub fn czml_data_source_load(czml: &JsValue) -> js_sys::Promise {
    czml_data_source_static_call("load", czml, None)
}

/// Helper to call `CzmlDataSource.load(czml, options)` using reflection.
#[cfg(target_arch = "wasm32")]
pub fn czml_data_source_load_with_options(czml: &JsValue, options: &JsValue) -> js_sys::Promise {
    czml_data_source_static_call("load", czml, Some(options))
}
