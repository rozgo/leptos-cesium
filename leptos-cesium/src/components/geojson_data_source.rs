//! GeoJSON data source component for loading GeoJSON data declaratively

use leptos::prelude::*;

use crate::bindings::Color;
use crate::core::JsSignal;

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
/// data sources. When the URL changes, the previous data source is removed and the new
/// one is loaded.
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

    /// Whether to remove all existing data sources before loading (default: true)
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

        Effect::new(move |_| {
            let url = url.get();
            let should_clear = clear_existing.get();

            // Build options if any styling props are provided
            let has_options = stroke.get_untracked().is_some()
                || stroke_width.get().is_some()
                || fill.get_untracked().is_some()
                || marker_color.get_untracked().is_some()
                || marker_size.get().is_some()
                || marker_symbol.get().is_some()
                || clamp_to_ground.get().is_some()
                || credit.get().is_some();

            viewer_context.with_viewer(|viewer: Viewer| {
                // Clear existing data sources if requested
                if should_clear {
                    viewer.data_sources().remove_all();
                }

                // Load GeoJSON data with or without options
                let promise = if has_options {
                    let mut options = GeoJsonLoadOptions::new();

                    if let Some(color) = stroke.get_untracked() {
                        options = options.stroke(color);
                    }
                    if let Some(width) = stroke_width.get() {
                        options = options.stroke_width(width);
                    }
                    if let Some(color) = fill.get_untracked() {
                        options = options.fill(color);
                    }
                    if let Some(color) = marker_color.get_untracked() {
                        options = options.marker_color(color);
                    }
                    if let Some(size) = marker_size.get() {
                        options = options.marker_size(size);
                    }
                    if let Some(symbol) = marker_symbol.get() {
                        options = options.marker_symbol(symbol);
                    }
                    if let Some(clamp) = clamp_to_ground.get() {
                        options = options.clamp_to_ground(clamp);
                    }
                    if let Some(credit_str) = credit.get() {
                        options = options.credit(credit_str);
                    }

                    geojson_data_source_load_with_options(&url, &options.build())
                } else {
                    geojson_data_source_load(&url)
                };

                let add_promise = viewer.data_sources().add(promise);

                // Handle the promise
                wasm_bindgen_futures::spawn_local(async move {
                    match JsFuture::from(add_promise).await {
                        Ok(_data_source_js) => {
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
            // Clear data sources when component unmounts
            #[cfg(target_arch = "wasm32")]
            if let Some(viewer_ctx) = use_cesium_context() {
                viewer_ctx.with_viewer(|viewer: Viewer| {
                    viewer.data_sources().remove_all();
                });
            }
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
