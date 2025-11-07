//! PolylineVolumeGraphics component

use crate::bindings::{Color, Material};
use crate::core::JsSignal;
use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::components::use_entity_context;
#[cfg(target_arch = "wasm32")]
use js_sys::{Array, Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

/// PolylineVolumeGraphics component for displaying a polyline with a 2D shape extruded along it
#[component(transparent)]
pub fn PolylineVolumeGraphics(
    /// Array of Cartesian3 positions that define the center line
    #[prop(into)]
    positions: JsSignal<Array>,
    /// Array of Cartesian2 positions defining the 2D shape to be extruded
    #[prop(into)]
    shape: JsSignal<Array>,
    /// Material (Color or Stripe pattern)
    #[prop(optional, into)]
    material: JsSignal<Option<Material>>,
    /// Whether to show outline
    #[prop(optional, into)]
    outline: Signal<Option<bool>>,
    /// Outline color
    #[prop(optional, into)]
    outline_color: JsSignal<Option<Color>>,
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

                // Set positions
                let _ = Reflect::set(
                    &polyline_volume_options,
                    &JsValue::from_str("positions"),
                    &JsValue::from(positions.get_untracked()),
                );

                // Set shape
                let _ = Reflect::set(
                    &polyline_volume_options,
                    &JsValue::from_str("shape"),
                    &JsValue::from(shape.get_untracked()),
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

                // Set outline color if provided
                if let Some(color) = outline_color.get_untracked() {
                    let _ = Reflect::set(
                        &polyline_volume_options,
                        &JsValue::from_str("outlineColor"),
                        &JsValue::from(color),
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
