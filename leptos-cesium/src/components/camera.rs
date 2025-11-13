//! Camera control components for declarative camera manipulation

use leptos::prelude::*;

use crate::core::JsSignal;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{
    BoundingSphere, Cartesian3, FlyToOptions, HeadingPitchRange, HeadingPitchRoll, SetViewOptions,
    Viewer, julian_date_now,
};
#[cfg(target_arch = "wasm32")]
use crate::components::use_cesium_context;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue};

#[cfg(not(target_arch = "wasm32"))]
use crate::bindings::{BoundingSphere, Cartesian3, HeadingPitchRange, HeadingPitchRoll};

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
///             destination=cartesian3_from_degrees(-116.52, 35.02, 95000.0)
///             orientation=Some(HeadingPitchRoll::new(6.0, -0.5, 0.0))
///         />
///     </ViewerContainer>
/// }
/// ```
#[component(transparent)]
pub fn CameraSetView(
    /// Camera destination (Cartesian3)
    #[prop(into)]
    destination: JsSignal<Cartesian3>,
    /// Camera orientation (heading, pitch, roll)
    #[prop(optional, into)]
    orientation: JsSignal<Option<HeadingPitchRoll>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraSetView must be inside ViewerContainer");

        Effect::new(move |_| {
            let dest = destination.get_untracked();
            let orient = orientation.get_untracked();

            viewer_context.with_viewer(|viewer: Viewer| {
                let mut options = SetViewOptions::new(dest);

                if let Some(o) = orient {
                    options = options.orientation(o);
                }

                viewer.camera().set_view(&options.build());
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (destination, orientation);
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
///             destination=cartesian3_from_degrees(-116.52, 35.02, 95000.0)
///             orientation=Some(HeadingPitchRoll::new(6.0, -0.5, 0.0))
///             duration=2.0
///         />
///     </ViewerContainer>
/// }
/// ```
#[component(transparent)]
pub fn CameraFlyTo(
    /// Camera destination (Cartesian3)
    #[prop(into)]
    destination: JsSignal<Cartesian3>,
    /// Camera orientation (heading, pitch, roll)
    #[prop(optional, into)]
    orientation: JsSignal<Option<HeadingPitchRoll>>,
    /// Duration of flight in seconds (default: 3.0)
    #[prop(optional, into, default = 3.0.into())]
    duration: Signal<f64>,
    /// Offset from destination (heading, pitch, range)
    #[prop(optional, into)]
    offset: JsSignal<Option<HeadingPitchRange>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraFlyTo must be inside ViewerContainer");

        Effect::new(move |_| {
            let dest = destination.get_untracked();
            let orient = orientation.get_untracked();
            let dur = duration.get();
            let off = offset.get_untracked();

            viewer_context.with_viewer(|viewer: Viewer| {
                let mut options = FlyToOptions::new(dest).duration(dur);

                if let Some(o) = orient {
                    options = options.orientation(o);
                }

                if let Some(offset_val) = off {
                    options = options.offset(offset_val);
                }

                viewer.camera().fly_to(&options.build());
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (destination, orientation, duration, offset);
    }
}

/// Camera fly to bounding sphere component for animated camera movement to fit an entity or target
///
/// This component flies the camera to a position where the entire bounding sphere is visible.
/// Useful for "zoom to entity" functionality.
///
/// # Example
///
/// ```rust,ignore
/// view! {
///     <ViewerContainer>
///         <CameraFlyToBoundingSphere
///             target=bounding_sphere_signal
///             offset=Some(HeadingPitchRange::new(0.0, -0.5, 0.0))
///             duration=2.0
///         />
///     </ViewerContainer>
/// }
/// ```
#[component(transparent)]
pub fn CameraFlyToBoundingSphere(
    /// Bounding sphere to fly to
    #[prop(into)]
    target: JsSignal<BoundingSphere>,
    /// Offset from the bounding sphere (heading, pitch, range)
    #[prop(optional, into)]
    offset: JsSignal<Option<HeadingPitchRange>>,
    /// Duration of flight in seconds (default: 3.0)
    #[prop(optional, into, default = 3.0.into())]
    duration: Signal<f64>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraFlyToBoundingSphere must be inside ViewerContainer");

        Effect::new(move |_| {
            let sphere = target.get_untracked();
            let off = offset.get_untracked();
            let dur = duration.get();

            viewer_context.with_viewer(|viewer: Viewer| {
                use js_sys::{Object, Reflect};

                let options = Object::new();

                // Set duration
                let _ = Reflect::set(
                    &options,
                    &JsValue::from_str("duration"),
                    &JsValue::from_f64(dur),
                );

                // Set offset if provided
                if let Some(offset_val) = off {
                    let _ = Reflect::set(
                        &options,
                        &JsValue::from_str("offset"),
                        &JsValue::from(offset_val),
                    );
                }

                // Call camera.flyToBoundingSphere(sphere, options)
                use js_sys::{Function, Reflect as JsReflect};
                let camera = viewer.camera();
                let fly_to_bs_fn =
                    JsReflect::get(&camera, &JsValue::from_str("flyToBoundingSphere"))
                        .expect("Camera.flyToBoundingSphere to exist");
                let fly_to_bs_fn: Function = fly_to_bs_fn
                    .dyn_into()
                    .expect("Camera.flyToBoundingSphere to be callable");

                let _ = fly_to_bs_fn.call2(&camera, &JsValue::from(sphere), &options.into());
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (target, offset, duration);
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
