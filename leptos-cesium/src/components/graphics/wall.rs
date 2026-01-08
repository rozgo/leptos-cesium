//! WallGraphics component

use crate::bindings::Material;
use crate::core::JsSignal;
use geo_types::LineString;
use leptos::prelude::*;
use palette::Srgba;

#[cfg(target_arch = "wasm32")]
use crate::bindings::Color;
#[cfg(target_arch = "wasm32")]
use crate::components::use_entity_context;
#[cfg(target_arch = "wasm32")]
use crate::core::linestring_to_cartesian_array;
#[cfg(target_arch = "wasm32")]
use js_sys::{Array, Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

/// WallGraphics component for displaying a wall between positions on the ground and a height
#[component(transparent)]
pub fn WallGraphics(
    /// Positions as geo_types::LineString (lon, lat pairs in degrees)
    #[prop(into)]
    positions: Signal<LineString<f64>>,
    /// Material (Color or Stripe pattern) - still uses JS Material type
    #[prop(optional, into)]
    material: JsSignal<Option<Material>>,
    /// Array of maximum heights for each position (in meters)
    #[prop(optional, into)]
    maximum_heights: Signal<Option<Vec<f64>>>,
    /// Array of minimum heights for each position (in meters)
    #[prop(optional, into)]
    minimum_heights: Signal<Option<Vec<f64>>>,
    /// Whether to show outline
    #[prop(optional, into)]
    outline: Signal<Option<bool>>,
    /// Outline color as RGBA
    #[prop(optional, into)]
    outline_color: Signal<Option<Srgba<f32>>>,
    /// Outline width
    #[prop(optional, into)]
    outline_width: Signal<Option<f64>>,
    /// Whether the wall is filled
    #[prop(optional, into)]
    fill: Signal<Option<bool>>,
    /// Show the wall
    #[prop(optional, into)]
    show: Signal<Option<bool>>,
    /// Granularity in meters
    #[prop(optional, into)]
    granularity: Signal<Option<f64>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let entity_context = use_entity_context().expect("WallGraphics must be a child of Entity");

        Effect::new(move |_| {
            entity_context.with_entity(|entity| {
                let wall_options = Object::new();

                // Set positions - convert LineString to Cartesian3 array
                let line = positions.get();
                let cesium_positions = linestring_to_cartesian_array(&line);
                let _ = Reflect::set(
                    &wall_options,
                    &JsValue::from_str("positions"),
                    &JsValue::from(cesium_positions),
                );

                // Set material if provided
                if let Some(mat) = material.get_untracked() {
                    let _ = Reflect::set(
                        &wall_options,
                        &JsValue::from_str("material"),
                        &mat.to_js_value(),
                    );
                }

                // Set maximum heights if provided
                if let Some(heights) = maximum_heights.get() {
                    let js_array = Array::new();
                    for h in heights {
                        js_array.push(&JsValue::from_f64(h));
                    }
                    let _ = Reflect::set(
                        &wall_options,
                        &JsValue::from_str("maximumHeights"),
                        &JsValue::from(js_array),
                    );
                }

                // Set minimum heights if provided
                if let Some(heights) = minimum_heights.get() {
                    let js_array = Array::new();
                    for h in heights {
                        js_array.push(&JsValue::from_f64(h));
                    }
                    let _ = Reflect::set(
                        &wall_options,
                        &JsValue::from_str("minimumHeights"),
                        &JsValue::from(js_array),
                    );
                }

                // Set outline if provided
                if let Some(val) = outline.get() {
                    let _ = Reflect::set(
                        &wall_options,
                        &JsValue::from_str("outline"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set outline color if provided - convert Srgba to Cesium Color
                if let Some(c) = outline_color.get() {
                    let cesium_color: Color = c.into();
                    let _ = Reflect::set(
                        &wall_options,
                        &JsValue::from_str("outlineColor"),
                        &JsValue::from(cesium_color),
                    );
                }

                // Set outline width if provided
                if let Some(width) = outline_width.get() {
                    let _ = Reflect::set(
                        &wall_options,
                        &JsValue::from_str("outlineWidth"),
                        &JsValue::from_f64(width),
                    );
                }

                // Set fill if provided
                if let Some(val) = fill.get() {
                    let _ = Reflect::set(
                        &wall_options,
                        &JsValue::from_str("fill"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set show if provided
                if let Some(val) = show.get() {
                    let _ = Reflect::set(
                        &wall_options,
                        &JsValue::from_str("show"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set granularity if provided
                if let Some(val) = granularity.get() {
                    let _ = Reflect::set(
                        &wall_options,
                        &JsValue::from_str("granularity"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set the wall property on the entity
                let _ = Reflect::set(&entity, &JsValue::from_str("wall"), &wall_options);
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            positions,
            material,
            maximum_heights,
            minimum_heights,
            outline,
            outline_color,
            outline_width,
            fill,
            show,
            granularity,
        );
    }
}
