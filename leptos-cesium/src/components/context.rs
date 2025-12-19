//! Context types wiring Cesium state through the Leptos component tree.

use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

#[cfg(not(feature = "ssr"))]
use crate::core::JsRwSignal;
use crate::{
    bindings::Entity,
    cesium::Viewer,
    core::{JsReadSignal, ThreadSafeJsValue},
};

/// Context exposing the active Cesium viewer to descendants.
#[derive(Debug, Clone, Copy)]
pub struct CesiumViewerContext {
    #[cfg(not(feature = "ssr"))]
    viewer: JsRwSignal<Option<ThreadSafeJsValue<JsValue>>>,
    #[cfg(not(feature = "ssr"))]
    selected_entity: JsRwSignal<Option<ThreadSafeJsValue<JsValue>>>,
    /// Reactive trigger that increments when selection changes
    /// Use this to trigger reactivity in components
    #[cfg(not(feature = "ssr"))]
    selection_version: RwSignal<usize>,
    #[cfg(not(feature = "ssr"))]
    thread_id: std::thread::ThreadId,
    #[cfg(feature = "ssr")]
    _phantom: std::marker::PhantomData<()>,
}

impl CesiumViewerContext {
    /// Create a fresh viewer context.
    pub fn new() -> Self {
        #[cfg(not(feature = "ssr"))]
        return Self {
            viewer: JsRwSignal::new_local(None),
            selected_entity: JsRwSignal::new_local(None),
            selection_version: RwSignal::new(0),
            thread_id: std::thread::current().id(),
        };
        #[cfg(feature = "ssr")]
        Self {
            _phantom: std::marker::PhantomData,
        }
    }

    /// Record the viewer instance in the context.
    #[cfg(not(feature = "ssr"))]
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

    #[cfg(feature = "ssr")]
    pub fn set_viewer(&self, viewer: Viewer) {
        let _ = viewer;
    }

    /// Returns the viewer, cloning the underlying JS handle if this is the correct thread.
    #[cfg(not(feature = "ssr"))]
    pub fn viewer(&self) -> Option<Viewer> {
        self.viewer
            .get()
            .map(|value| value.value().clone().unchecked_into::<Viewer>())
    }

    #[cfg(feature = "ssr")]
    pub fn viewer(&self) -> Option<Viewer> {
        None
    }

    /// Returns the viewer without tracking reactive dependencies.
    #[cfg(not(feature = "ssr"))]
    pub fn viewer_untracked(&self) -> Option<Viewer> {
        self.viewer
            .get_untracked()
            .map(|value| value.value().clone().unchecked_into::<Viewer>())
    }

    #[cfg(feature = "ssr")]
    pub fn viewer_untracked(&self) -> Option<Viewer> {
        None
    }

    /// Returns a read-only signal for the viewer.
    #[cfg(not(feature = "ssr"))]
    pub fn viewer_signal(&self) -> JsReadSignal<Option<ThreadSafeJsValue<JsValue>>> {
        if self.is_valid() {
            self.viewer.read_only()
        } else {
            panic!(
                "Accessing Cesium viewer from a different thread. Probably running on the server."
            );
        }
    }

    #[cfg(feature = "ssr")]
    pub fn viewer_signal(&self) -> JsReadSignal<Option<ThreadSafeJsValue<JsValue>>> {
        panic!("viewer_signal() is not available during SSR");
    }

    /// Clears the viewer from the context.
    #[cfg(not(feature = "ssr"))]
    pub fn clear_viewer(&self) {
        if self.is_valid() {
            self.viewer.set(None);
        }
    }

    #[cfg(feature = "ssr")]
    pub fn clear_viewer(&self) {
        // No-op during SSR
    }

    /// Executes a closure with the viewer reference if it is available on this thread.
    #[cfg(not(feature = "ssr"))]
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

    #[cfg(feature = "ssr")]
    pub fn with_viewer<F, R>(&self, _f: F) -> Option<R>
    where
        F: FnOnce(Viewer) -> R,
    {
        None
    }

    /// Set the selected entity (strongly-typed).
    #[cfg(not(feature = "ssr"))]
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

    #[cfg(feature = "ssr")]
    pub fn set_selected_entity(&self, entity: Option<Entity>) {
        let _ = entity;
    }

    /// Set the selected entity from a JsValue (used internally by event listener).
    #[cfg(not(feature = "ssr"))]
    #[allow(dead_code)] // Called from wasm32-only code in viewer_container.rs
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

    #[cfg(feature = "ssr")]
    #[allow(dead_code)]
    pub(crate) fn set_selected_entity_from_js(&self, entity: JsValue) {
        let _ = entity;
    }

    /// Returns the selected entity (strongly-typed as Entity).
    #[cfg(not(feature = "ssr"))]
    pub fn selected_entity(&self) -> Option<Entity> {
        self.selected_entity
            .get()
            .and_then(|value| value.value().clone().dyn_into::<Entity>().ok())
    }

    #[cfg(feature = "ssr")]
    pub fn selected_entity(&self) -> Option<Entity> {
        None
    }

    /// Returns the selected entity without tracking reactive dependencies.
    #[cfg(not(feature = "ssr"))]
    pub fn selected_entity_untracked(&self) -> Option<Entity> {
        self.selected_entity
            .get_untracked()
            .and_then(|value| value.value().clone().dyn_into::<Entity>().ok())
    }

    #[cfg(feature = "ssr")]
    pub fn selected_entity_untracked(&self) -> Option<Entity> {
        None
    }

    /// Returns the selected entity as a specific type (for advanced use cases).
    #[cfg(not(feature = "ssr"))]
    pub fn selected_entity_as<T: JsCast>(&self) -> Option<T> {
        self.selected_entity
            .get()
            .and_then(|value| value.value().clone().dyn_into::<T>().ok())
    }

    #[cfg(feature = "ssr")]
    pub fn selected_entity_as<T: JsCast>(&self) -> Option<T> {
        let _ = std::marker::PhantomData::<T>;
        None
    }

    /// Returns a read-only signal for the selected entity.
    #[cfg(not(feature = "ssr"))]
    pub fn selected_entity_signal(&self) -> JsReadSignal<Option<ThreadSafeJsValue<JsValue>>> {
        if self.is_valid() {
            self.selected_entity.read_only()
        } else {
            panic!(
                "Accessing Cesium viewer from a different thread. Probably running on the server."
            );
        }
    }

    #[cfg(feature = "ssr")]
    pub fn selected_entity_signal(&self) -> JsReadSignal<Option<ThreadSafeJsValue<JsValue>>> {
        panic!("selected_entity_signal() is not available during SSR");
    }

    /// Clear the selected entity.
    #[cfg(not(feature = "ssr"))]
    pub fn clear_selected_entity(&self) {
        if self.is_valid() {
            self.selected_entity.set(None);
            self.selection_version.update(|v| *v += 1);
        }
    }

    #[cfg(feature = "ssr")]
    pub fn clear_selected_entity(&self) {
        // No-op during SSR
    }

    /// Returns a reactive signal that updates when selection changes.
    /// Use this to trigger reactivity, then call selected_entity() to get the entity.
    #[cfg(not(feature = "ssr"))]
    pub fn selection_version(&self) -> ReadSignal<usize> {
        self.selection_version.read_only()
    }

    #[cfg(feature = "ssr")]
    pub fn selection_version(&self) -> ReadSignal<usize> {
        panic!("selection_version() is not available during SSR");
    }

    #[cfg(not(feature = "ssr"))]
    fn is_valid(&self) -> bool {
        std::thread::current().id() == self.thread_id && !self.viewer.is_disposed()
    }

    #[cfg(feature = "ssr")]
    #[allow(dead_code)] // Used by hydrate-gated methods
    fn is_valid(&self) -> bool {
        false
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
    #[cfg(not(feature = "ssr"))]
    entity: JsRwSignal<Option<ThreadSafeJsValue<JsValue>>>,
    #[cfg(not(feature = "ssr"))]
    thread_id: std::thread::ThreadId,
    #[cfg(feature = "ssr")]
    _phantom: std::marker::PhantomData<()>,
}

impl CesiumEntityContext {
    pub fn new() -> Self {
        #[cfg(not(feature = "ssr"))]
        return Self {
            entity: JsRwSignal::new_local(None),
            thread_id: std::thread::current().id(),
        };
        #[cfg(feature = "ssr")]
        Self {
            _phantom: std::marker::PhantomData,
        }
    }

    #[cfg(not(feature = "ssr"))]
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

    #[cfg(feature = "ssr")]
    pub fn set_entity(&self, entity: Entity) {
        let _ = entity;
    }

    #[cfg(not(feature = "ssr"))]
    pub fn entity<T: JsCast>(&self) -> Option<T> {
        if self.is_valid() {
            self.entity
                .get()
                .and_then(|value| value.value().clone().dyn_into::<T>().ok())
        } else {
            leptos::logging::error!(
                "Accessing Cesium entity from a different thread. Probably running on the server."
            );
            None
        }
    }

    #[cfg(feature = "ssr")]
    pub fn entity<T: JsCast>(&self) -> Option<T> {
        let _ = std::marker::PhantomData::<T>;
        None
    }

    #[cfg(not(feature = "ssr"))]
    pub fn entity_untracked<T: JsCast>(&self) -> Option<T> {
        if self.is_valid() {
            self.entity
                .get_untracked()
                .and_then(|value| value.value().clone().dyn_into::<T>().ok())
        } else {
            leptos::logging::error!(
                "Accessing Cesium entity from a different thread. Probably running on the server."
            );
            None
        }
    }

    #[cfg(feature = "ssr")]
    pub fn entity_untracked<T: JsCast>(&self) -> Option<T> {
        let _ = std::marker::PhantomData::<T>;
        None
    }

    /// Clears the entity reference from the context.
    #[cfg(not(feature = "ssr"))]
    pub fn clear_entity(&self) {
        if self.is_valid() {
            self.entity.set(None);
        }
    }

    #[cfg(feature = "ssr")]
    pub fn clear_entity(&self) {
        // No-op during SSR
    }

    /// Executes a closure with the entity reference if it can be cast to `T`.
    pub fn with_entity<T, F, R>(&self, f: F) -> Option<R>
    where
        T: JsCast,
        F: FnOnce(T) -> R,
    {
        self.entity::<T>().map(f)
    }

    #[cfg(not(feature = "ssr"))]
    fn is_valid(&self) -> bool {
        std::thread::current().id() == self.thread_id && !self.entity.is_disposed()
    }

    #[cfg(feature = "ssr")]
    #[allow(dead_code)] // Used by hydrate-gated methods
    fn is_valid(&self) -> bool {
        false
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
