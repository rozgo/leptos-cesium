//! Context types wiring Cesium state through the Leptos component tree.

use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

use crate::{
    cesium::{Entity, Viewer},
    core::{JsReadSignal, JsRwSignal, ThreadSafeJsValue},
};

/// Context exposing the active Cesium viewer to descendants.
#[derive(Debug, Clone, Copy)]
pub struct CesiumViewerContext {
    viewer: JsRwSignal<Option<ThreadSafeJsValue<JsValue>>>,
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
                return self
                    .entity
                    .get()
                    .and_then(|value| value.value().clone().dyn_into::<T>().ok());
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = std::marker::PhantomData::<T>;
                return None;
            }
        } else {
            leptos::logging::error!(
                "Accessing Cesium entity from a different thread. Probably running on the server."
            );
            return None;
        }
    }

    pub fn entity_untracked<T: JsCast>(&self) -> Option<T> {
        if self.is_valid() {
            #[cfg(target_arch = "wasm32")]
            {
                return self
                    .entity
                    .get_untracked()
                    .and_then(|value| value.value().clone().dyn_into::<T>().ok());
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = std::marker::PhantomData::<T>;
                return None;
            }
        } else {
            leptos::logging::error!(
                "Accessing Cesium entity from a different thread. Probably running on the server."
            );
            return None;
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
