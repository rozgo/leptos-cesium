//! Camera control components for declarative camera manipulation.

use geo_types::Rect;
use glam::DVec3;
use leptos::prelude::*;

use crate::core::JsSignal;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{
    BoundingSphere, CameraDestination as BindingCameraDestination,
    CameraOrientation as BindingCameraOrientation, Cartesian3, DirectionUp,
    FlyToBoundingSphereOptions, FlyToOptions, HeadingPitchRange, Matrix4, SetViewOptions, Viewer,
    julian_date_now,
};
#[cfg(target_arch = "wasm32")]
use crate::components::use_cesium_context;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(not(target_arch = "wasm32"))]
use crate::bindings::{BoundingSphere, Matrix4};

/// Destination accepted by CameraSetView/CameraFlyTo.
#[derive(Clone)]
pub enum CameraDestination {
    /// Longitude/latitude/height in degrees/meters.
    Degrees(DVec3),
    /// Cesium world Cartesian coordinates in meters.
    Cartesian(DVec3),
    /// Rectangle destination in degrees.
    Rectangle(Rect<f64>),
}

impl From<DVec3> for CameraDestination {
    fn from(value: DVec3) -> Self {
        Self::Degrees(value)
    }
}

impl From<(f64, f64, f64)> for CameraDestination {
    fn from(value: (f64, f64, f64)) -> Self {
        Self::Cartesian(DVec3::new(value.0, value.1, value.2))
    }
}

impl From<Rect<f64>> for CameraDestination {
    fn from(value: Rect<f64>) -> Self {
        Self::Rectangle(value)
    }
}

/// Orientation accepted by CameraSetView/CameraFlyTo.
#[derive(Clone, Debug, PartialEq)]
pub enum CameraOrientation {
    /// Heading/pitch/roll in radians.
    HeadingPitchRoll(f64, f64, f64),
    /// Direction/up vectors.
    DirectionUp(DVec3, DVec3),
}

impl From<(f64, f64, f64)> for CameraOrientation {
    fn from(value: (f64, f64, f64)) -> Self {
        Self::HeadingPitchRoll(value.0, value.1, value.2)
    }
}

/// Offset accepted by CameraLookAt/CameraLookAtTransform.
#[derive(Clone, Debug, PartialEq)]
pub enum LookAtOffset {
    /// Heading/pitch/range offset.
    HeadingPitchRange(f64, f64, f64),
    /// Cartesian offset vector.
    CartesianOffset(DVec3),
}

impl From<(f64, f64, f64)> for LookAtOffset {
    fn from(value: (f64, f64, f64)) -> Self {
        Self::HeadingPitchRange(value.0, value.1, value.2)
    }
}

impl From<DVec3> for LookAtOffset {
    fn from(value: DVec3) -> Self {
        Self::CartesianOffset(value)
    }
}

/// Move direction for CameraMove.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CameraMoveDirection {
    #[default]
    Forward,
    Backward,
    Up,
    Down,
    Right,
    Left,
    /// Move along custom axis.
    AlongAxis,
}

/// Zoom direction for CameraZoom.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CameraZoomDirection {
    #[default]
    In,
    Out,
}

#[cfg(target_arch = "wasm32")]
fn destination_to_binding(destination: CameraDestination) -> BindingCameraDestination {
    match destination {
        CameraDestination::Degrees(v) => {
            BindingCameraDestination::from(Cartesian3::from_degrees(v.x, v.y, v.z))
        }
        CameraDestination::Cartesian(v) => {
            BindingCameraDestination::from(Cartesian3::new(v.x, v.y, v.z))
        }
        CameraDestination::Rectangle(rect) => {
            let min = rect.min();
            let max = rect.max();
            BindingCameraDestination::from(crate::bindings::Rectangle::from_degrees(
                min.x, min.y, max.x, max.y,
            ))
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn orientation_to_binding(orientation: CameraOrientation) -> BindingCameraOrientation {
    match orientation {
        CameraOrientation::HeadingPitchRoll(heading, pitch, roll) => {
            BindingCameraOrientation::from(crate::bindings::HeadingPitchRoll::new(
                heading, pitch, roll,
            ))
        }
        CameraOrientation::DirectionUp(direction, up) => {
            BindingCameraOrientation::from(DirectionUp::new(
                Cartesian3::new(direction.x, direction.y, direction.z),
                Cartesian3::new(up.x, up.y, up.z),
            ))
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn look_at_offset_to_js(offset: LookAtOffset) -> JsValue {
    match offset {
        LookAtOffset::HeadingPitchRange(heading, pitch, range) => {
            JsValue::from(HeadingPitchRange::new(heading, pitch, range))
        }
        LookAtOffset::CartesianOffset(v) => JsValue::from(Cartesian3::new(v.x, v.y, v.z)),
    }
}

/// Fly to Cesium home view when `trigger` updates.
#[component(transparent)]
pub fn CameraFlyHome(
    #[prop(into)] trigger: Signal<()>,
    #[prop(optional, into)] duration: Signal<Option<f64>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraFlyHome must be inside ViewerContainer");
        let mut is_first_run = true;

        Effect::new(move |_| {
            trigger.get();
            if is_first_run {
                is_first_run = false;
                return;
            }
            let duration = duration.get_untracked();

            viewer_context.with_viewer(|viewer: Viewer| {
                if let Some(duration) = duration {
                    viewer.camera().fly_home(duration);
                } else {
                    viewer.camera().fly_home_default();
                }
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (trigger, duration);
    }
}

/// Set camera view using Cesium `Camera.setView`.
#[component(transparent)]
pub fn CameraSetView(
    #[prop(optional, into)] destination: Signal<Option<CameraDestination>>,
    #[prop(optional, into)] orientation: Signal<Option<CameraOrientation>>,
    #[prop(optional, into)] end_transform: JsSignal<Option<Matrix4>>,
    #[prop(optional, into)] convert: Signal<Option<bool>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraSetView must be inside ViewerContainer");

        Effect::new(move |_| {
            let destination = destination.get();
            let orientation = orientation.get();
            let end_transform = end_transform.get();
            let convert = convert.get();

            viewer_context.with_viewer(|viewer: Viewer| {
                let mut options = SetViewOptions::new();

                if let Some(destination) = destination {
                    options = options.destination(destination_to_binding(destination));
                }
                if let Some(orientation) = orientation {
                    options = options.orientation(orientation_to_binding(orientation));
                }
                if let Some(end_transform) = end_transform {
                    options = options.end_transform(end_transform);
                }
                if let Some(convert) = convert {
                    options = options.convert(convert);
                }

                viewer.camera().set_view(&options.build());
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (destination, orientation, end_transform, convert);
    }
}

/// Fly camera using Cesium `Camera.flyTo`.
#[component(transparent)]
pub fn CameraFlyTo(
    #[prop(into)] destination: Signal<CameraDestination>,
    #[prop(optional, into)] orientation: Signal<Option<CameraOrientation>>,
    #[prop(optional, into)] duration: Signal<Option<f64>>,
    #[prop(optional, into)] end_transform: JsSignal<Option<Matrix4>>,
    #[prop(optional, into)] convert: Signal<Option<bool>>,
    #[prop(optional, into)] maximum_height: Signal<Option<f64>>,
    #[prop(optional, into)] pitch_adjust_height: Signal<Option<f64>>,
    #[prop(optional, into)] fly_over_longitude: Signal<Option<f64>>,
    #[prop(optional, into)] fly_over_longitude_weight: Signal<Option<f64>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraFlyTo must be inside ViewerContainer");

        Effect::new(move |_| {
            let destination = destination.get();
            let orientation = orientation.get();
            let duration = duration.get();
            let end_transform = end_transform.get();
            let convert = convert.get();
            let maximum_height = maximum_height.get();
            let pitch_adjust_height = pitch_adjust_height.get();
            let fly_over_longitude = fly_over_longitude.get();
            let fly_over_longitude_weight = fly_over_longitude_weight.get();

            viewer_context.with_viewer(|viewer: Viewer| {
                let mut options = FlyToOptions::new(destination_to_binding(destination));

                if let Some(orientation) = orientation {
                    options = options.orientation(orientation_to_binding(orientation));
                }
                if let Some(duration) = duration {
                    options = options.duration(duration);
                }
                if let Some(end_transform) = end_transform {
                    options = options.end_transform(end_transform);
                }
                if let Some(convert) = convert {
                    options = options.convert(convert);
                }
                if let Some(maximum_height) = maximum_height {
                    options = options.maximum_height(maximum_height);
                }
                if let Some(pitch_adjust_height) = pitch_adjust_height {
                    options = options.pitch_adjust_height(pitch_adjust_height);
                }
                if let Some(fly_over_longitude) = fly_over_longitude {
                    options = options.fly_over_longitude(fly_over_longitude);
                }
                if let Some(fly_over_longitude_weight) = fly_over_longitude_weight {
                    options = options.fly_over_longitude_weight(fly_over_longitude_weight);
                }

                viewer.camera().fly_to(&options.build());
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            destination,
            orientation,
            duration,
            end_transform,
            convert,
            maximum_height,
            pitch_adjust_height,
            fly_over_longitude,
            fly_over_longitude_weight,
        );
    }
}

/// Fly camera to fit a bounding sphere.
#[component(transparent)]
pub fn CameraFlyToBoundingSphere(
    #[prop(into)] target: JsSignal<BoundingSphere>,
    #[prop(optional, into)] offset: Signal<Option<(f64, f64, f64)>>,
    #[prop(optional, into)] duration: Signal<Option<f64>>,
    #[prop(optional, into)] maximum_height: Signal<Option<f64>>,
    #[prop(optional, into)] pitch_adjust_height: Signal<Option<f64>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraFlyToBoundingSphere must be inside ViewerContainer");

        Effect::new(move |_| {
            let target = target.get();
            let offset = offset.get();
            let duration = duration.get();
            let maximum_height = maximum_height.get();
            let pitch_adjust_height = pitch_adjust_height.get();

            viewer_context.with_viewer(|viewer: Viewer| {
                let mut options = FlyToBoundingSphereOptions::new();
                let mut has_options = false;

                if let Some(duration) = duration {
                    options = options.duration(duration);
                    has_options = true;
                }
                if let Some(maximum_height) = maximum_height {
                    options = options.maximum_height(maximum_height);
                    has_options = true;
                }
                if let Some(pitch_adjust_height) = pitch_adjust_height {
                    options = options.pitch_adjust_height(pitch_adjust_height);
                    has_options = true;
                }
                if let Some((heading, pitch, range)) = offset {
                    options = options.offset(HeadingPitchRange::new(heading, pitch, range));
                    has_options = true;
                }

                if has_options {
                    viewer
                        .camera()
                        .fly_to_bounding_sphere(&target, &options.build());
                } else {
                    viewer.camera().fly_to_bounding_sphere_default(&target);
                }
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            target,
            offset,
            duration,
            maximum_height,
            pitch_adjust_height,
        );
    }
}

/// Orient camera to look at a target.
#[component(transparent)]
pub fn CameraLookAt(
    #[prop(into)] target: Signal<DVec3>,
    #[prop(into)] offset: Signal<LookAtOffset>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraLookAt must be inside ViewerContainer");

        Effect::new(move |_| {
            let target = target.get();
            let offset = offset.get();

            viewer_context.with_viewer(|viewer: Viewer| {
                let target = Cartesian3::from_degrees(target.x, target.y, target.z);
                let offset = look_at_offset_to_js(offset);
                viewer.camera().look_at(&target, &offset);
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (target, offset);
    }
}

/// Orient camera with a reference transform.
#[component(transparent)]
pub fn CameraLookAtTransform(
    #[prop(into)] transform: JsSignal<Matrix4>,
    #[prop(optional, into)] offset: Signal<Option<LookAtOffset>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraLookAtTransform must be inside ViewerContainer");

        Effect::new(move |_| {
            let transform = transform.get();
            let offset = offset.get();

            viewer_context.with_viewer(|viewer: Viewer| {
                if let Some(offset) = offset {
                    viewer
                        .camera()
                        .look_at_transform_with_offset(&transform, &look_at_offset_to_js(offset));
                } else {
                    viewer.camera().look_at_transform(&transform);
                }
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (transform, offset);
    }
}

/// Cancel in-progress camera flight when trigger updates.
#[component(transparent)]
pub fn CameraCancelFlight(#[prop(into)] trigger: Signal<()>) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraCancelFlight must be inside ViewerContainer");
        let mut is_first_run = true;
        Effect::new(move |_| {
            trigger.get();
            if is_first_run {
                is_first_run = false;
                return;
            }
            viewer_context.with_viewer(|viewer: Viewer| viewer.camera().cancel_flight());
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = trigger;
    }
}

/// Complete in-progress camera flight when trigger updates.
#[component(transparent)]
pub fn CameraCompleteFlight(#[prop(into)] trigger: Signal<()>) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraCompleteFlight must be inside ViewerContainer");
        let mut is_first_run = true;
        Effect::new(move |_| {
            trigger.get();
            if is_first_run {
                is_first_run = false;
                return;
            }
            viewer_context.with_viewer(|viewer: Viewer| viewer.camera().complete_flight());
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = trigger;
    }
}

/// Move camera in one of Cesium's canonical move directions.
#[component(transparent)]
pub fn CameraMove(
    #[prop(into)] trigger: Signal<()>,
    #[prop(into)] direction: Signal<CameraMoveDirection>,
    #[prop(optional, into)] amount: Signal<Option<f64>>,
    /// Required only for `CameraMoveDirection::AlongAxis`.
    #[prop(optional, into)]
    axis: Signal<Option<DVec3>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraMove must be inside ViewerContainer");
        let mut is_first_run = true;
        Effect::new(move |_| {
            trigger.get();
            if is_first_run {
                is_first_run = false;
                return;
            }

            // Trigger-driven action: read other inputs untracked to avoid repeated
            // movement when these values are updated.
            let direction = direction.get_untracked();
            let amount = amount.get_untracked();
            let axis = axis.get_untracked();

            viewer_context.with_viewer(|viewer: Viewer| {
                let camera = viewer.camera();
                let amount = amount.unwrap_or(camera.default_move_amount());

                match direction {
                    CameraMoveDirection::Forward => camera.move_forward(amount),
                    CameraMoveDirection::Backward => camera.move_backward(amount),
                    CameraMoveDirection::Up => camera.move_up(amount),
                    CameraMoveDirection::Down => camera.move_down(amount),
                    CameraMoveDirection::Right => camera.move_right(amount),
                    CameraMoveDirection::Left => camera.move_left(amount),
                    CameraMoveDirection::AlongAxis => {
                        if let Some(axis) = axis {
                            let axis = Cartesian3::new(axis.x, axis.y, axis.z);
                            camera.move_along(&axis, amount);
                        }
                    }
                }
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (trigger, direction, amount, axis);
    }
}

/// Zoom camera in/out.
#[component(transparent)]
pub fn CameraZoom(
    #[prop(into)] trigger: Signal<()>,
    #[prop(into)] direction: Signal<CameraZoomDirection>,
    #[prop(optional, into)] amount: Signal<Option<f64>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraZoom must be inside ViewerContainer");
        let mut is_first_run = true;
        Effect::new(move |_| {
            trigger.get();
            if is_first_run {
                is_first_run = false;
                return;
            }

            // Trigger-driven action: read other inputs untracked to avoid repeated
            // zoom when these values are updated.
            let direction = direction.get_untracked();
            let amount = amount.get_untracked();

            viewer_context.with_viewer(|viewer: Viewer| {
                let camera = viewer.camera();
                let amount = amount.unwrap_or(camera.default_zoom_amount());

                match direction {
                    CameraZoomDirection::In => camera.zoom_in(amount),
                    CameraZoomDirection::Out => camera.zoom_out(amount),
                }
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (trigger, direction, amount);
    }
}

/// Configure Cesium `ScreenSpaceCameraController` reactively.
#[component(transparent)]
pub fn CameraController(
    #[prop(optional, into)] enable_inputs: Signal<Option<bool>>,
    #[prop(optional, into)] enable_translate: Signal<Option<bool>>,
    #[prop(optional, into)] enable_zoom: Signal<Option<bool>>,
    #[prop(optional, into)] enable_rotate: Signal<Option<bool>>,
    #[prop(optional, into)] enable_tilt: Signal<Option<bool>>,
    #[prop(optional, into)] enable_look: Signal<Option<bool>>,
    #[prop(optional, into)] enable_collision_detection: Signal<Option<bool>>,
    #[prop(optional, into)] minimum_zoom_distance: Signal<Option<f64>>,
    #[prop(optional, into)] maximum_zoom_distance: Signal<Option<f64>>,
    #[prop(optional, into)] maximum_tilt_angle: Signal<Option<f64>>,
    #[prop(optional, into)] inertia_spin: Signal<Option<f64>>,
    #[prop(optional, into)] inertia_translate: Signal<Option<f64>>,
    #[prop(optional, into)] inertia_zoom: Signal<Option<f64>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("CameraController must be inside ViewerContainer");

        Effect::new(move |_| {
            let enable_inputs = enable_inputs.get();
            let enable_translate = enable_translate.get();
            let enable_zoom = enable_zoom.get();
            let enable_rotate = enable_rotate.get();
            let enable_tilt = enable_tilt.get();
            let enable_look = enable_look.get();
            let enable_collision_detection = enable_collision_detection.get();
            let minimum_zoom_distance = minimum_zoom_distance.get();
            let maximum_zoom_distance = maximum_zoom_distance.get();
            let maximum_tilt_angle = maximum_tilt_angle.get();
            let inertia_spin = inertia_spin.get();
            let inertia_translate = inertia_translate.get();
            let inertia_zoom = inertia_zoom.get();

            viewer_context.with_viewer(|viewer: Viewer| {
                let controller = viewer.scene().screen_space_camera_controller();

                if let Some(value) = enable_inputs {
                    controller.set_enable_inputs(value);
                }
                if let Some(value) = enable_translate {
                    controller.set_enable_translate(value);
                }
                if let Some(value) = enable_zoom {
                    controller.set_enable_zoom(value);
                }
                if let Some(value) = enable_rotate {
                    controller.set_enable_rotate(value);
                }
                if let Some(value) = enable_tilt {
                    controller.set_enable_tilt(value);
                }
                if let Some(value) = enable_look {
                    controller.set_enable_look(value);
                }
                if let Some(value) = enable_collision_detection {
                    controller.set_enable_collision_detection(value);
                }
                if let Some(value) = minimum_zoom_distance {
                    controller.set_minimum_zoom_distance(value);
                }
                if let Some(value) = maximum_zoom_distance {
                    controller.set_maximum_zoom_distance(value);
                }
                if let Some(value) = maximum_tilt_angle {
                    controller.set_maximum_tilt_angle(Some(value));
                }
                if let Some(value) = inertia_spin {
                    controller.set_inertia_spin(value);
                }
                if let Some(value) = inertia_translate {
                    controller.set_inertia_translate(value);
                }
                if let Some(value) = inertia_zoom {
                    controller.set_inertia_zoom(value);
                }
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            enable_inputs,
            enable_translate,
            enable_zoom,
            enable_rotate,
            enable_tilt,
            enable_look,
            enable_collision_detection,
            minimum_zoom_distance,
            maximum_zoom_distance,
            maximum_tilt_angle,
            inertia_spin,
            inertia_translate,
            inertia_zoom,
        );
    }
}

/// Reset viewer clock to now and stop animation.
#[component(transparent)]
pub fn ClockReset(#[prop(into)] trigger: Signal<()>) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("ClockReset must be inside ViewerContainer");
        let mut is_first_run = true;

        Effect::new(move |_| {
            trigger.get();
            if is_first_run {
                is_first_run = false;
                return;
            }

            viewer_context.with_viewer(|viewer: Viewer| {
                let clock = viewer.clock();
                let now = julian_date_now();
                clock.set_current_time(&now);
                clock.set_should_animate(false);
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = trigger;
    }
}
