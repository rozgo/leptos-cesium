//! CZML data source component for loading CZML data declaratively

use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{Viewer, czml_data_source_load};
#[cfg(target_arch = "wasm32")]
use crate::components::use_cesium_context;
#[cfg(target_arch = "wasm32")]
use crate::core::{JsStoredValue, OwnedSlot, RequestGate};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

/// CZML data source component for declaratively loading CZML data
///
/// This component loads CZML data from a URL and adds it to the viewer's data sources.
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
    /// URL to the CZML file
    #[prop(into)]
    url: Signal<String>,
    /// Whether to eagerly remove this component's currently tracked data source before loading (default: true).
    /// If false, the previous data source is kept until the new one loads successfully.
    #[prop(optional, into, default = true.into())]
    clear_existing: Signal<bool>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CzmlDataSource must be inside ViewerContainer");
        let loaded_data_source = JsStoredValue::new_local(OwnedSlot::<JsValue>::default());
        let request_gate = RequestGate::new();
        let request_gate_effect = request_gate.clone();

        Effect::new(move |_| {
            let url = url.get();
            let should_clear = clear_existing.get();
            let next_request = request_gate_effect.begin_request();

            viewer_context.with_viewer(|viewer: Viewer| {
                // Remove only the data source previously owned by this component.
                if should_clear {
                    loaded_data_source.update_value(|owned| {
                        owned.clear_with(|existing| {
                            let _ = viewer.data_sources().remove(existing);
                        });
                    });
                }

                // Load CZML data
                let promise = czml_data_source_load(&url);
                let add_promise = viewer.data_sources().add(promise);

                // Handle the promise
                let viewer_ctx_clone = viewer_context;
                let request_gate = request_gate_effect.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match JsFuture::from(add_promise).await {
                        Ok(data_source_js) => {
                            let stale = request_gate.is_stale(next_request);
                            web_sys::console::log_1(&JsValue::from_str(&format!(
                                "Successfully loaded CZML from {}",
                                url
                            )));

                            // Set the viewer's clock to the data source's clock to start animation
                            use crate::bindings::CzmlDataSource;
                            use js_sys::Reflect;
                            use wasm_bindgen::JsCast;

                            viewer_ctx_clone.with_viewer(|v: Viewer| {
                                if stale {
                                    let _ = v.data_sources().remove(&data_source_js);
                                    return;
                                }

                                loaded_data_source.update_value(|owned| {
                                    owned.replace_with(data_source_js.clone(), |existing| {
                                        let _ = v.data_sources().remove(existing);
                                    });
                                });

                                if let Ok(data_source) = data_source_js.dyn_into::<CzmlDataSource>()
                                {
                                    let ds_clock = data_source.clock();
                                    let _ = Reflect::set(
                                        &v,
                                        &JsValue::from_str("clock"),
                                        &JsValue::from(ds_clock),
                                    );
                                    // Ensure animation is enabled
                                    v.clock().set_should_animate(true);
                                }
                            });
                        }
                        Err(e) => {
                            web_sys::console::error_1(&JsValue::from_str(&format!(
                                "Failed to load CZML: {:?}",
                                e
                            )));
                        }
                    }
                });
            });
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
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (url, clear_existing);
    }
}
