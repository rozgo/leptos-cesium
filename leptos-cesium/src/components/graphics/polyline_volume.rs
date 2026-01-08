//! PolylineVolumeGraphics component

use crate::bindings::Material;
use crate::core::JsSignal;
use geo_types::LineString;
use glam::DVec2;
use leptos::prelude::*;
use palette::Srgba;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{Cartesian2, Color};
#[cfg(target_arch = "wasm32")]
use crate::components::use_entity_context;
#[cfg(target_arch = "wasm32")]
use crate::core::linestring_to_cartesian_array;
#[cfg(target_arch = "wasm32")]
use js_sys::{Array, Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

/// PolylineVolumeGraphics component for displaying a polyline with a 2D shape extruded along it
#[component(transparent)]
pub fn PolylineVolumeGraphics(
    /// Positions as geo_types::LineString (lon, lat pairs in degrees)
    #[prop(into)]
    positions: Signal<LineString<f64>>,
    /// 2D shape as Vec<DVec2> (x, y pairs in meters defining the cross-section)
    #[prop(into)]
    shape: Signal<Vec<DVec2>>,
    /// Material (Color or Stripe pattern) - still uses JS Material type
    #[prop(optional, into)]
    material: JsSignal<Option<Material>>,
    /// Whether to show outline
    #[prop(optional, into)]
    outline: Signal<Option<bool>>,
    /// Outline color as RGBA
    #[prop(optional, into)]
    outline_color: Signal<Option<Srgba<f32>>>,
    /// Outline width
    #[prop(optional, into)]
    outline_width: Signal<Option<f64>>,
    /// Whether the volume is filled
    #[prop(optional, into)]
    fill: Signal<Option<bool>>,
    /// Show the polyline volume
    #[prop(optional, into)]
    show: Signal<Option<bool>>,
    /// Granularity in meters
    #[prop(optional, into)]
    granularity: Signal<Option<f64>>,
    /// Corner type (ROUNDED, MITERED, BEVELED)
    #[prop(optional, into)]
    corner_type: Signal<Option<f64>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let entity_context =
            use_entity_context().expect("PolylineVolumeGraphics must be a child of Entity");

        Effect::new(move |_| {
            entity_context.with_entity(|entity| {
                let polyline_volume_options = Object::new();

                // Set positions - convert LineString to Cartesian3 array
                let line = positions.get();
                let cesium_positions = linestring_to_cartesian_array(&line);
                let _ = Reflect::set(
                    &polyline_volume_options,
                    &JsValue::from_str("positions"),
                    &JsValue::from(cesium_positions),
                );

                // Set shape - convert Vec<DVec2> to Cartesian2 array
                let shape_vec = shape.get();
                let shape_array = Array::new();
                for v in shape_vec {
                    shape_array.push(&JsValue::from(Cartesian2::new(v.x, v.y)));
                }
                let _ = Reflect::set(
                    &polyline_volume_options,
                    &JsValue::from_str("shape"),
                    &JsValue::from(shape_array),
                );

                // Set material if provided
                if let Some(mat) = material.get_untracked() {
                    let _ = Reflect::set(
                        &polyline_volume_options,
                        &JsValue::from_str("material"),
                        &mat.to_js_value(),
                    );
                }

                // Set outline if provided
                if let Some(val) = outline.get() {
                    let _ = Reflect::set(
                        &polyline_volume_options,
                        &JsValue::from_str("outline"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set outline color if provided - convert Srgba to Cesium Color
                if let Some(c) = outline_color.get() {
                    let cesium_color: Color = c.into();
                    let _ = Reflect::set(
                        &polyline_volume_options,
                        &JsValue::from_str("outlineColor"),
                        &JsValue::from(cesium_color),
                    );
                }

                // Set outline width if provided
                if let Some(width) = outline_width.get() {
                    let _ = Reflect::set(
                        &polyline_volume_options,
                        &JsValue::from_str("outlineWidth"),
                        &JsValue::from_f64(width),
                    );
                }

                // Set fill if provided
                if let Some(val) = fill.get() {
                    let _ = Reflect::set(
                        &polyline_volume_options,
                        &JsValue::from_str("fill"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set show if provided
                if let Some(val) = show.get() {
                    let _ = Reflect::set(
                        &polyline_volume_options,
                        &JsValue::from_str("show"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set granularity if provided
                if let Some(val) = granularity.get() {
                    let _ = Reflect::set(
                        &polyline_volume_options,
                        &JsValue::from_str("granularity"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set corner type if provided
                if let Some(val) = corner_type.get() {
                    let _ = Reflect::set(
                        &polyline_volume_options,
                        &JsValue::from_str("cornerType"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set the polylineVolume property on the entity
                let _ = Reflect::set(
                    &entity,
                    &JsValue::from_str("polylineVolume"),
                    &polyline_volume_options,
                );
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            positions,
            shape,
            material,
            outline,
            outline_color,
            outline_width,
            fill,
            show,
            granularity,
            corner_type,
        );
    }
}
