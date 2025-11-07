//! Camera control components for declarative camera manipulation

use leptos::prelude::*;

use crate::core::JsSignal;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{julian_date_now, Cartesian3, Viewer};
#[cfg(target_arch = "wasm32")]
use crate::components::use_cesium_context;
#[cfg(target_arch = "wasm32")]
use js_sys::{Object, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(not(target_arch = "wasm32"))]
use crate::bindings::Cartesian3;

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

/// Camera fly to component for animated camera movement to a destination
///
/// # Example
///
/// ```rust,ignore
/// view! {
///     <ViewerContainer>
///         <CameraFlyTo
///             destination=Some(cartesian3_from_degrees(-116.52, 35.02, 95000.0))
///             heading=Some(6.0)
///             duration=2.0
///         />
///     </ViewerContainer>
/// }
/// ```
#[component(transparent)]
pub fn CameraFlyTo(
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
    /// Duration of flight in seconds (default: 3.0)
    #[prop(optional, into)]
    duration: Signal<f64>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraFlyTo must be inside ViewerContainer");

        Effect::new(move |_| {
            // Skip if no destination
            let Some(dest) = destination.get_untracked() else {
                return;
            };

            viewer_context.with_viewer(|viewer: Viewer| {
                let fly_options = Object::new();

                // Set destination
                let _ = Reflect::set(
                    &fly_options,
                    &JsValue::from_str("destination"),
                    &JsValue::from(dest),
                );

                // Set duration
                let _ = Reflect::set(
                    &fly_options,
                    &JsValue::from_str("duration"),
                    &JsValue::from_f64(duration.get()),
                );

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
                        &fly_options,
                        &JsValue::from_str("orientation"),
                        &orientation,
                    );
                }

                viewer.camera().fly_to(&fly_options.into());
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (destination, heading, pitch, roll, duration);
    }
}

/// Clock reset component to reset viewer clock to current time and stop animation
///
/// # Example
///
/// ```rust,ignore
/// let (reset_trigger, set_reset_trigger) = signal(());
///
/// view! {
///     <ViewerContainer>
///         <ClockReset trigger=reset_trigger />
///         <button on:click=move |_| set_reset_trigger(())>"Reset Clock"</button>
///     </ViewerContainer>
/// }
/// ```
#[component(transparent)]
pub fn ClockReset(
    /// Trigger signal - clock resets whenever this signal updates
    #[prop(into)]
    trigger: Signal<()>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("ClockReset must be inside ViewerContainer");

        Effect::new(move |_| {
            // Track the trigger signal
            trigger.get();

            viewer_context.with_viewer(|viewer: Viewer| {
                let clock = viewer.clock();
                // Reset to current time
                let now = julian_date_now();
                clock.set_current_time(&JsValue::from(now));
                // Stop animation
                clock.set_should_animate(false);
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = trigger;
    }
}
