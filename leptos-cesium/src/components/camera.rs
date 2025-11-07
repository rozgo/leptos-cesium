//! Camera control components for declarative camera manipulation

use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{Cartesian3, Viewer};
#[cfg(target_arch = "wasm32")]
use crate::components::use_cesium_context;
#[cfg(target_arch = "wasm32")]
use crate::core::JsSignal;
#[cfg(target_arch = "wasm32")]
use js_sys::{Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(not(target_arch = "wasm32"))]
use crate::core::JsSignal;

/// Camera fly home component that triggers camera to return to home view
///
/// # Example
///
/// ```rust,ignore
/// let (go_home, set_go_home) = create_signal(());
///
/// view! {
///     <ViewerContainer>
///         <CameraFlyHome trigger=go_home />
///         <button on:click=move |_| set_go_home(())>"Home"</button>
///     </ViewerContainer>
/// }
/// ```
#[component(transparent)]
pub fn CameraFlyHome(
    /// Trigger signal - camera flies home whenever this signal updates
    #[prop(into)]
    trigger: Signal<()>,
    /// Duration of flight in seconds (default: 0.0 = instant)
    #[prop(optional, into)]
    duration: Signal<f64>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraFlyHome must be inside ViewerContainer");

        Effect::new(move |_| {
            // Track the trigger signal
            trigger.get();

            let dur = duration.get();

            viewer_context.with_viewer(|viewer: Viewer| {
                viewer.camera().fly_home(dur);
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (trigger, duration);
    }
}

/// Camera set view component for declarative camera positioning
///
/// # Example
///
/// ```rust,ignore
/// view! {
///     <ViewerContainer>
///         <CameraSetView
///             destination=Some(cartesian3_from_degrees(-116.52, 35.02, 95000.0))
///             heading=Some(6.0)
///         />
///     </ViewerContainer>
/// }
/// ```
#[component(transparent)]
pub fn CameraSetView(
    /// Camera destination (Cartesian3)
    #[prop(optional, into)]
    destination: JsSignal<Option<Cartesian3>>,
    /// Heading in radians
    #[prop(optional, into)]
    heading: Signal<Option<f64>>,
    /// Pitch in radians
    #[prop(optional, into)]
    pitch: Signal<Option<f64>>,
    /// Roll in radians
    #[prop(optional, into)]
    roll: Signal<Option<f64>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraSetView must be inside ViewerContainer");

        Effect::new(move |_| {
            viewer_context.with_viewer(|viewer: Viewer| {
                let view_options = Object::new();

                // Set destination if provided
                if let Some(dest) = destination.get_untracked() {
                    let _ = Reflect::set(
                        &view_options,
                        &JsValue::from_str("destination"),
                        &JsValue::from(dest),
                    );
                }

                // Set orientation if any orientation prop is provided
                if heading.get().is_some() || pitch.get().is_some() || roll.get().is_some() {
                    let orientation = Object::new();

                    if let Some(h) = heading.get() {
                        let _ = Reflect::set(
                            &orientation,
                            &JsValue::from_str("heading"),
                            &JsValue::from_f64(h),
                        );
                    }
                    if let Some(p) = pitch.get() {
                        let _ = Reflect::set(
                            &orientation,
                            &JsValue::from_str("pitch"),
                            &JsValue::from_f64(p),
                        );
                    }
                    if let Some(r) = roll.get() {
                        let _ = Reflect::set(
                            &orientation,
                            &JsValue::from_str("roll"),
                            &JsValue::from_f64(r),
                        );
                    }

                    let _ = Reflect::set(
                        &view_options,
                        &JsValue::from_str("orientation"),
                        &orientation,
                    );
                }

                viewer.camera().set_view(&view_options.into());
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (destination, heading, pitch, roll);
    }
}
