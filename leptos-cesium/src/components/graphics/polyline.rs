//! PolylineGraphics component

use crate::bindings::{Color, Material};
use crate::core::JsSignal;
use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::components::use_entity_context;
#[cfg(target_arch = "wasm32")]
use js_sys::{Array, Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(not(target_arch = "wasm32"))]
type Array = ();

/// PolylineGraphics component for displaying a polyline on an entity
#[component(transparent)]
pub fn PolylineGraphics(
    /// Array of Cartesian3 positions that define the line
    #[prop(into)]
    positions: JsSignal<Array>,
    /// Width of the polyline in pixels
    #[prop(into)]
    width: Signal<f64>,
    /// Material (Color or polyline-specific materials)
    #[prop(optional, into)]
    material: JsSignal<Option<Material>>,
    /// Whether to clamp the line to the ground
    #[prop(optional, into)]
    clamp_to_ground: Signal<Option<bool>>,
    /// Show the polyline
    #[prop(optional, into)]
    show: Signal<Option<bool>>,
    /// Line granularity in meters
    #[prop(optional, into)]
    granularity: Signal<Option<f64>>,
    /// Follow the surface of the ellipsoid
    #[prop(optional, into)]
    follow_surface: Signal<Option<bool>>,
    /// Depth fail material
    #[prop(optional, into)]
    depth_fail_material: JsSignal<Option<Color>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let entity_context =
            use_entity_context().expect("PolylineGraphics must be a child of Entity");

        Effect::new(move |_| {
            entity_context.with_entity(|entity| {
                let polyline_options = Object::new();

                // Set positions
                let _ = Reflect::set(
                    &polyline_options,
                    &JsValue::from_str("positions"),
                    &JsValue::from(positions.get_untracked()),
                );

                // Set width
                let _ = Reflect::set(
                    &polyline_options,
                    &JsValue::from_str("width"),
                    &JsValue::from_f64(width.get()),
                );

                // Set material if provided
                if let Some(mat) = material.get_untracked() {
                    let _ = Reflect::set(
                        &polyline_options,
                        &JsValue::from_str("material"),
                        &mat.to_js_value(),
                    );
                }

                // Set clamp to ground if provided
                if let Some(val) = clamp_to_ground.get() {
                    let _ = Reflect::set(
                        &polyline_options,
                        &JsValue::from_str("clampToGround"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set show if provided
                if let Some(val) = show.get() {
                    let _ = Reflect::set(
                        &polyline_options,
                        &JsValue::from_str("show"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set granularity if provided
                if let Some(val) = granularity.get() {
                    let _ = Reflect::set(
                        &polyline_options,
                        &JsValue::from_str("granularity"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set follow surface if provided
                if let Some(val) = follow_surface.get() {
                    let _ = Reflect::set(
                        &polyline_options,
                        &JsValue::from_str("followSurface"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set depth fail material if provided
                if let Some(color) = depth_fail_material.get_untracked() {
                    let _ = Reflect::set(
                        &polyline_options,
                        &JsValue::from_str("depthFailMaterial"),
                        &JsValue::from(color),
                    );
                }

                // Set the polyline property on the entity
                let _ = Reflect::set(&entity, &JsValue::from_str("polyline"), &polyline_options);
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            positions,
            width,
            material,
            clamp_to_ground,
            show,
            granularity,
            follow_surface,
            depth_fail_material,
        );
    }
}
