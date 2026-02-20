//! CZML data source component for loading CZML data declaratively

use leptos::prelude::*;
use wasm_bindgen::JsValue;

use crate::bindings::EntityCluster;
#[cfg(target_arch = "wasm32")]
use crate::bindings::{
    CzmlDataSource as CesiumCzmlDataSource, CzmlLoadOptions, Event, Viewer, czml_data_source_load,
    czml_data_source_load_with_options, parse_czml_json,
};
#[cfg(target_arch = "wasm32")]
use crate::components::use_cesium_context;
use crate::core::JsSignal;
#[cfg(target_arch = "wasm32")]
use crate::core::{JsRwSignal, JsStoredValue, OwnedSlot, RequestGate};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, closure::Closure};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

/// How incoming CZML should be applied to the target data source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CzmlLoadMode {
    /// Replace existing data (`CzmlDataSource.load`).
    #[default]
    Replace,
    /// Append data (`CzmlDataSource.process`).
    Append,
}

/// Source selector for CZML input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CzmlSource {
    /// Fetch CZML from a URL.
    Url(String),
    /// Parse JSON text as inline CZML.
    JsonString(String),
}

impl From<&str> for CzmlSource {
    fn from(value: &str) -> Self {
        Self::Url(value.to_string())
    }
}

impl From<String> for CzmlSource {
    fn from(value: String) -> Self {
        Self::Url(value)
    }
}

/// CZML data source component for declaratively loading CZML data
///
/// This component loads CZML data and adds it to the viewer's data sources.
/// When the URL changes, the previous data source owned by this component can be removed
/// before the new one is loaded.
///
/// # Example
///
/// ```rust,ignore
/// view! {
///     <ViewerContainer ion_token=token>
///         <CzmlDataSource url="SampleData/simple.czml" />
///     </ViewerContainer>
/// }
/// ```
#[component(transparent)]
pub fn CzmlDataSource(
    /// Source selector for CZML input. If set, this takes precedence over `url` and `data`.
    #[prop(optional, into)]
    source: Signal<Option<CzmlSource>>,
    /// URL to the CZML file.
    #[prop(optional, into)]
    url: Signal<Option<String>>,
    /// Inline JSON string containing CZML packets.
    #[prop(optional, into)]
    data: Signal<Option<String>>,
    /// Whether to use replace (`load`) or append (`process`) mode.
    #[prop(optional, into, default = CzmlLoadMode::Replace.into())]
    mode: Signal<CzmlLoadMode>,
    /// Whether to eagerly remove this component's currently tracked data source before loading (default: true).
    /// If false, the previous data source is kept until the new one loads successfully.
    #[prop(optional, into, default = true.into())]
    clear_existing: Signal<bool>,
    /// Whether this data source is visible after loading.
    #[prop(optional, into, default = true.into())]
    show: Signal<bool>,
    /// Optional data source name override.
    #[prop(optional, into)]
    name: Signal<Option<String>>,
    /// Optional source URI used to resolve relative links in CZML.
    #[prop(optional, into)]
    source_uri: Signal<Option<String>>,
    /// Optional credit text for this data source.
    #[prop(optional, into)]
    credit: Signal<Option<String>>,
    /// Optional clustering configuration.
    #[prop(optional, into)]
    clustering: JsSignal<Option<EntityCluster>>,
    /// Called with loading state transitions.
    #[prop(optional)]
    on_loading: Option<Callback<bool>>,
    /// Called with load or runtime error messages.
    #[prop(optional)]
    on_error: Option<Callback<String>>,
    /// Called when the data source emits changed events.
    #[prop(optional)]
    on_changed: Option<Callback<JsValue>>,
    /// Called when a load/process request completes successfully.
    #[prop(optional)]
    on_loaded: Option<Callback<JsValue>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CzmlDataSource must be inside ViewerContainer");
        let loaded_data_source = JsStoredValue::new_local(OwnedSlot::<JsValue>::default());
        let current_data_source = JsRwSignal::new_local(None::<JsValue>);
        let changed_listener =
            JsStoredValue::new_local(OwnedSlot::<(Event, Closure<dyn FnMut(JsValue)>)>::default());
        let error_listener = JsStoredValue::new_local(OwnedSlot::<(
            Event,
            Closure<dyn FnMut(JsValue, JsValue)>,
        )>::default());
        let loading_listener = JsStoredValue::new_local(OwnedSlot::<(
            Event,
            Closure<dyn FnMut(JsValue, JsValue)>,
        )>::default());
        let request_gate = RequestGate::new();
        let request_gate_effect = request_gate.clone();

        Effect::new(move |_| {
            let source = source.get();
            let url = url.get();
            let data = data.get();
            let mode = mode.get();
            let should_clear = clear_existing.get();
            let show_value = show.get();
            let name_value = name.get();
            let source_uri_value = source_uri.get();
            let credit_value = credit.get();
            let clustering_value = clustering.get_untracked();
            let next_request = request_gate_effect.begin_request();
            let on_loading_callback = on_loading;
            let on_error_callback = on_error;
            let on_changed_callback = on_changed;
            let on_loaded_callback = on_loaded;

            let czml_input = match resolve_czml_input(source, url, data) {
                Ok(Some(value)) => value,
                Ok(None) => {
                    if should_clear {
                        viewer_context.with_viewer(|viewer: Viewer| {
                            loaded_data_source.update_value(|owned| {
                                owned.clear_with(|existing| {
                                    let _ = viewer.data_sources().remove(existing);
                                });
                            });
                            current_data_source.set(None);
                            detach_listener(changed_listener);
                            detach_listener_2(error_listener);
                            detach_listener_2(loading_listener);
                        });
                    }
                    return;
                }
                Err(message) => {
                    if let Some(callback) = on_error_callback {
                        callback.run(message.clone());
                    } else {
                        web_sys::console::error_1(&JsValue::from_str(&message));
                    }
                    return;
                }
            };

            let mut options_builder = CzmlLoadOptions::new();
            if let Some(source_uri) = source_uri_value {
                options_builder = options_builder.source_uri(source_uri);
            }
            if let Some(credit) = credit_value {
                options_builder = options_builder.credit(credit);
            }
            let options_js = (!options_builder.is_empty()).then(|| options_builder.build());

            viewer_context.with_viewer(|viewer: Viewer| {
                if let Some(callback) = on_loading_callback {
                    callback.run(true);
                }

                // Remove only the data source previously owned by this component.
                if should_clear && mode == CzmlLoadMode::Replace {
                    loaded_data_source.update_value(|owned| {
                        owned.clear_with(|existing| {
                            let _ = viewer.data_sources().remove(existing);
                        });
                    });
                    current_data_source.set(None);
                    detach_listener(changed_listener);
                    detach_listener_2(error_listener);
                    detach_listener_2(loading_listener);
                }

                let existing_ds = current_data_source
                    .get_untracked()
                    .and_then(|value| value.dyn_into::<CesiumCzmlDataSource>().ok());
                let appending_to_existing =
                    mode == CzmlLoadMode::Append && existing_ds.as_ref().is_some();

                let promise = if let Some(existing_ds) = existing_ds {
                    match options_js.as_ref() {
                        Some(options) => existing_ds.process_with_options(&czml_input, options),
                        None => existing_ds.process(&czml_input),
                    }
                } else {
                    let load_promise = match options_js.as_ref() {
                        Some(options) => czml_data_source_load_with_options(&czml_input, options),
                        None => czml_data_source_load(&czml_input),
                    };
                    viewer.data_sources().add(load_promise)
                };

                // Handle the promise
                let viewer_ctx_clone = viewer_context;
                let request_gate = request_gate_effect.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match JsFuture::from(promise).await {
                        Ok(data_source_js) => {
                            let stale = request_gate.is_stale(next_request);
                            if let Some(callback) = on_loaded_callback {
                                callback.run(data_source_js.clone());
                            }
                            if let Some(callback) = on_loading_callback {
                                callback.run(false);
                            }

                            viewer_ctx_clone.with_viewer(|v: Viewer| {
                                if stale {
                                    if !appending_to_existing {
                                        let _ = v.data_sources().remove(&data_source_js);
                                    }
                                    return;
                                }

                                if !appending_to_existing {
                                    loaded_data_source.update_value(|owned| {
                                        owned.replace_with(data_source_js.clone(), |existing| {
                                            let _ = v.data_sources().remove(existing);
                                        });
                                    });
                                    current_data_source.set(Some(data_source_js.clone()));
                                }

                                if let Ok(data_source) =
                                    data_source_js.clone().dyn_into::<CesiumCzmlDataSource>()
                                {
                                    apply_czml_properties(
                                        &data_source,
                                        show_value,
                                        name_value.clone(),
                                        clustering_value.clone(),
                                    );

                                    attach_changed_listener(
                                        &data_source,
                                        on_changed_callback,
                                        changed_listener,
                                    );
                                    attach_error_listener(
                                        &data_source,
                                        on_error_callback,
                                        error_listener,
                                    );
                                    attach_loading_listener(
                                        &data_source,
                                        on_loading_callback,
                                        loading_listener,
                                    );
                                }
                            });

                            web_sys::console::log_1(&JsValue::from_str(
                                "Successfully loaded CZML data source",
                            ));
                        }
                        Err(e) => {
                            if let Some(callback) = on_loading_callback {
                                callback.run(false);
                            }

                            let message = js_error_to_string(&e);
                            if let Some(callback) = on_error_callback {
                                callback.run(message.clone());
                            } else {
                                web_sys::console::error_1(&JsValue::from_str(&format!(
                                    "Failed to load CZML: {}",
                                    message
                                )));
                            }
                        }
                    }
                });
            });
        });

        // Keep visible/name/clustering in sync on prop changes without forcing reload.
        Effect::new(move |_| {
            let show_value = show.get();
            let name_value = name.get();
            let clustering_value = clustering.get_untracked();

            if let Some(data_source_js) = current_data_source.get()
                && let Ok(data_source) = data_source_js.dyn_into::<CesiumCzmlDataSource>()
            {
                apply_czml_properties(&data_source, show_value, name_value, clustering_value);
            }
        });

        on_cleanup(move || {
            request_gate.close();

            viewer_context.with_viewer(|viewer: Viewer| {
                loaded_data_source.update_value(|owned| {
                    owned.clear_with(|existing| {
                        let _ = viewer.data_sources().remove(existing);
                    });
                });
            });
            current_data_source.set(None);
            detach_listener(changed_listener);
            detach_listener_2(error_listener);
            detach_listener_2(loading_listener);
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            source,
            url,
            data,
            mode,
            clear_existing,
            show,
            name,
            source_uri,
            credit,
            clustering,
            on_loading,
            on_error,
            on_changed,
            on_loaded,
        );
    }
}

#[cfg(target_arch = "wasm32")]
fn resolve_czml_input(
    source: Option<CzmlSource>,
    url: Option<String>,
    data: Option<String>,
) -> Result<Option<JsValue>, String> {
    match source {
        Some(CzmlSource::Url(value)) => Ok(Some(JsValue::from_str(&value))),
        Some(CzmlSource::JsonString(value)) => parse_czml_json(&value)
            .map(Some)
            .map_err(|err| format!("Invalid inline CZML JSON: {}", js_error_to_string(&err))),
        None => {
            if let Some(url) = url {
                Ok(Some(JsValue::from_str(&url)))
            } else if let Some(data) = data {
                parse_czml_json(&data).map(Some).map_err(|err| {
                    format!("Invalid inline CZML JSON: {}", js_error_to_string(&err))
                })
            } else {
                Ok(None)
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn apply_czml_properties(
    data_source: &CesiumCzmlDataSource,
    show: bool,
    name: Option<String>,
    clustering: Option<EntityCluster>,
) {
    data_source.set_show(show);
    if let Some(name) = name {
        data_source.set_name(&name);
    }
    if let Some(cluster) = clustering {
        data_source.set_clustering(&cluster);
    }
}

#[cfg(target_arch = "wasm32")]
fn attach_changed_listener(
    data_source: &CesiumCzmlDataSource,
    on_changed: Option<Callback<JsValue>>,
    slot: crate::core::JsStoredValue<OwnedSlot<(Event, Closure<dyn FnMut(JsValue)>)>>,
) {
    detach_listener(slot);
    let Some(on_changed) = on_changed else {
        return;
    };

    let event = data_source.changed_event();
    let callback = Closure::wrap(Box::new(move |value: JsValue| {
        on_changed.run(value);
    }) as Box<dyn FnMut(JsValue)>);

    event.add_event_listener(callback.as_ref().unchecked_ref());
    slot.update_value(|listener_slot| {
        listener_slot.replace_with((event, callback), |(existing_event, existing_callback)| {
            existing_event.remove_event_listener(existing_callback.as_ref().unchecked_ref());
        });
    });
}

#[cfg(target_arch = "wasm32")]
fn attach_error_listener(
    data_source: &CesiumCzmlDataSource,
    on_error: Option<Callback<String>>,
    slot: crate::core::JsStoredValue<OwnedSlot<(Event, Closure<dyn FnMut(JsValue, JsValue)>)>>,
) {
    detach_listener_2(slot);
    let Some(on_error) = on_error else {
        return;
    };

    let event = data_source.error_event();
    let callback = Closure::wrap(Box::new(move |_source: JsValue, error: JsValue| {
        on_error.run(js_error_to_string(&error));
    }) as Box<dyn FnMut(JsValue, JsValue)>);

    event.add_event_listener(callback.as_ref().unchecked_ref());
    slot.update_value(|listener_slot| {
        listener_slot.replace_with((event, callback), |(existing_event, existing_callback)| {
            existing_event.remove_event_listener(existing_callback.as_ref().unchecked_ref());
        });
    });
}

#[cfg(target_arch = "wasm32")]
fn attach_loading_listener(
    data_source: &CesiumCzmlDataSource,
    on_loading: Option<Callback<bool>>,
    slot: crate::core::JsStoredValue<OwnedSlot<(Event, Closure<dyn FnMut(JsValue, JsValue)>)>>,
) {
    detach_listener_2(slot);
    let Some(on_loading) = on_loading else {
        return;
    };

    let event = data_source.loading_event();
    let callback = Closure::wrap(Box::new(move |_source: JsValue, loading: JsValue| {
        on_loading.run(loading.as_bool().unwrap_or(false));
    }) as Box<dyn FnMut(JsValue, JsValue)>);

    event.add_event_listener(callback.as_ref().unchecked_ref());
    slot.update_value(|listener_slot| {
        listener_slot.replace_with((event, callback), |(existing_event, existing_callback)| {
            existing_event.remove_event_listener(existing_callback.as_ref().unchecked_ref());
        });
    });
}

#[cfg(target_arch = "wasm32")]
fn detach_listener(
    slot: crate::core::JsStoredValue<OwnedSlot<(Event, Closure<dyn FnMut(JsValue)>)>>,
) {
    slot.update_value(|listener_slot| {
        listener_slot.clear_with(|(event, callback)| {
            event.remove_event_listener(callback.as_ref().unchecked_ref());
        });
    });
}

#[cfg(target_arch = "wasm32")]
fn detach_listener_2(
    slot: crate::core::JsStoredValue<OwnedSlot<(Event, Closure<dyn FnMut(JsValue, JsValue)>)>>,
) {
    slot.update_value(|listener_slot| {
        listener_slot.clear_with(|(event, callback)| {
            event.remove_event_listener(callback.as_ref().unchecked_ref());
        });
    });
}

#[cfg(target_arch = "wasm32")]
fn js_error_to_string(error: &JsValue) -> String {
    if let Some(message) = error.as_string() {
        return message;
    }
    if let Ok(text) = js_sys::JSON::stringify(error)
        && let Some(value) = text.as_string()
        && !value.is_empty()
    {
        return value;
    }
    format!("{:?}", error)
}
