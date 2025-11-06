//! Context types wiring Cesium state through the Leptos component tree.

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::{
    cesium::{Entity, Viewer},
    core::{JsReadSignal, JsRwSignal, ThreadSafeJsValue},
};

#[cfg(target_arch = "wasm32")]
use crate::core::IntoThreadSafeJsValue;

/// Context exposing the active Cesium viewer to descendants.
#[derive(Debug, Clone, Copy)]
pub struct CesiumViewerContext {
    viewer: JsRwSignal<Option<ThreadSafeJsValue<Viewer>>>,
    thread_id: std::thread::ThreadId,
}

impl CesiumViewerContext {
    /// Create a fresh viewer context.
    pub fn new() -> Self {
        Self {
            viewer: JsRwSignal::new_local(None),
            thread_id: std::thread::current().id(),
        }
    }

    /// Record the viewer instance in the context.
    pub fn set_viewer(&self, viewer: &Viewer) {
        if !self.is_valid() {
            leptos::logging::error!(
                "Accessing Cesium viewer from a different thread. Probably running on the server."
            );
            return;
        }
        #[cfg(target_arch = "wasm32")]
        self.viewer
            .set(Some(viewer.clone().into_thread_safe_js_value()));
        #[cfg(not(target_arch = "wasm32"))]
        let _ = viewer;
    }

    /// Returns the viewer, cloning the underlying JS handle if this is the correct thread.
    pub fn viewer(&self) -> Option<ThreadSafeJsValue<Viewer>> {
        if self.is_valid() {
            self.viewer.get()
        } else {
            leptos::logging::error!(
                "Accessing Cesium viewer from a different thread. Probably running on the server."
            );
            None
        }
    }

    /// Returns the viewer without tracking reactive dependencies.
    pub fn viewer_untracked(&self) -> Option<ThreadSafeJsValue<Viewer>> {
        if self.is_valid() {
            self.viewer.get_untracked()
        } else {
            leptos::logging::error!(
                "Accessing Cesium viewer from a different thread. Probably running on the server."
            );
            None
        }
    }

    /// Returns a read-only signal for the viewer.
    pub fn viewer_signal(&self) -> JsReadSignal<Option<ThreadSafeJsValue<Viewer>>> {
        if self.is_valid() {
            self.viewer.read_only()
        } else {
            panic!("Accessing Cesium viewer from a different thread. Probably running on the server.");
        }
    }

    /// Clears the viewer from the context.
    pub fn clear_viewer(&self) {
        if self.is_valid() {
            self.viewer.set(None);
        }
    }

    /// Executes a closure with the viewer reference if it is available on this thread.
    pub fn with_viewer<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Viewer) -> R,
    {
        let viewer_ts = self.viewer_untracked()?;
        Some(f(viewer_ts.value()))
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
    entity: JsRwSignal<Option<ThreadSafeJsValue<Entity>>>,
    thread_id: std::thread::ThreadId,
}

impl CesiumEntityContext {
    pub fn new() -> Self {
        Self {
            entity: JsRwSignal::new_local(None),
            thread_id: std::thread::current().id(),
        }
    }

    pub fn set_entity(&self, entity: &Entity) {
        if !self.is_valid() {
            leptos::logging::error!(
                "Accessing Cesium entity from a different thread. Probably running on the server."
            );
            return;
        }
        #[cfg(target_arch = "wasm32")]
        self.entity
            .set(Some(entity.clone().into_thread_safe_js_value()));
        #[cfg(not(target_arch = "wasm32"))]
        let _ = entity;
    }

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
