use geo_types::coord;
use leptos::prelude::*;
use leptos_cesium::prelude::*;

const PITCH_CENTERED_NADIR: f64 = -std::f64::consts::FRAC_PI_2;
const SF_CARTESIAN_CAMERA_4500M: DVec3 = DVec3::new(-2708081.749, -4264062.038, 3888482.014);

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());
    let (apply_set_view, set_apply_set_view) = signal(false);
    let (apply_fly_to, set_apply_fly_to) = signal(false);

    let (set_view_destination, set_set_view_destination) = signal(Some(
        CameraDestination::Degrees(DVec3::new(-74.0445, 40.6892, 3000.0)),
    ));
    let (set_view_orientation, set_set_view_orientation) = signal(Some(
        CameraOrientation::HeadingPitchRoll(0.0, PITCH_CENTERED_NADIR, 0.0),
    ));

    let (fly_to_destination, set_fly_to_destination) = signal(CameraDestination::Degrees(
        DVec3::new(-122.4194, 37.7749, 4500.0),
    ));
    let (fly_to_orientation, set_fly_to_orientation) = signal(Some(
        CameraOrientation::HeadingPitchRoll(0.0, PITCH_CENTERED_NADIR, 0.0),
    ));
    let (fly_to_duration, set_fly_to_duration) = signal(Some(3.0));

    let (fly_home_trigger, set_fly_home_trigger) = signal(());
    let (move_trigger, set_move_trigger) = signal(());
    let (move_direction, set_move_direction) = signal(CameraMoveDirection::Forward);
    let (move_amount, set_move_amount) = signal(Some(1500.0));

    let (zoom_trigger, set_zoom_trigger) = signal(());
    let (zoom_direction, set_zoom_direction) = signal(CameraZoomDirection::In);
    let (zoom_amount, set_zoom_amount) = signal(Some(3000.0));

    let (enable_inputs, set_enable_inputs) = signal(Some(true));
    let (enable_collision_detection, set_enable_collision_detection) = signal(Some(true));
    let (mount_controller, set_mount_controller) = signal(false);

    view! {
        <div class="layout">
            <div class="panel">
                <h2>"Camera Controls"</h2>
                <p style="margin: 0 0 8px; font-size: 12px; opacity: 0.8;">
                    "Startup baseline: Cesium default home view. Camera commands only run on button clicks."
                </p>
                <p style="margin: 0 0 8px; font-size: 12px; opacity: 0.8;">
                    "Cesium pitch convention: -PI/2 looks straight down; point presets use exact nadir so destination stays centered."
                </p>

                <h3>"Set View"</h3>
                <div class="group">
                    <button on:click=move |_| {
                        set_set_view_destination.set(Some(CameraDestination::Degrees(DVec3::new(-74.0445, 40.6892, 3000.0))));
                        set_set_view_orientation.set(Some(CameraOrientation::HeadingPitchRoll(0.0, PITCH_CENTERED_NADIR, 0.0)));
                        set_apply_set_view.set(true);
                    }>
                        "Statue of Liberty"
                    </button>
                    <button on:click=move |_| {
                        set_set_view_destination.set(Some(CameraDestination::Rectangle(Rect::new(
                            coord! { x: -130.0, y: 22.0 },
                            coord! { x: -65.0, y: 50.0 },
                        ))));
                        set_set_view_orientation.set(None);
                        set_apply_set_view.set(true);
                    }>
                        "Continental US (Rectangle)"
                    </button>
                </div>

                <h3>"Fly To"</h3>
                <div class="group">
                    <button on:click=move |_| {
                        set_fly_to_destination.set(CameraDestination::Degrees(DVec3::new(-122.4194, 37.7749, 4500.0)));
                        set_fly_to_orientation.set(Some(CameraOrientation::HeadingPitchRoll(0.0, PITCH_CENTERED_NADIR, 0.0)));
                        set_fly_to_duration.set(Some(3.0));
                        set_apply_fly_to.set(true);
                    }>
                        "San Francisco"
                    </button>
                    <button on:click=move |_| {
                        set_fly_to_destination.set(CameraDestination::Degrees(DVec3::new(139.7671, 35.6812, 5500.0)));
                        set_fly_to_orientation.set(Some(CameraOrientation::HeadingPitchRoll(0.0, PITCH_CENTERED_NADIR, 0.0)));
                        set_fly_to_duration.set(Some(3.8));
                        set_apply_fly_to.set(true);
                    }>
                        "Tokyo"
                    </button>
                    <button on:click=move |_| {
                        set_fly_to_destination.set(CameraDestination::Cartesian(SF_CARTESIAN_CAMERA_4500M));
                        set_fly_to_orientation.set(Some(CameraOrientation::HeadingPitchRoll(0.0, PITCH_CENTERED_NADIR, 0.0)));
                        set_fly_to_duration.set(Some(2.0));
                        set_apply_fly_to.set(true);
                    }>
                        "Cartesian Destination"
                    </button>
                    <button on:click=move |_| set_fly_home_trigger.set(())>
                        "Fly Home"
                    </button>
                </div>

                <h3>"Move / Zoom"</h3>
                <div class="group">
                    <button on:click=move |_| {
                        set_move_direction.set(CameraMoveDirection::Forward);
                        set_move_amount.set(Some(1000.0));
                        set_move_trigger.set(());
                    }>
                        "Move Forward"
                    </button>
                    <button on:click=move |_| {
                        set_move_direction.set(CameraMoveDirection::Left);
                        set_move_amount.set(Some(800.0));
                        set_move_trigger.set(());
                    }>
                        "Move Left"
                    </button>
                    <button on:click=move |_| {
                        set_zoom_direction.set(CameraZoomDirection::In);
                        set_zoom_amount.set(Some(2500.0));
                        set_zoom_trigger.set(());
                    }>
                        "Zoom In"
                    </button>
                    <button on:click=move |_| {
                        set_zoom_direction.set(CameraZoomDirection::Out);
                        set_zoom_amount.set(Some(2500.0));
                        set_zoom_trigger.set(());
                    }>
                        "Zoom Out"
                    </button>
                </div>

                <h3>"Controller"</h3>
                <div class="toggles">
                    <label>
                        <input
                            type="checkbox"
                            prop:checked=mount_controller
                            on:change=move |ev| set_mount_controller.set(event_target_checked(&ev))
                        />
                        "Mount CameraController (bisect toggle)"
                    </label>
                    <label>
                        <input
                            type="checkbox"
                            prop:checked=move || enable_inputs.get().unwrap_or(false)
                            on:change=move |ev| set_enable_inputs.set(Some(event_target_checked(&ev)))
                        />
                        "Enable Inputs"
                    </label>
                    <label>
                        <input
                            type="checkbox"
                            prop:checked=move || enable_collision_detection.get().unwrap_or(false)
                            on:change=move |ev| {
                                set_enable_collision_detection.set(Some(event_target_checked(&ev)))
                            }
                        />
                        "Enable Collision"
                    </label>
                </div>
            </div>

            <div class="viewer-wrap">
                <ViewerContainer
                    ion_token=ion_token
                    animation=false
                    timeline=false
                    should_animate=false
                    style="width: 100%; height: 100%; position: relative; overflow: hidden;".to_string()
                >
                    <Entity name=Some("Statue of Liberty".to_string()) position=Some(DVec3::new(-74.0445, 40.6892, 10.0))>
                        <PointGraphics pixel_size=12.0 color=Some(Srgba::new(0.95, 0.3, 0.2, 1.0)) />
                    </Entity>
                    <Entity name=Some("San Francisco".to_string()) position=Some(DVec3::new(-122.4194, 37.7749, 10.0))>
                        <PointGraphics pixel_size=12.0 color=Some(Srgba::new(0.2, 0.75, 0.95, 1.0)) />
                    </Entity>
                    <Entity name=Some("Tokyo".to_string()) position=Some(DVec3::new(139.7671, 35.6812, 10.0))>
                        <PointGraphics pixel_size=12.0 color=Some(Srgba::new(0.2, 0.9, 0.55, 1.0)) />
                    </Entity>

                    {move || {
                        apply_set_view.get().then(|| view! {
                            <CameraSetView
                                destination=set_view_destination
                                orientation=set_view_orientation
                            />
                        })
                    }}

                    {move || {
                        apply_fly_to.get().then(|| view! {
                            <CameraFlyTo
                                destination=fly_to_destination
                                orientation=fly_to_orientation
                                duration=fly_to_duration
                            />
                        })
                    }}

                    <CameraFlyHome trigger=fly_home_trigger duration=1.5 />

                    <CameraMove
                        trigger=move_trigger
                        direction=move_direction
                        amount=move_amount
                    />

                    <CameraZoom
                        trigger=zoom_trigger
                        direction=zoom_direction
                        amount=zoom_amount
                    />

                    {move || {
                        mount_controller.get().then(|| view! {
                            <CameraController
                                enable_inputs=enable_inputs
                                enable_collision_detection=enable_collision_detection
                                inertia_spin=0.0
                                inertia_translate=0.0
                                inertia_zoom=0.0
                            />
                        })
                    }}
                </ViewerContainer>
            </div>
        </div>
    }
}
