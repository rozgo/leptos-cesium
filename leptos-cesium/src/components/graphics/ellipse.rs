//! EllipseGraphics component

use crate::bindings::{Color, Material};
use crate::core::JsSignal;
use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::components::use_entity_context;
#[cfg(target_arch = "wasm32")]
use js_sys::{Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

/// EllipseGraphics component for displaying an ellipse on an entity
#[component(transparent)]
pub fn EllipseGraphics(
    /// Semi-minor axis in meters
    #[prop(into)]
    semi_minor_axis: Signal<f64>,
    /// Semi-major axis in meters
    #[prop(into)]
    semi_major_axis: Signal<f64>,
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
    /// Rotation in radians
    #[prop(optional, into)]
    rotation: Signal<Option<f64>>,
    /// Texture rotation in radians
    #[prop(optional, into)]
    st_rotation: Signal<Option<f64>>,
    /// Extruded height in meters
    #[prop(optional, into)]
    extruded_height: Signal<Option<f64>>,
    /// Height of the ellipse in meters
    #[prop(optional, into)]
    height: Signal<Option<f64>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let entity_context =
            use_entity_context().expect("EllipseGraphics must be a child of Entity");

        Effect::new(move |_| {
            entity_context.with_entity(|entity| {
                let ellipse_options = Object::new();

                // Set semi axes
                let _ = Reflect::set(
                    &ellipse_options,
                    &JsValue::from_str("semiMinorAxis"),
                    &JsValue::from_f64(semi_minor_axis.get()),
                );
                let _ = Reflect::set(
                    &ellipse_options,
                    &JsValue::from_str("semiMajorAxis"),
                    &JsValue::from_f64(semi_major_axis.get()),
                );

                // Set material if provided
                if let Some(mat) = material.get_untracked() {
                    let _ = Reflect::set(
                        &ellipse_options,
                        &JsValue::from_str("material"),
                        &mat.to_js_value(),
                    );
                }

                // Set outline if provided
                if let Some(val) = outline.get() {
                    let _ = Reflect::set(
                        &ellipse_options,
                        &JsValue::from_str("outline"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set outline color if provided
                if let Some(color) = outline_color.get_untracked() {
                    let _ = Reflect::set(
                        &ellipse_options,
                        &JsValue::from_str("outlineColor"),
                        &JsValue::from(color),
                    );
                }

                // Set outline width if provided
                if let Some(width) = outline_width.get() {
                    let _ = Reflect::set(
                        &ellipse_options,
                        &JsValue::from_str("outlineWidth"),
                        &JsValue::from_f64(width),
                    );
                }

                // Set rotation if provided
                if let Some(val) = rotation.get() {
                    let _ = Reflect::set(
                        &ellipse_options,
                        &JsValue::from_str("rotation"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set stRotation if provided
                if let Some(val) = st_rotation.get() {
                    let _ = Reflect::set(
                        &ellipse_options,
                        &JsValue::from_str("stRotation"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set extruded height if provided
                if let Some(val) = extruded_height.get() {
                    let _ = Reflect::set(
                        &ellipse_options,
                        &JsValue::from_str("extrudedHeight"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set height if provided
                if let Some(val) = height.get() {
                    let _ = Reflect::set(
                        &ellipse_options,
                        &JsValue::from_str("height"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set the ellipse property on the entity
                let _ = Reflect::set(&entity, &JsValue::from_str("ellipse"), &ellipse_options);
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            semi_minor_axis,
            semi_major_axis,
            material,
            outline,
            outline_color,
            outline_width,
            rotation,
            st_rotation,
            extruded_height,
            height,
        );
    }
}
