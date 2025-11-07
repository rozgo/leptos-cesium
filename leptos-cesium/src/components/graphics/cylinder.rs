//! CylinderGraphics component

use crate::bindings::{Color, Material};
use crate::core::JsSignal;
use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::components::use_entity_context;
#[cfg(target_arch = "wasm32")]
use js_sys::{Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

/// CylinderGraphics component for displaying a cylinder on an entity
#[component(transparent)]
pub fn CylinderGraphics(
    /// Length of the cylinder
    #[prop(into)]
    length: Signal<f64>,
    /// Radius of the top of the cylinder
    #[prop(into)]
    top_radius: Signal<f64>,
    /// Radius of the bottom of the cylinder
    #[prop(into)]
    bottom_radius: Signal<f64>,
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
    /// Whether the cylinder is filled
    #[prop(optional, into)]
    fill: Signal<Option<bool>>,
    /// Show the cylinder
    #[prop(optional, into)]
    show: Signal<Option<bool>>,
    /// Number of vertical lines to use for the outline
    #[prop(optional, into)]
    number_of_vertical_lines: Signal<Option<f64>>,
    /// Number of edges around the perimeter
    #[prop(optional, into)]
    slices: Signal<Option<f64>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let entity_context =
            use_entity_context().expect("CylinderGraphics must be a child of Entity");

        Effect::new(move |_| {
            entity_context.with_entity(|entity| {
                let cylinder_options = Object::new();

                // Set length
                let _ = Reflect::set(
                    &cylinder_options,
                    &JsValue::from_str("length"),
                    &JsValue::from_f64(length.get()),
                );

                // Set top radius
                let _ = Reflect::set(
                    &cylinder_options,
                    &JsValue::from_str("topRadius"),
                    &JsValue::from_f64(top_radius.get()),
                );

                // Set bottom radius
                let _ = Reflect::set(
                    &cylinder_options,
                    &JsValue::from_str("bottomRadius"),
                    &JsValue::from_f64(bottom_radius.get()),
                );

                // Set material if provided
                if let Some(mat) = material.get_untracked() {
                    let _ = Reflect::set(
                        &cylinder_options,
                        &JsValue::from_str("material"),
                        &mat.to_js_value(),
                    );
                }

                // Set outline if provided
                if let Some(val) = outline.get() {
                    let _ = Reflect::set(
                        &cylinder_options,
                        &JsValue::from_str("outline"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set outline color if provided
                if let Some(color) = outline_color.get_untracked() {
                    let _ = Reflect::set(
                        &cylinder_options,
                        &JsValue::from_str("outlineColor"),
                        &JsValue::from(color),
                    );
                }

                // Set outline width if provided
                if let Some(width) = outline_width.get() {
                    let _ = Reflect::set(
                        &cylinder_options,
                        &JsValue::from_str("outlineWidth"),
                        &JsValue::from_f64(width),
                    );
                }

                // Set fill if provided
                if let Some(val) = fill.get() {
                    let _ = Reflect::set(
                        &cylinder_options,
                        &JsValue::from_str("fill"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set show if provided
                if let Some(val) = show.get() {
                    let _ = Reflect::set(
                        &cylinder_options,
                        &JsValue::from_str("show"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set number of vertical lines if provided
                if let Some(val) = number_of_vertical_lines.get() {
                    let _ = Reflect::set(
                        &cylinder_options,
                        &JsValue::from_str("numberOfVerticalLines"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set slices if provided
                if let Some(val) = slices.get() {
                    let _ = Reflect::set(
                        &cylinder_options,
                        &JsValue::from_str("slices"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set the cylinder property on the entity
                let _ = Reflect::set(&entity, &JsValue::from_str("cylinder"), &cylinder_options);
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            length,
            top_radius,
            bottom_radius,
            material,
            outline,
            outline_color,
            outline_width,
            fill,
            show,
            number_of_vertical_lines,
            slices,
        );
    }
}
