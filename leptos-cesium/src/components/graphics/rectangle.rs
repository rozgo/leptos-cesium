//! RectangleGraphics component

use crate::bindings::Material;
use crate::core::JsSignal;
use geo_types::Rect;
use leptos::prelude::*;
use palette::Srgba;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{Color, Rectangle};
#[cfg(target_arch = "wasm32")]
use crate::components::use_entity_context;
#[cfg(target_arch = "wasm32")]
use js_sys::{Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

/// RectangleGraphics component for displaying a rectangle on an entity
#[component(transparent)]
pub fn RectangleGraphics(
    /// Rectangle coordinates as geo_types::Rect (west, south, east, north in degrees)
    #[prop(into)]
    coordinates: Signal<Rect<f64>>,
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
    /// Extruded height in meters
    #[prop(optional, into)]
    extruded_height: Signal<Option<f64>>,
    /// Height of the rectangle in meters
    #[prop(optional, into)]
    height: Signal<Option<f64>>,
    /// Rotation in radians
    #[prop(optional, into)]
    rotation: Signal<Option<f64>>,
    /// Texture rotation in radians
    #[prop(optional, into)]
    st_rotation: Signal<Option<f64>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let entity_context =
            use_entity_context().expect("RectangleGraphics must be a child of Entity");

        Effect::new(move |_| {
            entity_context.with_entity(|entity| {
                let rectangle_options = Object::new();

                // Set coordinates - convert Rect to Cesium Rectangle
                let rect = coordinates.get();
                let cesium_rect: Rectangle = rect.into();
                let _ = Reflect::set(
                    &rectangle_options,
                    &JsValue::from_str("coordinates"),
                    &JsValue::from(cesium_rect),
                );

                // Set material if provided
                if let Some(mat) = material.get_untracked() {
                    let _ = Reflect::set(
                        &rectangle_options,
                        &JsValue::from_str("material"),
                        &mat.to_js_value(),
                    );
                }

                // Set outline if provided
                if let Some(val) = outline.get() {
                    let _ = Reflect::set(
                        &rectangle_options,
                        &JsValue::from_str("outline"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set outline color if provided - convert Srgba to Cesium Color
                if let Some(c) = outline_color.get() {
                    let cesium_color: Color = c.into();
                    let _ = Reflect::set(
                        &rectangle_options,
                        &JsValue::from_str("outlineColor"),
                        &JsValue::from(cesium_color),
                    );
                }

                // Set outline width if provided
                if let Some(width) = outline_width.get() {
                    let _ = Reflect::set(
                        &rectangle_options,
                        &JsValue::from_str("outlineWidth"),
                        &JsValue::from_f64(width),
                    );
                }

                // Set extruded height if provided
                if let Some(val) = extruded_height.get() {
                    let _ = Reflect::set(
                        &rectangle_options,
                        &JsValue::from_str("extrudedHeight"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set height if provided
                if let Some(val) = height.get() {
                    let _ = Reflect::set(
                        &rectangle_options,
                        &JsValue::from_str("height"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set rotation if provided
                if let Some(val) = rotation.get() {
                    let _ = Reflect::set(
                        &rectangle_options,
                        &JsValue::from_str("rotation"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set stRotation if provided
                if let Some(val) = st_rotation.get() {
                    let _ = Reflect::set(
                        &rectangle_options,
                        &JsValue::from_str("stRotation"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set the rectangle property on the entity
                let _ = Reflect::set(&entity, &JsValue::from_str("rectangle"), &rectangle_options);
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            coordinates,
            material,
            outline,
            outline_color,
            outline_width,
            extruded_height,
            height,
            rotation,
            st_rotation,
        );
    }
}
