//! Minimal Cesium viewer bindings needed to bootstrap rendering.

use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::bindings::JulianDate;
use crate::bindings::camera::{BoundingSphere, HeadingPitchRange};
use crate::bindings::cartesian2::Cartesian2;
use crate::bindings::coordinates::Cartesian3;
use crate::bindings::data_source::{DataSource, DataSourceCollection};
use crate::bindings::entity::{Entity, EntityCollection};
use crate::bindings::rectangle::Rectangle;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Cesium, js_name = Viewer)]
    pub type Viewer;

    #[wasm_bindgen(constructor, js_namespace = Cesium, js_class = Viewer)]
    pub fn new(container: &HtmlElement, options: &JsValue) -> Viewer;

    #[wasm_bindgen(method, js_name = destroy)]
    pub fn destroy(this: &Viewer) -> bool;

    #[wasm_bindgen(method, getter, js_name = entities)]
    pub fn entities(this: &Viewer) -> EntityCollection;

    #[wasm_bindgen(method, getter, js_name = dataSources)]
    pub fn data_sources(this: &Viewer) -> DataSourceCollection;

    #[wasm_bindgen(method, getter, js_name = camera)]
    pub fn camera(this: &Viewer) -> Camera;

    #[wasm_bindgen(method, getter, js_name = clock)]
    pub fn clock(this: &Viewer) -> Clock;

    #[wasm_bindgen(method, getter, js_name = scene)]
    pub fn scene(this: &Viewer) -> Scene;

    #[wasm_bindgen(method, js_name = zoomTo)]
    pub fn zoom_to(this: &Viewer, target: &JsValue) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = zoomTo)]
    pub fn zoom_to_with_offset(
        this: &Viewer,
        target: &JsValue,
        offset: &JsValue,
    ) -> js_sys::Promise;

    /// Selected entity property (get/set)
    #[wasm_bindgen(method, getter, js_name = selectedEntity)]
    pub fn selected_entity(this: &Viewer) -> Option<Entity>;

    #[wasm_bindgen(method, setter, js_name = selectedEntity)]
    pub fn set_selected_entity(this: &Viewer, entity: Option<&Entity>);

    /// Event fired when the selected entity changes
    #[wasm_bindgen(method, getter, js_name = selectedEntityChanged)]
    pub fn selected_entity_changed(this: &Viewer) -> Event;

    /// Tracked entity property (get/set) - the entity currently being tracked by the camera
    #[wasm_bindgen(method, getter, js_name = trackedEntity)]
    pub fn tracked_entity(this: &Viewer) -> Option<Entity>;

    #[wasm_bindgen(method, setter, js_name = trackedEntity)]
    pub fn set_tracked_entity(this: &Viewer, entity: Option<&Entity>);

    /// Event fired when the tracked entity changes
    #[wasm_bindgen(method, getter, js_name = trackedEntityChanged)]
    pub fn tracked_entity_changed(this: &Viewer) -> Event;

    /// Gets or sets whether data sources can suspend animation while assets stream.
    #[wasm_bindgen(method, getter, js_name = allowDataSourcesToSuspendAnimation)]
    pub fn allow_data_sources_to_suspend_animation(this: &Viewer) -> bool;

    #[wasm_bindgen(method, setter, js_name = allowDataSourcesToSuspendAnimation)]
    pub fn set_allow_data_sources_to_suspend_animation(this: &Viewer, value: bool);

    /// Gets or sets the data source tracked by the viewer clock.
    #[wasm_bindgen(method, getter, js_name = clockTrackedDataSource)]
    pub fn clock_tracked_data_source(this: &Viewer) -> Option<DataSource>;

    #[wasm_bindgen(method, setter, js_name = clockTrackedDataSource)]
    pub fn set_clock_tracked_data_source(this: &Viewer, value: Option<&DataSource>);

    /// Cesium Event type for event handling
    #[wasm_bindgen(js_namespace = Cesium, js_name = Event)]
    pub type Event;

    #[wasm_bindgen(method, js_name = addEventListener)]
    pub fn add_event_listener(this: &Event, listener: &js_sys::Function);

    #[wasm_bindgen(method, js_name = removeEventListener)]
    pub fn remove_event_listener(this: &Event, listener: &js_sys::Function);

    /// Camera for controlling the view
    #[wasm_bindgen(js_namespace = Cesium, js_name = Camera)]
    pub type Camera;

    /// Generic matrix type used by camera transforms.
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = Matrix4)]
    pub type Matrix4;

    /// Ellipsoid type used for pick and view rectangle helpers.
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = Ellipsoid)]
    pub type Ellipsoid;

    /// Ray type returned by camera pick helpers.
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = Ray)]
    pub type Ray;

    /// Cartesian4 used for world/camera coordinate transforms.
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = Cartesian4)]
    pub type Cartesian4;

    /// Cartographic camera position representation.
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Cesium, js_name = Cartographic)]
    pub type Cartographic;

    /// Current camera position in world coordinates.
    #[wasm_bindgen(method, getter, js_name = position)]
    pub fn position(this: &Camera) -> Cartesian3;

    #[wasm_bindgen(method, setter, js_name = position)]
    pub fn set_position(this: &Camera, value: &Cartesian3);

    /// Current camera direction vector.
    #[wasm_bindgen(method, getter, js_name = direction)]
    pub fn direction(this: &Camera) -> Cartesian3;

    #[wasm_bindgen(method, setter, js_name = direction)]
    pub fn set_direction(this: &Camera, value: &Cartesian3);

    /// Current camera up vector.
    #[wasm_bindgen(method, getter, js_name = up)]
    pub fn up(this: &Camera) -> Cartesian3;

    #[wasm_bindgen(method, setter, js_name = up)]
    pub fn set_up(this: &Camera, value: &Cartesian3);

    /// Current camera right vector.
    #[wasm_bindgen(method, getter, js_name = right)]
    pub fn right(this: &Camera) -> Cartesian3;

    #[wasm_bindgen(method, setter, js_name = right)]
    pub fn set_right(this: &Camera, value: &Cartesian3);

    /// Camera frustum object.
    #[wasm_bindgen(method, getter, js_name = frustum)]
    pub fn frustum(this: &Camera) -> JsValue;

    #[wasm_bindgen(method, setter, js_name = frustum)]
    pub fn set_frustum(this: &Camera, value: &JsValue);

    #[wasm_bindgen(method, getter, js_name = defaultMoveAmount)]
    pub fn default_move_amount(this: &Camera) -> f64;

    #[wasm_bindgen(method, setter, js_name = defaultMoveAmount)]
    pub fn set_default_move_amount(this: &Camera, value: f64);

    #[wasm_bindgen(method, getter, js_name = defaultLookAmount)]
    pub fn default_look_amount(this: &Camera) -> f64;

    #[wasm_bindgen(method, setter, js_name = defaultLookAmount)]
    pub fn set_default_look_amount(this: &Camera, value: f64);

    #[wasm_bindgen(method, getter, js_name = defaultRotateAmount)]
    pub fn default_rotate_amount(this: &Camera) -> f64;

    #[wasm_bindgen(method, setter, js_name = defaultRotateAmount)]
    pub fn set_default_rotate_amount(this: &Camera, value: f64);

    #[wasm_bindgen(method, getter, js_name = defaultZoomAmount)]
    pub fn default_zoom_amount(this: &Camera) -> f64;

    #[wasm_bindgen(method, setter, js_name = defaultZoomAmount)]
    pub fn set_default_zoom_amount(this: &Camera, value: f64);

    #[wasm_bindgen(method, getter, js_name = constrainedAxis)]
    pub fn constrained_axis(this: &Camera) -> Option<Cartesian3>;

    #[wasm_bindgen(method, setter, js_name = constrainedAxis)]
    pub fn set_constrained_axis(this: &Camera, value: Option<&Cartesian3>);

    #[wasm_bindgen(method, getter, js_name = maximumZoomFactor)]
    pub fn maximum_zoom_factor(this: &Camera) -> f64;

    #[wasm_bindgen(method, setter, js_name = maximumZoomFactor)]
    pub fn set_maximum_zoom_factor(this: &Camera, value: f64);

    #[wasm_bindgen(method, getter, js_name = percentageChanged)]
    pub fn percentage_changed(this: &Camera) -> f64;

    #[wasm_bindgen(method, setter, js_name = percentageChanged)]
    pub fn set_percentage_changed(this: &Camera, value: f64);

    #[wasm_bindgen(method, getter, js_name = positionCartographic)]
    pub fn position_cartographic(this: &Camera) -> Cartographic;

    #[wasm_bindgen(method, getter, js_name = positionWC)]
    pub fn position_wc(this: &Camera) -> Cartesian3;

    #[wasm_bindgen(method, getter, js_name = directionWC)]
    pub fn direction_wc(this: &Camera) -> Cartesian3;

    #[wasm_bindgen(method, getter, js_name = upWC)]
    pub fn up_wc(this: &Camera) -> Cartesian3;

    #[wasm_bindgen(method, getter, js_name = rightWC)]
    pub fn right_wc(this: &Camera) -> Cartesian3;

    #[wasm_bindgen(method, getter, js_name = heading)]
    pub fn heading(this: &Camera) -> f64;

    #[wasm_bindgen(method, getter, js_name = pitch)]
    pub fn pitch(this: &Camera) -> f64;

    #[wasm_bindgen(method, getter, js_name = roll)]
    pub fn roll(this: &Camera) -> f64;

    #[wasm_bindgen(method, getter, js_name = moveStart)]
    pub fn move_start(this: &Camera) -> Event;

    #[wasm_bindgen(method, getter, js_name = moveEnd)]
    pub fn move_end(this: &Camera) -> Event;

    #[wasm_bindgen(method, getter, js_name = changed)]
    pub fn changed(this: &Camera) -> Event;

    #[wasm_bindgen(method, js_name = setView)]
    pub fn set_view(this: &Camera, options: &JsValue);

    #[wasm_bindgen(method, js_name = flyHome)]
    pub fn fly_home(this: &Camera, duration: f64);

    #[wasm_bindgen(method, js_name = flyHome)]
    pub fn fly_home_default(this: &Camera);

    #[wasm_bindgen(method, js_name = flyTo)]
    pub fn fly_to(this: &Camera, options: &JsValue);

    #[wasm_bindgen(method, js_name = flyToBoundingSphere)]
    pub fn fly_to_bounding_sphere(
        this: &Camera,
        bounding_sphere: &BoundingSphere,
        options: &JsValue,
    );

    #[wasm_bindgen(method, js_name = flyToBoundingSphere)]
    pub fn fly_to_bounding_sphere_default(this: &Camera, bounding_sphere: &BoundingSphere);

    #[wasm_bindgen(method, js_name = lookAt)]
    pub fn look_at(this: &Camera, target: &Cartesian3, offset: &JsValue);

    #[wasm_bindgen(method, js_name = lookAtTransform)]
    pub fn look_at_transform(this: &Camera, transform: &Matrix4);

    #[wasm_bindgen(method, js_name = lookAtTransform)]
    pub fn look_at_transform_with_offset(this: &Camera, transform: &Matrix4, offset: &JsValue);

    #[wasm_bindgen(method, js_name = worldToCameraCoordinates)]
    pub fn world_to_camera_coordinates(this: &Camera, cartesian: &Cartesian4) -> Cartesian4;

    #[wasm_bindgen(method, js_name = worldToCameraCoordinatesPoint)]
    pub fn world_to_camera_coordinates_point(this: &Camera, cartesian: &Cartesian3) -> Cartesian3;

    #[wasm_bindgen(method, js_name = worldToCameraCoordinatesVector)]
    pub fn world_to_camera_coordinates_vector(this: &Camera, cartesian: &Cartesian3) -> Cartesian3;

    #[wasm_bindgen(method, js_name = cameraToWorldCoordinates)]
    pub fn camera_to_world_coordinates(this: &Camera, cartesian: &Cartesian4) -> Cartesian4;

    #[wasm_bindgen(method, js_name = cameraToWorldCoordinatesPoint)]
    pub fn camera_to_world_coordinates_point(this: &Camera, cartesian: &Cartesian3) -> Cartesian3;

    #[wasm_bindgen(method, js_name = cameraToWorldCoordinatesVector)]
    pub fn camera_to_world_coordinates_vector(this: &Camera, cartesian: &Cartesian3) -> Cartesian3;

    #[wasm_bindgen(method, js_name = move)]
    pub fn move_along(this: &Camera, direction: &Cartesian3, amount: f64);

    #[wasm_bindgen(method, js_name = moveForward)]
    pub fn move_forward(this: &Camera, amount: f64);

    #[wasm_bindgen(method, js_name = moveBackward)]
    pub fn move_backward(this: &Camera, amount: f64);

    #[wasm_bindgen(method, js_name = moveUp)]
    pub fn move_up(this: &Camera, amount: f64);

    #[wasm_bindgen(method, js_name = moveDown)]
    pub fn move_down(this: &Camera, amount: f64);

    #[wasm_bindgen(method, js_name = moveRight)]
    pub fn move_right(this: &Camera, amount: f64);

    #[wasm_bindgen(method, js_name = moveLeft)]
    pub fn move_left(this: &Camera, amount: f64);

    #[wasm_bindgen(method, js_name = lookLeft)]
    pub fn look_left(this: &Camera, amount: f64);

    #[wasm_bindgen(method, js_name = lookRight)]
    pub fn look_right(this: &Camera, amount: f64);

    #[wasm_bindgen(method, js_name = lookUp)]
    pub fn look_up(this: &Camera, amount: f64);

    #[wasm_bindgen(method, js_name = lookDown)]
    pub fn look_down(this: &Camera, amount: f64);

    #[wasm_bindgen(method, js_name = look)]
    pub fn look(this: &Camera, axis: &Cartesian3, angle: f64);

    #[wasm_bindgen(method, js_name = twistLeft)]
    pub fn twist_left(this: &Camera, amount: f64);

    #[wasm_bindgen(method, js_name = twistRight)]
    pub fn twist_right(this: &Camera, amount: f64);

    #[wasm_bindgen(method, js_name = rotate)]
    pub fn rotate(this: &Camera, axis: &Cartesian3, angle: f64);

    #[wasm_bindgen(method, js_name = rotateDown)]
    pub fn rotate_down(this: &Camera, angle: f64);

    #[wasm_bindgen(method, js_name = rotateUp)]
    pub fn rotate_up(this: &Camera, angle: f64);

    #[wasm_bindgen(method, js_name = rotateRight)]
    pub fn rotate_right(this: &Camera, angle: f64);

    #[wasm_bindgen(method, js_name = rotateLeft)]
    pub fn rotate_left(this: &Camera, angle: f64);

    #[wasm_bindgen(method, js_name = zoomIn)]
    pub fn zoom_in(this: &Camera, amount: f64);

    #[wasm_bindgen(method, js_name = zoomOut)]
    pub fn zoom_out(this: &Camera, amount: f64);

    #[wasm_bindgen(method, js_name = getMagnitude)]
    pub fn get_magnitude(this: &Camera) -> f64;

    #[wasm_bindgen(method, js_name = getRectangleCameraCoordinates)]
    pub fn get_rectangle_camera_coordinates(this: &Camera, rectangle: &Rectangle) -> Cartesian3;

    #[wasm_bindgen(method, js_name = pickEllipsoid)]
    pub fn pick_ellipsoid(this: &Camera, window_position: &Cartesian2) -> Option<Cartesian3>;

    #[wasm_bindgen(method, js_name = pickEllipsoid)]
    pub fn pick_ellipsoid_with_ellipsoid(
        this: &Camera,
        window_position: &Cartesian2,
        ellipsoid: &Ellipsoid,
    ) -> Option<Cartesian3>;

    #[wasm_bindgen(method, js_name = getPickRay)]
    pub fn get_pick_ray(this: &Camera, window_position: &Cartesian2) -> Option<Ray>;

    #[wasm_bindgen(method, js_name = distanceToBoundingSphere)]
    pub fn distance_to_bounding_sphere(this: &Camera, bounding_sphere: &BoundingSphere) -> f64;

    #[wasm_bindgen(method, js_name = getPixelSize)]
    pub fn get_pixel_size(
        this: &Camera,
        bounding_sphere: &BoundingSphere,
        drawing_buffer_width: f64,
        drawing_buffer_height: f64,
    ) -> f64;

    #[wasm_bindgen(method, js_name = cancelFlight)]
    pub fn cancel_flight(this: &Camera);

    #[wasm_bindgen(method, js_name = completeFlight)]
    pub fn complete_flight(this: &Camera);

    #[wasm_bindgen(method, js_name = viewBoundingSphere)]
    pub fn view_bounding_sphere(this: &Camera, bounding_sphere: &BoundingSphere);

    #[wasm_bindgen(method, js_name = viewBoundingSphere)]
    pub fn view_bounding_sphere_with_offset(
        this: &Camera,
        bounding_sphere: &BoundingSphere,
        offset: &HeadingPitchRange,
    );

    #[wasm_bindgen(method, js_name = computeViewRectangle)]
    pub fn compute_view_rectangle(this: &Camera) -> Option<Rectangle>;

    #[wasm_bindgen(method, js_name = computeViewRectangle)]
    pub fn compute_view_rectangle_with_ellipsoid(
        this: &Camera,
        ellipsoid: &Ellipsoid,
    ) -> Option<Rectangle>;

    #[wasm_bindgen(method, js_name = switchToPerspectiveFrustum)]
    pub fn switch_to_perspective_frustum(this: &Camera);

    #[wasm_bindgen(method, js_name = switchToOrthographicFrustum)]
    pub fn switch_to_orthographic_frustum(this: &Camera);

    /// Clock for controlling time and animation
    #[wasm_bindgen(js_namespace = Cesium, js_name = Clock)]
    pub type Clock;

    #[wasm_bindgen(method, getter, js_name = shouldAnimate)]
    pub fn should_animate(this: &Clock) -> bool;

    #[wasm_bindgen(method, setter, js_name = shouldAnimate)]
    pub fn set_should_animate(this: &Clock, value: bool);

    #[wasm_bindgen(method, getter, js_name = canAnimate)]
    pub fn can_animate(this: &Clock) -> bool;

    #[wasm_bindgen(method, setter, js_name = canAnimate)]
    pub fn set_can_animate(this: &Clock, value: bool);

    #[wasm_bindgen(method, getter, js_name = currentTime)]
    pub fn current_time(this: &Clock) -> JulianDate;

    #[wasm_bindgen(method, setter, js_name = currentTime)]
    pub fn set_current_time(this: &Clock, value: &JulianDate);

    #[wasm_bindgen(method, getter, js_name = startTime)]
    pub fn start_time(this: &Clock) -> JulianDate;

    #[wasm_bindgen(method, setter, js_name = startTime)]
    pub fn set_start_time(this: &Clock, value: &JulianDate);

    #[wasm_bindgen(method, getter, js_name = stopTime)]
    pub fn stop_time(this: &Clock) -> JulianDate;

    #[wasm_bindgen(method, setter, js_name = stopTime)]
    pub fn set_stop_time(this: &Clock, value: &JulianDate);

    #[wasm_bindgen(method, getter, js_name = multiplier)]
    pub fn multiplier(this: &Clock) -> f64;

    #[wasm_bindgen(method, setter, js_name = multiplier)]
    pub fn set_multiplier(this: &Clock, value: f64);

    #[wasm_bindgen(method, getter, js_name = clockRange)]
    pub fn clock_range(this: &Clock) -> i32;

    #[wasm_bindgen(method, setter, js_name = clockRange)]
    pub fn set_clock_range(this: &Clock, value: i32);

    #[wasm_bindgen(method, getter, js_name = clockStep)]
    pub fn clock_step(this: &Clock) -> i32;

    #[wasm_bindgen(method, setter, js_name = clockStep)]
    pub fn set_clock_step(this: &Clock, value: i32);

    #[wasm_bindgen(method, getter, js_name = onTick)]
    pub fn on_tick(this: &Clock) -> Event;

    #[wasm_bindgen(method, getter, js_name = onStop)]
    pub fn on_stop(this: &Clock) -> Event;

    /// Scene contains the primitives and other visual elements
    #[wasm_bindgen(js_namespace = Cesium, js_name = Scene)]
    pub type Scene;

    #[wasm_bindgen(method, getter, js_name = primitives)]
    pub fn primitives(this: &Scene) -> PrimitiveCollection;

    #[wasm_bindgen(method, getter, js_name = screenSpaceCameraController)]
    pub fn screen_space_camera_controller(this: &Scene) -> ScreenSpaceCameraController;

    /// Camera interaction controller.
    #[wasm_bindgen(js_namespace = Cesium, js_name = ScreenSpaceCameraController)]
    pub type ScreenSpaceCameraController;

    #[wasm_bindgen(method, getter, js_name = enableInputs)]
    pub fn enable_inputs(this: &ScreenSpaceCameraController) -> bool;

    #[wasm_bindgen(method, setter, js_name = enableInputs)]
    pub fn set_enable_inputs(this: &ScreenSpaceCameraController, value: bool);

    #[wasm_bindgen(method, getter, js_name = enableTranslate)]
    pub fn enable_translate(this: &ScreenSpaceCameraController) -> bool;

    #[wasm_bindgen(method, setter, js_name = enableTranslate)]
    pub fn set_enable_translate(this: &ScreenSpaceCameraController, value: bool);

    #[wasm_bindgen(method, getter, js_name = enableZoom)]
    pub fn enable_zoom(this: &ScreenSpaceCameraController) -> bool;

    #[wasm_bindgen(method, setter, js_name = enableZoom)]
    pub fn set_enable_zoom(this: &ScreenSpaceCameraController, value: bool);

    #[wasm_bindgen(method, getter, js_name = enableRotate)]
    pub fn enable_rotate(this: &ScreenSpaceCameraController) -> bool;

    #[wasm_bindgen(method, setter, js_name = enableRotate)]
    pub fn set_enable_rotate(this: &ScreenSpaceCameraController, value: bool);

    #[wasm_bindgen(method, getter, js_name = enableTilt)]
    pub fn enable_tilt(this: &ScreenSpaceCameraController) -> bool;

    #[wasm_bindgen(method, setter, js_name = enableTilt)]
    pub fn set_enable_tilt(this: &ScreenSpaceCameraController, value: bool);

    #[wasm_bindgen(method, getter, js_name = enableLook)]
    pub fn enable_look(this: &ScreenSpaceCameraController) -> bool;

    #[wasm_bindgen(method, setter, js_name = enableLook)]
    pub fn set_enable_look(this: &ScreenSpaceCameraController, value: bool);

    #[wasm_bindgen(method, getter, js_name = enableCollisionDetection)]
    pub fn enable_collision_detection(this: &ScreenSpaceCameraController) -> bool;

    #[wasm_bindgen(method, setter, js_name = enableCollisionDetection)]
    pub fn set_enable_collision_detection(this: &ScreenSpaceCameraController, value: bool);

    #[wasm_bindgen(method, getter, js_name = minimumZoomDistance)]
    pub fn minimum_zoom_distance(this: &ScreenSpaceCameraController) -> f64;

    #[wasm_bindgen(method, setter, js_name = minimumZoomDistance)]
    pub fn set_minimum_zoom_distance(this: &ScreenSpaceCameraController, value: f64);

    #[wasm_bindgen(method, getter, js_name = maximumZoomDistance)]
    pub fn maximum_zoom_distance(this: &ScreenSpaceCameraController) -> f64;

    #[wasm_bindgen(method, setter, js_name = maximumZoomDistance)]
    pub fn set_maximum_zoom_distance(this: &ScreenSpaceCameraController, value: f64);

    #[wasm_bindgen(method, getter, js_name = maximumTiltAngle)]
    pub fn maximum_tilt_angle(this: &ScreenSpaceCameraController) -> Option<f64>;

    #[wasm_bindgen(method, setter, js_name = maximumTiltAngle)]
    pub fn set_maximum_tilt_angle(this: &ScreenSpaceCameraController, value: Option<f64>);

    #[wasm_bindgen(method, getter, js_name = inertiaSpin)]
    pub fn inertia_spin(this: &ScreenSpaceCameraController) -> f64;

    #[wasm_bindgen(method, setter, js_name = inertiaSpin)]
    pub fn set_inertia_spin(this: &ScreenSpaceCameraController, value: f64);

    #[wasm_bindgen(method, getter, js_name = inertiaTranslate)]
    pub fn inertia_translate(this: &ScreenSpaceCameraController) -> f64;

    #[wasm_bindgen(method, setter, js_name = inertiaTranslate)]
    pub fn set_inertia_translate(this: &ScreenSpaceCameraController, value: f64);

    #[wasm_bindgen(method, getter, js_name = inertiaZoom)]
    pub fn inertia_zoom(this: &ScreenSpaceCameraController) -> f64;

    #[wasm_bindgen(method, setter, js_name = inertiaZoom)]
    pub fn set_inertia_zoom(this: &ScreenSpaceCameraController, value: f64);

    /// Collection of primitives in the scene
    #[wasm_bindgen(js_namespace = Cesium, js_name = PrimitiveCollection)]
    pub type PrimitiveCollection;

    #[wasm_bindgen(method, js_name = add)]
    pub fn add(this: &PrimitiveCollection, primitive: &JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = remove)]
    pub fn remove(this: &PrimitiveCollection, primitive: &JsValue) -> bool;

    #[wasm_bindgen(method, js_name = removeAll)]
    pub fn remove_all(this: &PrimitiveCollection);
}

impl Viewer {
    /// Clears the tracked entity (convenience method)
    pub fn clear_tracked_entity(&self) {
        self.set_tracked_entity(None);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ClockRangeMode {
    #[default]
    Unbounded = 0,
    Clamped = 1,
    LoopStop = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ClockStepMode {
    #[default]
    TickDependent = 0,
    SystemClockMultiplier = 1,
    SystemClock = 2,
}

// Helper function to get current JulianDate using reflection
#[cfg(target_arch = "wasm32")]
pub fn julian_date_now() -> JulianDate {
    use js_sys::{Function, Reflect, global};
    use wasm_bindgen::{JsCast, JsValue};

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium"))
        .expect("Cesium global to be available");
    let julian_date_class = Reflect::get(&cesium, &JsValue::from_str("JulianDate"))
        .expect("Cesium.JulianDate to exist");
    let now_fn = Reflect::get(&julian_date_class, &JsValue::from_str("now"))
        .expect("Cesium.JulianDate.now to exist");
    let now_fn: Function = now_fn
        .dyn_into()
        .expect("Cesium.JulianDate.now to be callable");

    now_fn
        .call0(&julian_date_class)
        .expect("Cesium.JulianDate.now call to succeed")
        .unchecked_into::<JulianDate>()
}
