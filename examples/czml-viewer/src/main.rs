use leptos::prelude::*;
use leptos_cesium::prelude::*;

#[component]
fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());

    // Signals to control which CZML file to load
    let (czml_url, set_czml_url) = signal("".to_string());

    // Signal to trigger fly home
    let (fly_home_trigger, set_fly_home_trigger) = signal(());

    // Signal to control vehicle camera view
    let (show_vehicle_camera, set_show_vehicle_camera) = signal(false);

    // Button handlers
    let on_satellites = move |_| {
        set_czml_url.set("SampleData/simple.czml".to_string());
        set_show_vehicle_camera.set(false);
        set_fly_home_trigger.set(());
    };

    let on_vehicle = move |_| {
        set_czml_url.set("SampleData/Vehicle.czml".to_string());
        set_show_vehicle_camera.set(true);
    };

    let on_reset = move |_| {
        set_czml_url.set("".to_string());
        set_show_vehicle_camera.set(false);
    };

    view! {
        <div style="width: 100%; height: 100%; position: relative;">
            <div class="controls">
                <button on:click=on_satellites>"Satellites"</button>
                <button on:click=on_vehicle>"Vehicle"</button>
                <button on:click=on_reset>"Reset"</button>
            </div>
            <ViewerContainer
                ion_token=ion_token
                animation=true
                timeline=true
                style="width: 100%; height: 100%;".to_string()
            >
                // Declaratively load CZML when URL changes
                {move || {
                    let url = czml_url.get();
                    (!url.is_empty()).then(|| view! {
                        <CzmlDataSource url=url clear_existing=true />
                    })
                }}

                // Declaratively control camera - fly home when satellites button clicked
                <CameraFlyHome trigger=fly_home_trigger duration=0.0 />

                // Declaratively set camera view for vehicle
                {move || {
                    show_vehicle_camera.get().then(|| view! {
                        <CameraSetView
                            destination=Some(cartesian3_from_degrees(-116.52, 35.02, 95000.0))
                            heading=Some(6.0)
                        />
                    })
                }}
            </ViewerContainer>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> });
}
