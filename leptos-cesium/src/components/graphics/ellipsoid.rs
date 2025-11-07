//! EllipsoidGraphics component

use crate::bindings::{Cartesian3, Color, Material};
use crate::core::JsSignal;
use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::components::use_entity_context;
#[cfg(target_arch = "wasm32")]
use js_sys::{Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

/// EllipsoidGraphics component for displaying an ellipsoid/sphere on an entity
#[component(transparent)]
pub fn EllipsoidGraphics(
    /// Ellipsoid radii (x, y, z)
    #[prop(into)]
    radii: JsSignal<Cartesian3>,
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
    /// Whether the ellipsoid is filled
    #[prop(optional, into)]
    fill: Signal<Option<bool>>,
    /// Show the ellipsoid
    #[prop(optional, into)]
    show: Signal<Option<bool>>,
    /// Number of vertical lines to draw for the outline
    #[prop(optional, into)]
    stack_partitions: Signal<Option<f64>>,
    /// Number of horizontal lines to draw for the outline
    #[prop(optional, into)]
    slice_partitions: Signal<Option<f64>>,
    /// Number of samples per outline ring
    #[prop(optional, into)]
    subdivision_divisions: Signal<Option<f64>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let entity_context =
            use_entity_context().expect("EllipsoidGraphics must be a child of Entity");

        Effect::new(move |_| {
            entity_context.with_entity(|entity| {
                let ellipsoid_options = Object::new();

                // Set radii
                let _ = Reflect::set(
                    &ellipsoid_options,
                    &JsValue::from_str("radii"),
                    &JsValue::from(radii.get_untracked()),
                );

                // Set material if provided
                if let Some(mat) = material.get_untracked() {
                    let _ = Reflect::set(
                        &ellipsoid_options,
                        &JsValue::from_str("material"),
                        &mat.to_js_value(),
                    );
                }

                // Set outline if provided
                if let Some(val) = outline.get() {
                    let _ = Reflect::set(
                        &ellipsoid_options,
                        &JsValue::from_str("outline"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set outline color if provided
                if let Some(color) = outline_color.get_untracked() {
                    let _ = Reflect::set(
                        &ellipsoid_options,
                        &JsValue::from_str("outlineColor"),
                        &JsValue::from(color),
                    );
                }

                // Set outline width if provided
                if let Some(width) = outline_width.get() {
                    let _ = Reflect::set(
                        &ellipsoid_options,
                        &JsValue::from_str("outlineWidth"),
                        &JsValue::from_f64(width),
                    );
                }

                // Set fill if provided
                if let Some(val) = fill.get() {
                    let _ = Reflect::set(
                        &ellipsoid_options,
                        &JsValue::from_str("fill"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set show if provided
                if let Some(val) = show.get() {
                    let _ = Reflect::set(
                        &ellipsoid_options,
                        &JsValue::from_str("show"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set stack partitions if provided
                if let Some(val) = stack_partitions.get() {
                    let _ = Reflect::set(
                        &ellipsoid_options,
                        &JsValue::from_str("stackPartitions"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set slice partitions if provided
                if let Some(val) = slice_partitions.get() {
                    let _ = Reflect::set(
                        &ellipsoid_options,
                        &JsValue::from_str("slicePartitions"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set subdivision divisions if provided
                if let Some(val) = subdivision_divisions.get() {
                    let _ = Reflect::set(
                        &ellipsoid_options,
                        &JsValue::from_str("subdivisionDivisions"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set the ellipsoid property on the entity
                let _ = Reflect::set(&entity, &JsValue::from_str("ellipsoid"), &ellipsoid_options);
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            radii,
            material,
            outline,
            outline_color,
            outline_width,
            fill,
            show,
            stack_partitions,
            slice_partitions,
            subdivision_divisions,
        );
    }
}
