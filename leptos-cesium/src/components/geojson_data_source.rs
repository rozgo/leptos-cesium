//! GeoJSON data source component for loading GeoJSON data declaratively

use leptos::prelude::*;

use crate::bindings::Color;
use crate::core::JsSignal;
#[cfg(target_arch = "wasm32")]
use crate::core::{JsStoredValue, OwnedSlot, RequestGate};

#[cfg(target_arch = "wasm32")]
use crate::bindings::GeoJsonLoadOptions;
#[cfg(target_arch = "wasm32")]
use crate::bindings::{Viewer, geojson_data_source_load, geojson_data_source_load_with_options};
#[cfg(target_arch = "wasm32")]
use crate::components::use_cesium_context;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

/// GeoJSON data source component for declaratively loading GeoJSON data
///
/// This component loads GeoJSON or TopoJSON data from a URL and adds it to the viewer's
/// data sources. When the URL changes, the previous data source owned by this component
/// can be removed and the new one is loaded.
///
/// GeoJSON features are automatically converted to Cesium entities. The component supports
/// extensive styling options for polygons, polylines, and point markers.
///
/// # Basic Example
///
/// ```rust,ignore
/// view! {
///     <ViewerContainer ion_token=token>
///         <GeoJsonDataSource url="data/countries.geojson" />
///     </ViewerContainer>
/// }
/// ```
///
/// # Advanced Styling Example
///
/// ```rust,ignore
/// view! {
///     <ViewerContainer ion_token=token>
///         <GeoJsonDataSource
///             url="data/countries.geojson"
///             stroke=Color::blue()
///             stroke_width=3.0
///             fill=Color::red().with_alpha(0.5)
///             marker_color=Color::green()
///             marker_size=64.0
///             clamp_to_ground=true
///         />
///     </ViewerContainer>
/// }
/// ```
///
/// # Reactive URL Example
///
/// ```rust,ignore
/// let (selected, set_selected) = signal("countries.geojson".to_string());
///
/// view! {
///     <select on:change=move |ev| set_selected(event_target_value(&ev))>
///         <option value="countries.geojson">"Countries"</option>
///         <option value="cities.geojson">"Cities"</option>
///     </select>
///
///     <ViewerContainer ion_token=token>
///         <GeoJsonDataSource url=move || format!("data/{}", selected.get()) />
///     </ViewerContainer>
/// }
/// ```
#[component(transparent)]
pub fn GeoJsonDataSource(
    /// URL to the GeoJSON or TopoJSON file
    #[prop(into)]
    url: Signal<String>,

    /// Whether to eagerly remove this component's currently tracked data source before loading (default: true).
    /// If false, the previous data source is kept until the new one loads successfully.
    #[prop(optional, into, default = true.into())]
    clear_existing: Signal<bool>,

    /// Stroke color for polylines and polygon outlines (default: Cesium.Color.BLACK)
    #[prop(optional, into)]
    stroke: JsSignal<Option<Color>>,

    /// Stroke width for polylines and polygon outlines (default: 2.0)
    #[prop(optional, into)]
    stroke_width: Signal<Option<f64>>,

    /// Fill color for polygons (default: Cesium.Color.YELLOW)
    #[prop(optional, into)]
    fill: JsSignal<Option<Color>>,

    /// Marker color for point features (default: Cesium.Color.ROYALBLUE)
    #[prop(optional, into)]
    marker_color: JsSignal<Option<Color>>,

    /// Marker size for point features in pixels (default: 48)
    #[prop(optional, into)]
    marker_size: Signal<Option<f64>>,

    /// Marker symbol for point features (Maki identifier or single character)
    #[prop(optional, into)]
    marker_symbol: Signal<Option<String>>,

    /// Whether to clamp features to the ground (default: false)
    #[prop(optional, into)]
    clamp_to_ground: Signal<Option<bool>>,

    /// Credit/attribution for the data
    #[prop(optional, into)]
    credit: Signal<Option<String>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("GeoJsonDataSource must be inside ViewerContainer");
        let loaded_data_source = JsStoredValue::new_local(OwnedSlot::<JsValue>::default());
        let request_gate = RequestGate::new();
        let request_gate_effect = request_gate.clone();

        Effect::new(move |_| {
            let url = url.get();
            let should_clear = clear_existing.get();
            let stroke = stroke.get();
            let stroke_width = stroke_width.get();
            let fill = fill.get();
            let marker_color = marker_color.get();
            let marker_size = marker_size.get();
            let marker_symbol = marker_symbol.get();
            let clamp_to_ground = clamp_to_ground.get();
            let credit = credit.get();
            let next_request = request_gate_effect.begin_request();

            // Build options if any styling props are provided
            let has_options = stroke.is_some()
                || stroke_width.is_some()
                || fill.is_some()
                || marker_color.is_some()
                || marker_size.is_some()
                || marker_symbol.is_some()
                || clamp_to_ground.is_some()
                || credit.is_some();

            viewer_context.with_viewer(|viewer: Viewer| {
                // Remove only the data source previously owned by this component.
                if should_clear {
                    loaded_data_source.update_value(|owned| {
                        owned.clear_with(|existing| {
                            let _ = viewer.data_sources().remove(existing);
                        });
                    });
                }

                // Load GeoJSON data with or without options
                let promise = if has_options {
                    let mut options = GeoJsonLoadOptions::new();

                    if let Some(color) = stroke {
                        options = options.stroke(color);
                    }
                    if let Some(width) = stroke_width {
                        options = options.stroke_width(width);
                    }
                    if let Some(color) = fill {
                        options = options.fill(color);
                    }
                    if let Some(color) = marker_color {
                        options = options.marker_color(color);
                    }
                    if let Some(size) = marker_size {
                        options = options.marker_size(size);
                    }
                    if let Some(symbol) = marker_symbol {
                        options = options.marker_symbol(symbol);
                    }
                    if let Some(clamp) = clamp_to_ground {
                        options = options.clamp_to_ground(clamp);
                    }
                    if let Some(credit_str) = credit {
                        options = options.credit(credit_str);
                    }

                    geojson_data_source_load_with_options(&url, &options.build())
                } else {
                    geojson_data_source_load(&url)
                };

                let add_promise = viewer.data_sources().add(promise);

                // Handle the promise
                let viewer_ctx = viewer_context;
                let request_gate = request_gate_effect.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match JsFuture::from(add_promise).await {
                        Ok(data_source_js) => {
                            let stale = request_gate.is_stale(next_request);

                            viewer_ctx.with_viewer(|viewer: Viewer| {
                                if stale {
                                    let _ = viewer.data_sources().remove(&data_source_js);
                                    return;
                                }

                                loaded_data_source.update_value(|owned| {
                                    owned.replace_with(data_source_js.clone(), |existing| {
                                        let _ = viewer.data_sources().remove(existing);
                                    });
                                });
                            });

                            web_sys::console::log_1(&JsValue::from_str(&format!(
                                "Successfully loaded GeoJSON from {}",
                                url
                            )));
                        }
                        Err(e) => {
                            web_sys::console::error_1(&JsValue::from_str(&format!(
                                "Failed to load GeoJSON: {:?}",
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
        let _ = (
            url,
            clear_existing,
            stroke,
            stroke_width,
            fill,
            marker_color,
            marker_size,
            marker_symbol,
            clamp_to_ground,
            credit,
        );
    }
}
