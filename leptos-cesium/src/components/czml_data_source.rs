//! CZML data source component for loading CZML data declaratively

use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{Viewer, czml_data_source_load};
#[cfg(target_arch = "wasm32")]
use crate::components::use_cesium_context;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

/// CZML data source component for declaratively loading CZML data
///
/// This component loads CZML data from a URL and adds it to the viewer's data sources.
/// When the URL changes, the previous data source is removed and the new one is loaded.
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
    /// Whether to remove all existing data sources before loading (default: true)
    #[prop(optional, into, default = true.into())]
    clear_existing: Signal<bool>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CzmlDataSource must be inside ViewerContainer");

        Effect::new(move |_| {
            let url = url.get();
            let should_clear = clear_existing.get();

            viewer_context.with_viewer(|viewer: Viewer| {
                // Clear existing data sources if requested
                if should_clear {
                    viewer.data_sources().remove_all();
                }

                // Load CZML data
                let promise = czml_data_source_load(&url);
                let add_promise = viewer.data_sources().add(promise);

                // Handle the promise
                let viewer_ctx_clone = viewer_context;
                wasm_bindgen_futures::spawn_local(async move {
                    match JsFuture::from(add_promise).await {
                        Ok(data_source_js) => {
                            web_sys::console::log_1(&JsValue::from_str(&format!(
                                "Successfully loaded CZML from {}",
                                url
                            )));

                            // Set the viewer's clock to the data source's clock to start animation
                            use crate::bindings::CzmlDataSource;
                            use js_sys::Reflect;
                            use wasm_bindgen::JsCast;

                            if let Ok(data_source) = data_source_js.dyn_into::<CzmlDataSource>() {
                                let ds_clock = data_source.clock();
                                viewer_ctx_clone.with_viewer(|v: Viewer| {
                                    let _ = Reflect::set(
                                        &v,
                                        &JsValue::from_str("clock"),
                                        &JsValue::from(ds_clock),
                                    );
                                    // Ensure animation is enabled
                                    v.clock().set_should_animate(true);
                                });
                            }
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
            // Clear data sources when component unmounts
            if let Some(viewer_ctx) = use_cesium_context() {
                viewer_ctx.with_viewer(|viewer: Viewer| {
                    viewer.data_sources().remove_all();
                });
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (url, clear_existing);
    }
}
