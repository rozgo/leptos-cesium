//! Entity component placeholder until bindings and data model are implemented.

use leptos::prelude::*;

use crate::components::extend_context_with_entity;

/// Minimal stub for a Cesium entity component. Provides an entity context to children and clears
/// it on unmount.
#[component]
pub fn Entity(children: Children) -> impl IntoView {
    let entity_context = extend_context_with_entity();

    on_cleanup(move || {
        entity_context.clear_entity();
    });

    view! { <>{children()}</> }
}
