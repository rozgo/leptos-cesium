//! Cesium3DTileset component for loading 3D tiles.

use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{
    Cesium3DTilesetOptions, GooglePhotorealistic3DTilesApiOptions, Viewer,
    create_google_photorealistic_3d_tileset,
};
#[cfg(target_arch = "wasm32")]
use crate::components::use_cesium_context;
#[cfg(target_arch = "wasm32")]
use crate::core::{JsStoredValue, OwnedSlot, RequestGate};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

/// Component for Google Photorealistic 3D Tiles.
///
/// This uses Cesium's `createGooglePhotorealistic3DTileset()` API.
/// When no Google API key is provided, it falls back to Cesium Ion asset 2275207.
///
/// # Example
///
/// ```rust,ignore
/// // Default: uses Cesium Ion asset (requires Ion token with Google 3D Tiles access)
/// view! {
///     <ViewerContainer ion_token=token>
///         <GooglePhotorealistic3DTiles />
///     </ViewerContainer>
/// }
/// ```
#[component(transparent)]
pub fn GooglePhotorealistic3DTiles(
    /// Optional Google Maps API key. If not provided, uses Cesium Ion asset 2275207
    #[prop(optional, into)]
    google_api_key: Option<Signal<Option<String>>>,
    /// Cache size in bytes. Default: 1536 MB
    #[prop(optional)]
    cache_bytes: Option<u32>,
    /// Maximum cache overflow in bytes. Default: 1024 MB
    #[prop(optional)]
    maximum_cache_overflow_bytes: Option<u32>,
    /// Enable collision detection. Default: true
    #[prop(optional)]
    enable_collision: Option<bool>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context = use_cesium_context()
            .expect("GooglePhotorealistic3DTiles must be inside ViewerContainer");
        let loaded_primitive = JsStoredValue::new_local(OwnedSlot::<JsValue>::default());
        let request_gate = RequestGate::new();
        let request_gate_effect = request_gate.clone();

        Effect::new(move |_| {
            let google_api_key = google_api_key.and_then(|key_signal| key_signal.get());
            let next_request = request_gate_effect.begin_request();

            viewer_context.with_viewer(|viewer: Viewer| {
                loaded_primitive.update_value(|owned| {
                    let scene = viewer.scene();
                    let primitives = scene.primitives();
                    owned.clear_with(|existing| {
                        let _ = primitives.remove(existing);
                    });
                });

                web_sys::console::log_1(&JsValue::from_str(
                    "GooglePhotorealistic3DTiles: loading tileset...",
                ));

                // Build API options
                let mut api_options = GooglePhotorealistic3DTilesApiOptions::default();
                if google_api_key.is_some() {
                    api_options.key = google_api_key.clone();
                }

                // Build tileset options
                let mut tileset_options = Cesium3DTilesetOptions::default();
                if let Some(cache) = cache_bytes {
                    tileset_options.cache_bytes = Some(cache);
                }
                if let Some(overflow) = maximum_cache_overflow_bytes {
                    tileset_options.maximum_cache_overflow_bytes = Some(overflow);
                }
                if let Some(collision) = enable_collision {
                    tileset_options.enable_collision = Some(collision);
                }

                let api_options_js = api_options.to_js_value();
                let tileset_options_js = tileset_options.to_js_value();

                let promise =
                    create_google_photorealistic_3d_tileset(&api_options_js, &tileset_options_js);
                let viewer_ctx = viewer_context;
                let request_gate = request_gate_effect.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    match JsFuture::from(promise).await {
                        Ok(tileset) => {
                            let stale = request_gate.is_stale(next_request);
                            viewer_ctx.with_viewer(|viewer: Viewer| {
                                let scene = viewer.scene();
                                let primitives = scene.primitives();

                                if stale {
                                    let _ = primitives.remove(&tileset);
                                    return;
                                }

                                primitives.add(&tileset);
                                loaded_primitive.update_value(|owned| {
                                    owned.replace_with(tileset.clone(), |existing| {
                                        let _ = primitives.remove(existing);
                                    });
                                });

                                web_sys::console::log_1(&JsValue::from_str(
                                    "GooglePhotorealistic3DTiles: tileset loaded and added to scene",
                                ));
                            });
                        }
                        Err(e) => {
                            web_sys::console::error_2(
                                &JsValue::from_str("GooglePhotorealistic3DTiles: failed to load:"),
                                &e,
                            );
                        }
                    }
                });
            });
        });

        on_cleanup(move || {
            request_gate.close();

            // Remove only the primitive created by this component.
            viewer_context.with_viewer(|viewer: Viewer| {
                let scene = viewer.scene();
                let primitives = scene.primitives();
                loaded_primitive.update_value(|owned| {
                    owned.clear_with(|existing| {
                        let _ = primitives.remove(existing);
                        web_sys::console::log_1(&JsValue::from_str(
                            "GooglePhotorealistic3DTiles: primitive removed from scene",
                        ));
                    });
                });
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            google_api_key,
            cache_bytes,
            maximum_cache_overflow_bytes,
            enable_collision,
        );
    }
}
