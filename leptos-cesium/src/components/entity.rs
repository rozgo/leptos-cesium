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
#[cfg(target_arch = "wasm32")]
use web_sys::console;

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
            console::debug_1(&JsValue::from_str("Entity: effect tick"));
            if entity_context.entity_untracked::<CesiumEntity>().is_some() {
                console::debug_1(&JsValue::from_str("Entity: entity already exists"));
                return;
            }

            viewer_context.with_viewer(|viewer: Viewer| {
                console::debug_1(&JsValue::from_str("Entity: creating entity"));
                let entities = viewer.entities();
                let entity_options = Object::new();

                // Set name if provided
                if let Some(n) = name.get() {
                    let _ = Reflect::set(
                        &entity_options,
                        &JsValue::from_str("name"),
                        &JsValue::from_str(&n),
                    );
                }

                // Set position if provided - convert DVec3 to Cartesian3
                if let Some(pos) = position.get() {
                    let cartesian: Cartesian3 = pos.into();
                    let _ = Reflect::set(
                        &entity_options,
                        &JsValue::from_str("position"),
                        &JsValue::from(cartesian),
                    );
                }

                // Set description if provided
                if let Some(desc) = description.get() {
                    let _ = Reflect::set(
                        &entity_options,
                        &JsValue::from_str("description"),
                        &JsValue::from_str(&desc),
                    );
                }

                // Set show if provided
                if let Some(s) = show.get() {
                    let _ = Reflect::set(
                        &entity_options,
                        &JsValue::from_str("show"),
                        &JsValue::from_bool(s),
                    );
                }

                let entity = entities.add_with_options(&entity_options.into());
                console::debug_1(&JsValue::from_str("Entity: entity created"));
                entity_context.set_entity(entity);
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (name, position, description, show);
        }
    });

    on_cleanup(move || {
        #[cfg(target_arch = "wasm32")]
        {
            console::debug_1(&JsValue::from_str("Entity: cleanup"));
            if let Some(entity) = entity_context.entity_untracked::<CesiumEntity>() {
                viewer_context.with_viewer(|viewer: Viewer| {
                    viewer.entities().remove(&entity);
                    console::debug_1(&JsValue::from_str("Entity: removed from viewer"));
                });
            }
        }
        entity_context.clear_entity();
    });

    view! { <>{children()}</> }
}
