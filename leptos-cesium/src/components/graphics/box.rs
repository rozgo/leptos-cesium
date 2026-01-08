//! BoxGraphics component

use crate::bindings::Material;
use crate::core::JsSignal;
use glam::DVec3;
use leptos::prelude::*;
use palette::Srgba;

#[cfg(target_arch = "wasm32")]
use crate::bindings::Color;
#[cfg(target_arch = "wasm32")]
use crate::components::use_entity_context;
#[cfg(target_arch = "wasm32")]
use crate::core::dvec3_to_cartesian_dimensions;
#[cfg(target_arch = "wasm32")]
use js_sys::{Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

/// BoxGraphics component for displaying a box on an entity
#[component(transparent)]
pub fn BoxGraphics(
    /// Box dimensions as DVec3 (x=width, y=height, z=depth in meters)
    #[prop(into)]
    dimensions: Signal<DVec3>,
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
    /// Whether the box is filled
    #[prop(optional, into)]
    fill: Signal<Option<bool>>,
    /// Show the box
    #[prop(optional, into)]
    show: Signal<Option<bool>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let entity_context = use_entity_context().expect("BoxGraphics must be a child of Entity");

        Effect::new(move |_| {
            entity_context.with_entity(|entity| {
                let box_options = Object::new();

                // Set dimensions - convert DVec3 to Cesium Cartesian3 (dimensional, not geographic)
                let dims = dimensions.get();
                let cesium_dims = dvec3_to_cartesian_dimensions(dims);
                let _ = Reflect::set(
                    &box_options,
                    &JsValue::from_str("dimensions"),
                    &JsValue::from(cesium_dims),
                );

                // Set material if provided
                if let Some(mat) = material.get_untracked() {
                    let _ = Reflect::set(
                        &box_options,
                        &JsValue::from_str("material"),
                        &mat.to_js_value(),
                    );
                }

                // Set outline if provided
                if let Some(val) = outline.get() {
                    let _ = Reflect::set(
                        &box_options,
                        &JsValue::from_str("outline"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set outline color if provided - convert Srgba to Cesium Color
                if let Some(c) = outline_color.get() {
                    let cesium_color: Color = c.into();
                    let _ = Reflect::set(
                        &box_options,
                        &JsValue::from_str("outlineColor"),
                        &JsValue::from(cesium_color),
                    );
                }

                // Set outline width if provided
                if let Some(width) = outline_width.get() {
                    let _ = Reflect::set(
                        &box_options,
                        &JsValue::from_str("outlineWidth"),
                        &JsValue::from_f64(width),
                    );
                }

                // Set fill if provided
                if let Some(val) = fill.get() {
                    let _ = Reflect::set(
                        &box_options,
                        &JsValue::from_str("fill"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set show if provided
                if let Some(val) = show.get() {
                    let _ = Reflect::set(
                        &box_options,
                        &JsValue::from_str("show"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set the box property on the entity
                let _ = Reflect::set(&entity, &JsValue::from_str("box"), &box_options);
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            dimensions,
            material,
            outline,
            outline_color,
            outline_width,
            fill,
            show,
        );
    }
}
