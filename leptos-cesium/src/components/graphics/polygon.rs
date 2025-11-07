//! PolygonGraphics component

use crate::bindings::{Color, Material, PolygonHierarchy};
use crate::core::JsSignal;
use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::components::use_entity_context;
#[cfg(target_arch = "wasm32")]
use js_sys::{Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

/// PolygonGraphics component for displaying a polygon on an entity
#[component(transparent)]
pub fn PolygonGraphics(
    /// Polygon hierarchy (positions with optional holes)
    #[prop(into)]
    hierarchy: JsSignal<PolygonHierarchy>,
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
    /// Extruded height in meters
    #[prop(optional, into)]
    extruded_height: Signal<Option<f64>>,
    /// Height of the polygon in meters
    #[prop(optional, into)]
    height: Signal<Option<f64>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let entity_context =
            use_entity_context().expect("PolygonGraphics must be a child of Entity");

        Effect::new(move |_| {
            entity_context.with_entity(|entity| {
                let polygon_options = Object::new();

                // Set hierarchy
                let _ = Reflect::set(
                    &polygon_options,
                    &JsValue::from_str("hierarchy"),
                    &JsValue::from(hierarchy.get_untracked()),
                );

                // Set material if provided
                if let Some(mat) = material.get_untracked() {
                    let _ = Reflect::set(
                        &polygon_options,
                        &JsValue::from_str("material"),
                        &mat.to_js_value(),
                    );
                }

                // Set outline if provided
                if let Some(val) = outline.get() {
                    let _ = Reflect::set(
                        &polygon_options,
                        &JsValue::from_str("outline"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set outline color if provided
                if let Some(color) = outline_color.get_untracked() {
                    let _ = Reflect::set(
                        &polygon_options,
                        &JsValue::from_str("outlineColor"),
                        &JsValue::from(color),
                    );
                }

                // Set outline width if provided
                if let Some(width) = outline_width.get() {
                    let _ = Reflect::set(
                        &polygon_options,
                        &JsValue::from_str("outlineWidth"),
                        &JsValue::from_f64(width),
                    );
                }

                // Set extruded height if provided
                if let Some(val) = extruded_height.get() {
                    let _ = Reflect::set(
                        &polygon_options,
                        &JsValue::from_str("extrudedHeight"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set height if provided
                if let Some(val) = height.get() {
                    let _ = Reflect::set(
                        &polygon_options,
                        &JsValue::from_str("height"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set the polygon property on the entity
                let _ = Reflect::set(&entity, &JsValue::from_str("polygon"), &polygon_options);
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            hierarchy,
            material,
            outline,
            outline_color,
            outline_width,
            extruded_height,
            height,
        );
    }
}
