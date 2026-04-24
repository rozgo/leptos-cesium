//! Entity component for creating Cesium entities

use glam::DVec3;
use leptos::prelude::*;

use crate::components::extend_context_with_entity;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{Cartesian3, Entity as CesiumEntity, Viewer};
#[cfg(target_arch = "wasm32")]
use crate::components::use_cesium_context;
#[cfg(target_arch = "wasm32")]
use js_sys::{Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

/// Entity component for creating Cesium entities with graphics
#[component]
pub fn Entity(
    /// Optional entity name
    #[prop(optional, into)]
    name: Signal<Option<String>>,
    /// Optional position as DVec3 (x=longitude, y=latitude, z=height in degrees/meters)
    #[prop(optional, into)]
    position: Signal<Option<DVec3>>,
    /// Optional description
    #[prop(optional, into)]
    description: Signal<Option<String>>,
    /// Whether to show the entity
    #[prop(optional, into)]
    show: Signal<Option<bool>>,
    /// Child graphics components
    children: Children,
) -> impl IntoView {
    let entity_context = extend_context_with_entity();

    #[cfg(target_arch = "wasm32")]
    let viewer_context = use_cesium_context().expect("Entity must be inside ViewerContainer");

    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            if entity_context.entity_untracked::<CesiumEntity>().is_some() {
                return;
            }

            let Some(viewer) = viewer_context.viewer() else {
                return;
            };

            let entity = viewer.entities().add_with_options(&Object::new().into());
            entity_context.set_entity(entity);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (name, position, description, show);
        }
    });

    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let value = name.get();
            entity_context.with_entity(|entity: CesiumEntity| {
                set_optional_string_property(&entity, "name", value.as_deref());
            });
        }
    });

    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let value = position.get();
            entity_context.with_entity(|entity: CesiumEntity| {
                let value = value.map(|position| {
                    let cartesian: Cartesian3 = position.into();
                    JsValue::from(cartesian)
                });
                set_optional_js_property(&entity, "position", value.as_ref());
            });
        }
    });

    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let value = description.get();
            entity_context.with_entity(|entity: CesiumEntity| {
                set_optional_string_property(&entity, "description", value.as_deref());
            });
        }
    });

    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let value = show.get().unwrap_or(true);
            entity_context.with_entity(|entity: CesiumEntity| {
                let _ = Reflect::set(
                    &entity,
                    &JsValue::from_str("show"),
                    &JsValue::from_bool(value),
                );
            });
        }
    });

    on_cleanup(move || {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(entity) = entity_context.entity_untracked::<CesiumEntity>() {
                viewer_context.with_viewer(|viewer: Viewer| {
                    viewer.entities().remove(&entity);
                });
            }
        }
        entity_context.clear_entity();
    });

    view! { <>{children()}</> }
}

#[cfg(target_arch = "wasm32")]
fn set_optional_string_property(entity: &CesiumEntity, property: &str, value: Option<&str>) {
    match value {
        Some(value) => {
            let _ = Reflect::set(
                entity,
                &JsValue::from_str(property),
                &JsValue::from_str(value),
            );
        }
        None => {
            let _ = Reflect::set(entity, &JsValue::from_str(property), &JsValue::UNDEFINED);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn set_optional_js_property(entity: &CesiumEntity, property: &str, value: Option<&JsValue>) {
    let _ = Reflect::set(
        entity,
        &JsValue::from_str(property),
        value.unwrap_or(&JsValue::UNDEFINED),
    );
}
