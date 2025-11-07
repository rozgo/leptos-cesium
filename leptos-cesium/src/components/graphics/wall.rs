//! WallGraphics component

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

/// WallGraphics component for displaying a wall between positions on the ground and a height
#[component(transparent)]
pub fn WallGraphics(
    /// Array of Cartesian3 positions that define the wall path
    #[prop(into)]
    positions: JsSignal<Array>,
    /// Material (Color or Stripe pattern)
    #[prop(optional, into)]
    material: JsSignal<Option<Material>>,
    /// Array of maximum heights for each position
    #[prop(optional, into)]
    maximum_heights: JsSignal<Option<Array>>,
    /// Array of minimum heights for each position
    #[prop(optional, into)]
    minimum_heights: JsSignal<Option<Array>>,
    /// Whether to show outline
    #[prop(optional, into)]
    outline: Signal<Option<bool>>,
    /// Outline color
    #[prop(optional, into)]
    outline_color: JsSignal<Option<Color>>,
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

                // Set positions
                let _ = Reflect::set(
                    &wall_options,
                    &JsValue::from_str("positions"),
                    &JsValue::from(positions.get_untracked()),
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
                if let Some(heights) = maximum_heights.get_untracked() {
                    let _ = Reflect::set(
                        &wall_options,
                        &JsValue::from_str("maximumHeights"),
                        &JsValue::from(heights),
                    );
                }

                // Set minimum heights if provided
                if let Some(heights) = minimum_heights.get_untracked() {
                    let _ = Reflect::set(
                        &wall_options,
                        &JsValue::from_str("minimumHeights"),
                        &JsValue::from(heights),
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

                // Set outline color if provided
                if let Some(color) = outline_color.get_untracked() {
                    let _ = Reflect::set(
                        &wall_options,
                        &JsValue::from_str("outlineColor"),
                        &JsValue::from(color),
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
