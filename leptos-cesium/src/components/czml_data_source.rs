//! CZML data source component for loading CZML data declaratively

use leptos::prelude::*;
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
use std::collections::VecDeque;

use super::czml_overlay_media::{CzmlMediaError, CzmlMediaResolver};
#[cfg(target_arch = "wasm32")]
use super::czml_overlay_media::{
    CzmlOverlayBinding, CzmlOverlayMedia, reconcile_data_source_overlay_media,
};
#[cfg(target_arch = "wasm32")]
use super::overlay::{
    TrackedEntityImageOverlay, TrackedEntityVideoOverlay, TrackedEntityYouTubeOverlay,
};
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

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
struct PendingAppendRequest {
    czml_input: JsValue,
    options_js: Option<JsValue>,
    media_base_uri: Option<String>,
    media_overlays: bool,
}

/// CZML data source component for declaratively loading CZML data
///
/// This component loads CZML data and adds it to the viewer's data sources.
/// When the URL changes, the previous data source owned by this component can be removed
/// before the new one is loaded.
/// If packets include flattened `properties.media_*` metadata, this component can automatically
/// render tracked DOM overlays for supported media kinds using each entity's `position`.
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
    /// Optional trigger for imperative-style reprocessing (for example, replaying the same payload).
    #[prop(optional, into, default = ().into())]
    trigger: Signal<()>,
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
    /// Whether `properties.media_*` overlay media should be rendered from `entity.position`.
    #[prop(optional, into, default = true.into())]
    media_overlays: Signal<bool>,
    /// Optional custom media resolver.
    #[prop(optional)]
    resolve_media: Option<CzmlMediaResolver>,
    /// Called with loading state transitions.
    #[prop(optional)]
    on_loading: Option<Callback<bool>>,
    /// Called with media reconciliation state transitions.
    #[prop(optional)]
    on_media_loading: Option<Callback<bool>>,
    /// Called with load or runtime error messages.
    #[prop(optional)]
    on_error: Option<Callback<String>>,
    /// Called with media parse/reconciliation errors.
    #[prop(optional)]
    on_media_error: Option<Callback<CzmlMediaError>>,
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
        let append_queue = JsRwSignal::new_local(VecDeque::<PendingAppendRequest>::new());
        let append_worker_running = JsRwSignal::new_local(false);
        let overlay_bindings = JsRwSignal::new_local(Vec::<CzmlOverlayBinding>::new());
        let request_gate = RequestGate::new();
        let request_gate_effect = request_gate.clone();

        Effect::new(move |_| {
            trigger.get();

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
            let media_overlays_enabled = media_overlays.get();
            let media_base_uri =
                derive_media_base_uri(source.clone(), url.clone(), source_uri_value.clone());
            let on_loading_callback = on_loading;
            let on_media_loading_callback = on_media_loading;
            let on_error_callback = on_error;
            let on_media_error_callback = on_media_error;
            let on_changed_callback = on_changed;
            let on_loaded_callback = on_loaded;
            let resolve_media_callback = resolve_media;

            let czml_input = match resolve_czml_input(source, url, data) {
                Ok(Some(value)) => value,
                Ok(None) => {
                    let _ = request_gate_effect.begin_request();
                    append_queue.update(|queue| queue.clear());

                    if should_clear {
                        overlay_bindings.set(Vec::new());
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
                let existing_ds = current_data_source
                    .try_get_untracked()
                    .flatten()
                    .and_then(|value| value.dyn_into::<CesiumCzmlDataSource>().ok());

                // Keep append mode ordered when writing into an existing data source.
                // Cesium process() is async; queueing avoids out-of-order completion races.
                if mode == CzmlLoadMode::Append && existing_ds.as_ref().is_some() {
                    let _ = append_queue.try_update(|queue| {
                        queue.push_back(PendingAppendRequest {
                            czml_input: czml_input.clone(),
                            options_js: options_js.clone(),
                            media_base_uri: media_base_uri.clone(),
                            media_overlays: media_overlays_enabled,
                        });
                    });

                    if append_worker_running.try_get_untracked().unwrap_or(false) {
                        return;
                    }

                    let _ = append_worker_running.try_set(true);
                    let worker_request = request_gate_effect.begin_request();
                    if let Some(callback) = on_loading_callback {
                        callback.run(true);
                    }

                    let append_queue_worker = append_queue;
                    let append_worker_running_worker = append_worker_running;
                    let current_data_source_worker = current_data_source;
                    let request_gate_worker = request_gate_effect.clone();
                    let overlay_bindings_worker = overlay_bindings;

                    wasm_bindgen_futures::spawn_local(async move {
                        loop {
                            if request_gate_worker.is_stale(worker_request) {
                                break;
                            }

                            let mut next_request = None;
                            if append_queue_worker
                                .try_update(|queue| {
                                    next_request = queue.pop_front();
                                })
                                .is_none()
                            {
                                break;
                            }
                            let Some(request) = next_request else {
                                break;
                            };

                            let Some(data_source) = current_data_source_worker
                                .try_get_untracked()
                                .flatten()
                                .and_then(|value| value.dyn_into::<CesiumCzmlDataSource>().ok())
                            else {
                                break;
                            };

                            let promise = match request.options_js.as_ref() {
                                Some(options) => {
                                    data_source.process_with_options(&request.czml_input, options)
                                }
                                None => data_source.process(&request.czml_input),
                            };

                            match JsFuture::from(promise).await {
                                Ok(data_source_js) => {
                                    if request_gate_worker.is_stale(worker_request) {
                                        break;
                                    }

                                    reconcile_media_overlays_if_enabled(
                                        request.media_overlays,
                                        &data_source_js,
                                        worker_request,
                                        request_gate_worker.clone(),
                                        overlay_bindings_worker,
                                        resolve_media_callback,
                                        on_media_loading_callback,
                                        on_media_error_callback,
                                        request.media_base_uri.clone(),
                                    );
                                    if let Some(callback) = on_loaded_callback {
                                        callback.run(data_source_js);
                                    }
                                }
                                Err(e) => {
                                    if request_gate_worker.is_stale(worker_request) {
                                        break;
                                    }

                                    let message = js_error_to_string(&e);
                                    if let Some(callback) = on_error_callback {
                                        callback.run(message.clone());
                                    } else {
                                        web_sys::console::error_1(&JsValue::from_str(&format!(
                                            "Failed to process CZML append packet: {}",
                                            message
                                        )));
                                    }
                                }
                            }
                        }

                        if !request_gate_worker.is_stale(worker_request)
                            && let Some(callback) = on_loading_callback
                        {
                            callback.run(false);
                        }
                        let _ = append_worker_running_worker.try_set(false);
                    });

                    return;
                }

                let next_request = request_gate_effect.begin_request();
                let _ = append_queue.try_update(|queue| queue.clear());

                if let Some(callback) = on_loading_callback {
                    callback.run(true);
                }

                // Remove only the data source previously owned by this component.
                if should_clear && mode == CzmlLoadMode::Replace {
                    let _ = overlay_bindings.try_set(Vec::new());
                    let _ = loaded_data_source.try_update_value(|owned| {
                        owned.clear_with(|existing| {
                            let _ = viewer.data_sources().remove(existing);
                        });
                    });
                    let _ = current_data_source.try_set(None);
                    detach_listener(changed_listener);
                    detach_listener_2(error_listener);
                    detach_listener_2(loading_listener);
                }

                let existing_ds = current_data_source
                    .try_get_untracked()
                    .flatten()
                    .and_then(|value| value.dyn_into::<CesiumCzmlDataSource>().ok());

                // Replace mode on an existing source should call CzmlDataSource.load(),
                // not process(), to preserve Cesium semantics.
                let replacing_existing = mode == CzmlLoadMode::Replace && existing_ds.is_some();
                let promise = if replacing_existing {
                    let existing_ds = existing_ds.expect("checked above");
                    match options_js.as_ref() {
                        Some(options) => existing_ds.load_with_options(&czml_input, options),
                        None => existing_ds.load(&czml_input),
                    }
                } else {
                    let load_promise = match options_js.as_ref() {
                        Some(options) => czml_data_source_load_with_options(&czml_input, options),
                        None => czml_data_source_load(&czml_input),
                    };
                    viewer.data_sources().add(load_promise)
                };
                let created_new_data_source = !replacing_existing;

                // Handle the promise
                let viewer_ctx_clone = viewer_context;
                let request_gate = request_gate_effect.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match JsFuture::from(promise).await {
                        Ok(data_source_js) => {
                            if request_gate.is_stale(next_request) {
                                if created_new_data_source {
                                    let _ = viewer_ctx_clone.with_viewer(|v: Viewer| {
                                        let _ = v.data_sources().remove(&data_source_js);
                                    });
                                }
                                return;
                            }

                            if let Some(callback) = on_loading_callback {
                                callback.run(false);
                            }

                            let Some(v) = viewer_ctx_clone.viewer_untracked() else {
                                return;
                            };

                            if let Ok(data_source) =
                                data_source_js.clone().dyn_into::<CesiumCzmlDataSource>()
                            {
                                let _ = current_data_source.try_set(Some(data_source_js.clone()));
                                if created_new_data_source {
                                    let _ = loaded_data_source.try_update_value(|owned| {
                                        owned.replace_with(
                                            data_source_js.clone(),
                                            |existing| {
                                                let _ = v.data_sources().remove(existing);
                                            },
                                        );
                                    });
                                }

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

                                reconcile_media_overlays_if_enabled(
                                    media_overlays_enabled,
                                    &data_source_js,
                                    next_request,
                                    request_gate.clone(),
                                    overlay_bindings,
                                    resolve_media_callback,
                                    on_media_loading_callback,
                                    on_media_error_callback,
                                    media_base_uri.clone(),
                                );

                                if let Some(callback) = on_loaded_callback {
                                    callback.run(data_source_js.clone());
                                }
                            }

                            web_sys::console::log_1(&JsValue::from_str(
                                "Successfully loaded CZML data source",
                            ));
                        }
                        Err(e) => {
                            if request_gate.is_stale(next_request) {
                                return;
                            }

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
            let _ = append_queue.try_update(|queue| queue.clear());
            let _ = append_worker_running.try_set(false);
            let _ = overlay_bindings.try_set(Vec::new());

            viewer_context.with_viewer(|viewer: Viewer| {
                let _ = loaded_data_source.try_update_value(|owned| {
                    owned.clear_with(|existing| {
                        let _ = viewer.data_sources().remove(existing);
                    });
                });
            });
            let _ = current_data_source.try_set(None);
            detach_listener(changed_listener);
            detach_listener_2(error_listener);
            detach_listener_2(loading_listener);
        });

        view! {
            <For
                each=move || overlay_bindings.get()
                key=|binding| binding.entity_id.clone()
                let:binding
            >
                {move || {
                    let entity = binding.entity.clone();

                    match binding.media.clone() {
                        CzmlOverlayMedia::Image {
                            src,
                            width_px,
                            height_px,
                            cross_origin,
                        } => {
                            view! {
                                <TrackedEntityImageOverlay
                                    entity=entity
                                    show=show
                                    src=src
                                    width_px=width_px
                                    height_px=height_px
                                    cross_origin=cross_origin
                                />
                            }
                                .into_any()
                        }
                        CzmlOverlayMedia::Video {
                            src,
                            width_px,
                            height_px,
                            autoplay,
                            loop_video,
                            muted,
                            plays_inline,
                            controls,
                            cross_origin,
                            poster,
                            preload,
                        } => {
                            view! {
                                <TrackedEntityVideoOverlay
                                    entity=entity
                                    show=show
                                    src=src
                                    width_px=width_px
                                    height_px=height_px
                                    autoplay=autoplay
                                    loop_video=loop_video
                                    muted=muted
                                    plays_inline=plays_inline
                                    controls=controls
                                    cross_origin=cross_origin
                                    poster=poster
                                    preload=preload
                                />
                            }
                                .into_any()
                        }
                        CzmlOverlayMedia::Youtube {
                            video_id,
                            width_px,
                            height_px,
                            autoplay,
                            mute,
                            controls,
                            start_seconds,
                        } => {
                            view! {
                                <TrackedEntityYouTubeOverlay
                                    entity=entity
                                    show=show
                                    video_id=video_id
                                    width_px=width_px
                                    height_px=height_px
                                    autoplay=autoplay
                                    mute=mute
                                    controls=controls
                                    start_seconds=start_seconds
                                />
                            }
                                .into_any()
                        }
                    }
                }}
            </For>
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            source,
            url,
            data,
            mode,
            trigger,
            clear_existing,
            show,
            name,
            source_uri,
            credit,
            clustering,
            media_overlays,
            resolve_media,
            on_loading,
            on_media_loading,
            on_error,
            on_media_error,
            on_changed,
            on_loaded,
        );

        ().into_view()
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
fn derive_media_base_uri(
    source: Option<CzmlSource>,
    url: Option<String>,
    source_uri: Option<String>,
) -> Option<String> {
    if let Some(source_uri) = source_uri.filter(|value| !value.is_empty()) {
        return Some(source_uri);
    }

    match source {
        Some(CzmlSource::Url(value)) if !value.is_empty() => Some(value),
        Some(CzmlSource::Url(_)) => None,
        Some(CzmlSource::JsonString(_)) => None,
        None => url.filter(|value| !value.is_empty()),
    }
}

#[cfg(target_arch = "wasm32")]
fn reconcile_media_overlays_if_enabled(
    media_overlays: bool,
    data_source_js: &JsValue,
    request_id: u64,
    request_gate: RequestGate,
    overlay_bindings: JsRwSignal<Vec<CzmlOverlayBinding>>,
    resolve_media: Option<CzmlMediaResolver>,
    on_media_loading: Option<Callback<bool>>,
    on_media_error: Option<Callback<CzmlMediaError>>,
    media_base_uri: Option<String>,
) {
    if !media_overlays {
        overlay_bindings.set(Vec::new());
        return;
    }

    if let Some(callback) = on_media_loading {
        callback.run(true);
    }

    let bindings = reconcile_data_source_overlay_media(
        data_source_js.clone(),
        request_id,
        request_gate,
        resolve_media,
        on_media_error,
        media_base_uri,
    );
    if let Some(bindings) = bindings {
        overlay_bindings.set(bindings);
    }

    if let Some(callback) = on_media_loading {
        callback.run(false);
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
