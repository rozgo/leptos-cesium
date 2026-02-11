//! CorridorGraphics component

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
use js_sys::{Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

/// CorridorGraphics component for displaying a corridor along a path
#[component(transparent)]
pub fn CorridorGraphics(
    /// Positions as geo_types::LineString (lon, lat pairs in degrees)
    #[prop(into)]
    positions: Signal<LineString<f64>>,
    /// Width of the corridor in meters
    #[prop(into)]
    width: Signal<f64>,
    /// Material (Color or Stripe pattern) - still uses JS Material type
    #[prop(optional, into)]
    material: JsSignal<Option<Material>>,
    /// Height of the corridor above the surface
    #[prop(optional, into)]
    height: Signal<Option<f64>>,
    /// Extruded height of the corridor
    #[prop(optional, into)]
    extruded_height: Signal<Option<f64>>,
    /// Whether to show outline
    #[prop(optional, into)]
    outline: Signal<Option<bool>>,
    /// Outline color as RGBA
    #[prop(optional, into)]
    outline_color: Signal<Option<Srgba<f32>>>,
    /// Outline width
    #[prop(optional, into)]
    outline_width: Signal<Option<f64>>,
    /// Whether the corridor is filled
    #[prop(optional, into)]
    fill: Signal<Option<bool>>,
    /// Show the corridor
    #[prop(optional, into)]
    show: Signal<Option<bool>>,
    /// Corner type (ROUNDED, MITERED, BEVELED)
    #[prop(optional, into)]
    corner_type: Signal<Option<f64>>,
    /// Granularity in meters
    #[prop(optional, into)]
    granularity: Signal<Option<f64>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let entity_context =
            use_entity_context().expect("CorridorGraphics must be a child of Entity");

        Effect::new(move |_| {
            entity_context.with_entity(|entity| {
                let corridor_options = Object::new();

                // Set positions - convert LineString to Cartesian3 array
                let line = positions.get();
                let cesium_positions = linestring_to_cartesian_array(&line);
                let _ = Reflect::set(
                    &corridor_options,
                    &JsValue::from_str("positions"),
                    &JsValue::from(cesium_positions),
                );

                // Set width
                let _ = Reflect::set(
                    &corridor_options,
                    &JsValue::from_str("width"),
                    &JsValue::from_f64(width.get()),
                );

                // Set material if provided
                if let Some(mat) = material.get() {
                    let _ = Reflect::set(
                        &corridor_options,
                        &JsValue::from_str("material"),
                        &mat.to_js_value(),
                    );
                }

                // Set height if provided
                if let Some(val) = height.get() {
                    let _ = Reflect::set(
                        &corridor_options,
                        &JsValue::from_str("height"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set extruded height if provided
                if let Some(val) = extruded_height.get() {
                    let _ = Reflect::set(
                        &corridor_options,
                        &JsValue::from_str("extrudedHeight"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set outline if provided
                if let Some(val) = outline.get() {
                    let _ = Reflect::set(
                        &corridor_options,
                        &JsValue::from_str("outline"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set outline color if provided - convert Srgba to Cesium Color
                if let Some(c) = outline_color.get() {
                    let cesium_color: Color = c.into();
                    let _ = Reflect::set(
                        &corridor_options,
                        &JsValue::from_str("outlineColor"),
                        &JsValue::from(cesium_color),
                    );
                }

                // Set outline width if provided
                if let Some(width_val) = outline_width.get() {
                    let _ = Reflect::set(
                        &corridor_options,
                        &JsValue::from_str("outlineWidth"),
                        &JsValue::from_f64(width_val),
                    );
                }

                // Set fill if provided
                if let Some(val) = fill.get() {
                    let _ = Reflect::set(
                        &corridor_options,
                        &JsValue::from_str("fill"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set show if provided
                if let Some(val) = show.get() {
                    let _ = Reflect::set(
                        &corridor_options,
                        &JsValue::from_str("show"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set corner type if provided
                if let Some(val) = corner_type.get() {
                    let _ = Reflect::set(
                        &corridor_options,
                        &JsValue::from_str("cornerType"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set granularity if provided
                if let Some(val) = granularity.get() {
                    let _ = Reflect::set(
                        &corridor_options,
                        &JsValue::from_str("granularity"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set the corridor property on the entity
                let _ = Reflect::set(&entity, &JsValue::from_str("corridor"), &corridor_options);
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            positions,
            width,
            material,
            height,
            extruded_height,
            outline,
            outline_color,
            outline_width,
            fill,
            show,
            corner_type,
            granularity,
        );
    }
}
