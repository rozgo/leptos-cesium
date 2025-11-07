//! BoxGraphics component

use crate::bindings::{Cartesian3, Color, Material};
use crate::core::JsSignal;
use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::components::use_entity_context;
#[cfg(target_arch = "wasm32")]
use js_sys::{Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

/// BoxGraphics component for displaying a box on an entity
#[component(transparent)]
pub fn BoxGraphics(
    /// Box dimensions (width, height, depth)
    #[prop(into)]
    dimensions: JsSignal<Cartesian3>,
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
    /// Whether the box is filled
    #[prop(optional, into)]
    fill: Signal<Option<bool>>,
    /// Show the box
    #[prop(optional, into)]
    show: Signal<Option<bool>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let entity_context =
            use_entity_context().expect("BoxGraphics must be a child of Entity");

        Effect::new(move |_| {
            entity_context.with_entity(|entity| {
                let box_options = Object::new();

                // Set dimensions
                let _ = Reflect::set(
                    &box_options,
                    &JsValue::from_str("dimensions"),
                    &JsValue::from(dimensions.get_untracked()),
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

                // Set outline color if provided
                if let Some(color) = outline_color.get_untracked() {
                    let _ = Reflect::set(
                        &box_options,
                        &JsValue::from_str("outlineColor"),
                        &JsValue::from(color),
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
