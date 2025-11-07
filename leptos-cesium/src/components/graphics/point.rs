//! PointGraphics component

use crate::bindings::Color;
use crate::core::JsSignal;
use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::components::use_entity_context;
#[cfg(target_arch = "wasm32")]
use js_sys::{Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

/// PointGraphics component for displaying a point on an entity
#[component(transparent)]
pub fn PointGraphics(
    /// Size of the point in pixels
    #[prop(into)]
    pixel_size: Signal<f64>,
    /// Point color
    #[prop(optional, into)]
    color: JsSignal<Option<Color>>,
    /// Whether to show outline
    #[prop(optional, into)]
    outline: Signal<Option<bool>>,
    /// Outline color
    #[prop(optional, into)]
    outline_color: JsSignal<Option<Color>>,
    /// Outline width in pixels
    #[prop(optional, into)]
    outline_width: Signal<Option<f64>>,
    /// Show the point
    #[prop(optional, into)]
    show: Signal<Option<bool>>,
    /// Height reference (NONE, CLAMP_TO_GROUND, RELATIVE_TO_GROUND)
    #[prop(optional, into)]
    height_reference: Signal<Option<f64>>,
    /// Distance in meters from camera to disable depth test
    #[prop(optional, into)]
    disable_depth_test_distance: Signal<Option<f64>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let entity_context = use_entity_context().expect("PointGraphics must be a child of Entity");

        Effect::new(move |_| {
            entity_context.with_entity(|entity| {
                let point_options = Object::new();

                // Set pixel size
                let _ = Reflect::set(
                    &point_options,
                    &JsValue::from_str("pixelSize"),
                    &JsValue::from_f64(pixel_size.get()),
                );

                // Set color if provided
                if let Some(c) = color.get_untracked() {
                    let _ = Reflect::set(
                        &point_options,
                        &JsValue::from_str("color"),
                        &JsValue::from(c),
                    );
                }

                // Set outline if provided
                if let Some(val) = outline.get() {
                    let _ = Reflect::set(
                        &point_options,
                        &JsValue::from_str("outlineColor"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set outline color if provided
                if let Some(c) = outline_color.get_untracked() {
                    let _ = Reflect::set(
                        &point_options,
                        &JsValue::from_str("outlineColor"),
                        &JsValue::from(c),
                    );
                }

                // Set outline width if provided
                if let Some(width) = outline_width.get() {
                    let _ = Reflect::set(
                        &point_options,
                        &JsValue::from_str("outlineWidth"),
                        &JsValue::from_f64(width),
                    );
                }

                // Set show if provided
                if let Some(val) = show.get() {
                    let _ = Reflect::set(
                        &point_options,
                        &JsValue::from_str("show"),
                        &JsValue::from_bool(val),
                    );
                }

                // Set height reference if provided
                if let Some(val) = height_reference.get() {
                    let _ = Reflect::set(
                        &point_options,
                        &JsValue::from_str("heightReference"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set disable depth test distance if provided
                if let Some(val) = disable_depth_test_distance.get() {
                    let _ = Reflect::set(
                        &point_options,
                        &JsValue::from_str("disableDepthTestDistance"),
                        &JsValue::from_f64(val),
                    );
                }

                // Set the point property on the entity
                let _ = Reflect::set(&entity, &JsValue::from_str("point"), &point_options);
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            pixel_size,
            color,
            outline,
            outline_color,
            outline_width,
            show,
            height_reference,
            disable_depth_test_distance,
        );
    }
}
