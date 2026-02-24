//! Billboard graphics component.

use glam::DVec3;
use leptos::prelude::*;
use palette::Srgba;

use crate::bindings::{HorizontalOrigin, MediaSource, VerticalOrigin};
use crate::core::JsSignal;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{Cartesian2, Color};
#[cfg(target_arch = "wasm32")]
use crate::components::use_entity_context;
#[cfg(target_arch = "wasm32")]
use crate::core::dvec3_to_cartesian_dimensions;
#[cfg(target_arch = "wasm32")]
use js_sys::{Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

/// Billboard graphics attached to an entity.
#[component(transparent)]
pub fn BillboardGraphics(
    /// Image/media source for the billboard.
    #[prop(optional, into)]
    image: JsSignal<Option<MediaSource>>,
    /// Uniform billboard scale.
    #[prop(optional, into)]
    scale: Signal<Option<f64>>,
    /// Explicit billboard width in pixels.
    #[prop(optional, into)]
    width: Signal<Option<f64>>,
    /// Explicit billboard height in pixels.
    #[prop(optional, into)]
    height: Signal<Option<f64>>,
    /// Billboard rotation in radians.
    #[prop(optional, into)]
    rotation: Signal<Option<f64>>,
    /// Optional tint color.
    #[prop(optional, into)]
    color: Signal<Option<Srgba<f32>>>,
    /// Horizontal anchor origin.
    #[prop(optional, into)]
    horizontal_origin: Signal<Option<HorizontalOrigin>>,
    /// Vertical anchor origin.
    #[prop(optional, into)]
    vertical_origin: Signal<Option<VerticalOrigin>>,
    /// Pixel offset in screen space.
    #[prop(optional, into)]
    pixel_offset: Signal<Option<(f64, f64)>>,
    /// Eye offset in meters.
    #[prop(optional, into)]
    eye_offset: Signal<Option<DVec3>>,
    /// Show/hide billboard.
    #[prop(optional, into)]
    show: Signal<Option<bool>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let entity_context =
            use_entity_context().expect("BillboardGraphics must be a child of Entity");

        Effect::new(move |_| {
            entity_context.with_entity(|entity| {
                let billboard_options = Object::new();

                if let Some(source) = image.get() {
                    let _ = Reflect::set(
                        &billboard_options,
                        &JsValue::from_str("image"),
                        &source.to_js_value(),
                    );
                }

                if let Some(value) = scale.get() {
                    let _ = Reflect::set(
                        &billboard_options,
                        &JsValue::from_str("scale"),
                        &JsValue::from_f64(value),
                    );
                }

                if let Some(value) = width.get() {
                    let _ = Reflect::set(
                        &billboard_options,
                        &JsValue::from_str("width"),
                        &JsValue::from_f64(value),
                    );
                }

                if let Some(value) = height.get() {
                    let _ = Reflect::set(
                        &billboard_options,
                        &JsValue::from_str("height"),
                        &JsValue::from_f64(value),
                    );
                }

                if let Some(value) = rotation.get() {
                    let _ = Reflect::set(
                        &billboard_options,
                        &JsValue::from_str("rotation"),
                        &JsValue::from_f64(value),
                    );
                }

                if let Some(value) = color.get() {
                    let color: Color = value.into();
                    let _ = Reflect::set(
                        &billboard_options,
                        &JsValue::from_str("color"),
                        &JsValue::from(color),
                    );
                }

                if let Some(value) = horizontal_origin.get() {
                    let _ = Reflect::set(
                        &billboard_options,
                        &JsValue::from_str("horizontalOrigin"),
                        &value.to_js_value(),
                    );
                }

                if let Some(value) = vertical_origin.get() {
                    let _ = Reflect::set(
                        &billboard_options,
                        &JsValue::from_str("verticalOrigin"),
                        &value.to_js_value(),
                    );
                }

                if let Some((x, y)) = pixel_offset.get() {
                    let offset = Cartesian2::new(x, y);
                    let _ = Reflect::set(
                        &billboard_options,
                        &JsValue::from_str("pixelOffset"),
                        &JsValue::from(offset),
                    );
                }

                if let Some(value) = eye_offset.get() {
                    let eye_offset = dvec3_to_cartesian_dimensions(value);
                    let _ = Reflect::set(
                        &billboard_options,
                        &JsValue::from_str("eyeOffset"),
                        &JsValue::from(eye_offset),
                    );
                }

                if let Some(value) = show.get() {
                    let _ = Reflect::set(
                        &billboard_options,
                        &JsValue::from_str("show"),
                        &JsValue::from_bool(value),
                    );
                }

                let _ = Reflect::set(&entity, &JsValue::from_str("billboard"), &billboard_options);
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            image,
            scale,
            width,
            height,
            rotation,
            color,
            horizontal_origin,
            vertical_origin,
            pixel_offset,
            eye_offset,
            show,
        );
    }
}
