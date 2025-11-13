//! Context types wiring Cesium state through the Leptos component tree.

use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

use crate::{
    bindings::Entity,
    cesium::Viewer,
    core::{JsReadSignal, JsRwSignal, ThreadSafeJsValue},
};

/// Context exposing the active Cesium viewer to descendants.
#[derive(Debug, Clone, Copy)]
pub struct CesiumViewerContext {
    viewer: JsRwSignal<Option<ThreadSafeJsValue<JsValue>>>,
    selected_entity: JsRwSignal<Option<ThreadSafeJsValue<JsValue>>>,
    /// Reactive trigger that increments when selection changes
    /// Use this to trigger reactivity in components
    selection_version: RwSignal<usize>,
    thread_id: std::thread::ThreadId,
}

impl CesiumViewerContext {
    /// Create a fresh viewer context.
    pub fn new() -> Self {
        Self {
            viewer: JsRwSignal::new_local(None),
            selected_entity: JsRwSignal::new_local(None),
            selection_version: RwSignal::new(0),
            thread_id: std::thread::current().id(),
        }
    }

    /// Record the viewer instance in the context.
    #[cfg(target_arch = "wasm32")]
    pub fn set_viewer(&self, viewer: Viewer) {
        if !self.is_valid() {
            leptos::logging::error!(
                "Accessing Cesium viewer from a different thread. Probably running on the server."
            );
            return;
        }
        let value: JsValue = viewer.into();
        self.viewer.set(Some(ThreadSafeJsValue::new(value)));
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_viewer(&self, viewer: Viewer) {
        let _ = viewer;
    }

    /// Returns the viewer, cloning the underlying JS handle if this is the correct thread.
    #[cfg(target_arch = "wasm32")]
    pub fn viewer(&self) -> Option<Viewer> {
        self.viewer
            .get()
            .map(|value| value.value().clone().unchecked_into::<Viewer>())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn viewer(&self) -> Option<Viewer> {
        None
    }

    /// Returns the viewer without tracking reactive dependencies.
    #[cfg(target_arch = "wasm32")]
    pub fn viewer_untracked(&self) -> Option<Viewer> {
        self.viewer
            .get_untracked()
            .map(|value| value.value().clone().unchecked_into::<Viewer>())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn viewer_untracked(&self) -> Option<Viewer> {
        None
    }

    /// Returns a read-only signal for the viewer.
    pub fn viewer_signal(&self) -> JsReadSignal<Option<ThreadSafeJsValue<JsValue>>> {
        if self.is_valid() {
            self.viewer.read_only()
        } else {
            panic!(
                "Accessing Cesium viewer from a different thread. Probably running on the server."
            );
        }
    }

    /// Clears the viewer from the context.
    pub fn clear_viewer(&self) {
        if self.is_valid() {
            self.viewer.set(None);
        }
    }

    /// Executes a closure with the viewer reference if it is available on this thread.
    #[cfg(target_arch = "wasm32")]
    pub fn with_viewer<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(Viewer) -> R,
    {
        let viewer = self
            .viewer
            .get_untracked()
            .map(|value| value.value().clone().unchecked_into::<Viewer>())?;
        Some(f(viewer))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn with_viewer<F, R>(&self, _f: F) -> Option<R>
    where
        F: FnOnce(Viewer) -> R,
    {
        None
    }

    /// Set the selected entity (strongly-typed).
    #[cfg(target_arch = "wasm32")]
    pub fn set_selected_entity(&self, entity: Option<Entity>) {
        if !self.is_valid() {
            leptos::logging::error!(
                "Accessing Cesium viewer from a different thread. Probably running on the server."
            );
            return;
        }
        if let Some(e) = entity {
            let value: JsValue = e.into();
            self.selected_entity
                .set(Some(ThreadSafeJsValue::new(value)));
        } else {
            self.selected_entity.set(None);
        }
        // Increment version to trigger reactivity
        self.selection_version.update(|v| *v += 1);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_selected_entity(&self, entity: Option<Entity>) {
        let _ = entity;
    }

    /// Set the selected entity from a JsValue (used internally by event listener).
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn set_selected_entity_from_js(&self, entity: JsValue) {
        if !self.is_valid() {
            leptos::logging::error!(
                "Accessing Cesium viewer from a different thread. Probably running on the server."
            );
            return;
        }
        if entity.is_undefined() || entity.is_null() {
            self.selected_entity.set(None);
        } else {
            self.selected_entity
                .set(Some(ThreadSafeJsValue::new(entity)));
        }
        // Increment version to trigger reactivity
        self.selection_version.update(|v| *v += 1);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn set_selected_entity_from_js(&self, entity: JsValue) {
        let _ = entity;
    }

    /// Returns the selected entity (strongly-typed as Entity).
    #[cfg(target_arch = "wasm32")]
    pub fn selected_entity(&self) -> Option<Entity> {
        self.selected_entity
            .get()
            .and_then(|value| value.value().clone().dyn_into::<Entity>().ok())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn selected_entity(&self) -> Option<Entity> {
        None
    }

    /// Returns the selected entity without tracking reactive dependencies.
    #[cfg(target_arch = "wasm32")]
    pub fn selected_entity_untracked(&self) -> Option<Entity> {
        self.selected_entity
            .get_untracked()
            .and_then(|value| value.value().clone().dyn_into::<Entity>().ok())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn selected_entity_untracked(&self) -> Option<Entity> {
        None
    }

    /// Returns the selected entity as a specific type (for advanced use cases).
    #[cfg(target_arch = "wasm32")]
    pub fn selected_entity_as<T: JsCast>(&self) -> Option<T> {
        self.selected_entity
            .get()
            .and_then(|value| value.value().clone().dyn_into::<T>().ok())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn selected_entity_as<T: JsCast>(&self) -> Option<T> {
        let _ = std::marker::PhantomData::<T>;
        None
    }

    /// Returns a read-only signal for the selected entity.
    pub fn selected_entity_signal(&self) -> JsReadSignal<Option<ThreadSafeJsValue<JsValue>>> {
        if self.is_valid() {
            self.selected_entity.read_only()
        } else {
            panic!(
                "Accessing Cesium viewer from a different thread. Probably running on the server."
            );
        }
    }

    /// Clear the selected entity.
    pub fn clear_selected_entity(&self) {
        if self.is_valid() {
            self.selected_entity.set(None);
            self.selection_version.update(|v| *v += 1);
        }
    }

    /// Returns a reactive signal that updates when selection changes.
    /// Use this to trigger reactivity, then call selected_entity() to get the entity.
    pub fn selection_version(&self) -> ReadSignal<usize> {
        self.selection_version.read_only()
    }

    fn is_valid(&self) -> bool {
        std::thread::current().id() == self.thread_id && !self.viewer.is_disposed()
    }
}

impl Default for CesiumViewerContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Provide a fresh Cesium viewer context to the component tree.
pub fn provide_cesium_context() -> CesiumViewerContext {
    let context = CesiumViewerContext::new();
    provide_context(context);
    context
}

/// Retrieve the viewer context if one exists.
pub fn use_cesium_context() -> Option<CesiumViewerContext> {
    use_context::<CesiumViewerContext>()
}

/// Context exposing entity handles within a viewer.
#[derive(Debug, Clone, Copy)]
pub struct CesiumEntityContext {
    entity: JsRwSignal<Option<ThreadSafeJsValue<JsValue>>>,
    thread_id: std::thread::ThreadId,
}

impl CesiumEntityContext {
    pub fn new() -> Self {
        Self {
            entity: JsRwSignal::new_local(None),
            thread_id: std::thread::current().id(),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn set_entity(&self, entity: Entity) {
        if !self.is_valid() {
            leptos::logging::error!(
                "Accessing Cesium entity from a different thread. Probably running on the server."
            );
            return;
        }
        let value: JsValue = entity.into();
        self.entity.set(Some(ThreadSafeJsValue::new(value)));
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_entity(&self, entity: Entity) {
        let _ = entity;
    }

    pub fn entity<T: JsCast>(&self) -> Option<T> {
        if self.is_valid() {
            #[cfg(target_arch = "wasm32")]
            {
                self.entity
                    .get()
                    .and_then(|value| value.value().clone().dyn_into::<T>().ok())
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = std::marker::PhantomData::<T>;
                None
            }
        } else {
            leptos::logging::error!(
                "Accessing Cesium entity from a different thread. Probably running on the server."
            );
            None
        }
    }

    pub fn entity_untracked<T: JsCast>(&self) -> Option<T> {
        if self.is_valid() {
            #[cfg(target_arch = "wasm32")]
            {
                self.entity
                    .get_untracked()
                    .and_then(|value| value.value().clone().dyn_into::<T>().ok())
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = std::marker::PhantomData::<T>;
                None
            }
        } else {
            leptos::logging::error!(
                "Accessing Cesium entity from a different thread. Probably running on the server."
            );
            None
        }
    }

    /// Clears the entity reference from the context.
    pub fn clear_entity(&self) {
        if self.is_valid() {
            self.entity.set(None);
        }
    }

    /// Executes a closure with the entity reference if it can be cast to `T`.
    pub fn with_entity<T, F, R>(&self, f: F) -> Option<R>
    where
        T: JsCast,
        F: FnOnce(T) -> R,
    {
        self.entity::<T>().map(f)
    }

    fn is_valid(&self) -> bool {
        std::thread::current().id() == self.thread_id && !self.entity.is_disposed()
    }
}

impl Default for CesiumEntityContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Create and provide an entity context, useful for nested entity components.
pub fn extend_context_with_entity() -> CesiumEntityContext {
    let context = CesiumEntityContext::new();
    provide_context(context);
    context
}

/// Retrieve the current entity context.
pub fn use_entity_context() -> Option<CesiumEntityContext> {
    use_context::<CesiumEntityContext>()
}
