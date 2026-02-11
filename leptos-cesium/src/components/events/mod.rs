//! Event builders for Cesium viewer and entities.

#[cfg(target_arch = "wasm32")]
use crate::bindings::Event;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

mod viewer_events;

pub use viewer_events::*;

/// A single bound Cesium event listener.
#[cfg(target_arch = "wasm32")]
pub struct CesiumBoundListener {
    event: Event,
    callback: Closure<dyn FnMut(JsValue)>,
}

#[cfg(target_arch = "wasm32")]
impl CesiumBoundListener {
    pub fn new(event: Event, callback: Closure<dyn FnMut(JsValue)>) -> Self {
        Self { event, callback }
    }

    pub fn detach(self) {
        self.event
            .remove_event_listener(self.callback.as_ref().unchecked_ref());
    }
}

/// Build typed Cesium event handlers that can be attached to a Cesium target and detached on cleanup.
///
/// Example:
///
/// ```rust,ignore
/// cesium_events!(
///     (ViewerEvents, Viewer),
///     (selected_entity_changed, selected_entity_changed, wasm_bindgen::JsValue),
///     (tracked_entity_changed, tracked_entity_changed, wasm_bindgen::JsValue),
/// );
/// ```
#[macro_export]
macro_rules! cesium_events {
    (($t:ident, $target:ty), $(($rust:ident, $getter:ident, $b:ty)),+ $(,)?) => {
        $crate::paste! {
            use leptos::prelude::*;

            #[derive(Clone, Default)]
            pub struct $t {
                inner: StoredValue<[<Inner $t>], LocalStorage>,
            }

            #[derive(Default)]
            struct [<Inner $t>] {
                $(
                    [<$rust _handler>]: Option<Box<dyn FnMut($b)>>,
                )+
                #[cfg(target_arch = "wasm32")]
                listeners: Vec<$crate::components::events::CesiumBoundListener>,
            }

            impl $t {
                pub fn new() -> Self {
                    Self {
                        inner: StoredValue::new_local(Default::default()),
                    }
                }

                /// Attach currently configured handlers to the target.
                ///
                /// Calling `setup` again first detaches previously attached handlers for this builder.
                pub fn setup(&self, target: &$target) {
                    self.inner.update_value(|inner| {
                        #[cfg(target_arch = "wasm32")]
                        {
                            for listener in inner.listeners.drain(..) {
                                listener.detach();
                            }
                        }

                        $(
                            if let Some(handler) = inner.[<$rust _handler>].take() {
                                #[cfg(target_arch = "wasm32")]
                                {
                                    use wasm_bindgen::JsCast;

                                    let event = target.$getter();
                                    let mut handler = handler;
                                    let callback = wasm_bindgen::closure::Closure::wrap(
                                        Box::new(move |value: wasm_bindgen::JsValue| {
                                            if let Ok(arg) = value.dyn_into::<$b>() {
                                                handler(arg);
                                            }
                                        }) as Box<dyn FnMut(wasm_bindgen::JsValue)>
                                    );

                                    event.add_event_listener(callback.as_ref().unchecked_ref());
                                    inner.listeners.push(
                                        $crate::components::events::CesiumBoundListener::new(event, callback)
                                    );
                                }
                                #[cfg(not(target_arch = "wasm32"))]
                                {
                                    let _ = handler;
                                    let _ = target;
                                }
                            }
                        )+
                    });
                }

                /// Detach all listeners currently attached by this builder.
                pub fn teardown(&self) {
                    self.inner.update_value(|inner| {
                        #[cfg(target_arch = "wasm32")]
                        {
                            for listener in inner.listeners.drain(..) {
                                listener.detach();
                            }
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = inner;
                        }
                    });
                }

                $(
                #[must_use = "This method returns the event handler list, which must be stored in a variable to be used later."]
                pub fn [<set_ $rust>](self, handler: impl FnMut($b) + 'static) -> Self {
                    self.inner
                        .update_value(|v| v.[<$rust _handler>] = Some(Box::new(handler)));
                    self
                }
                )+
            }
        }
    };
}

pub use cesium_events;
