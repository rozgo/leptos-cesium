use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::JsCast;
use leptos::wasm_bindgen::JsValue;
use leptos_cesium::prelude::*;

#[component]
fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());

    // Signals to control which CZML file to load
    // Start with satellites loaded by default
    let (czml_url, set_czml_url) = signal("SampleData/simple.czml".to_string());

    // Signals to control viewer-level data-source focus behavior
    let loaded_target = JsRwSignal::new_local(None::<ViewerTarget>);
    let loaded_data_source = JsRwSignal::new_local(None::<DataSource>);
    let (focus_loaded_trigger, set_focus_loaded_trigger) = signal(());
    let (track_clock_trigger, set_track_clock_trigger) = signal(());

    // Signal to trigger fly home
    let (fly_home_trigger, set_fly_home_trigger) = signal(());

    // Trigger initial fly home after a brief delay to ensure viewer is ready
    Effect::new(move |_| {
        set_fly_home_trigger.set(());
    });

    let on_loaded = Callback::new(move |value: JsValue| {
        loaded_target.set(Some(ViewerTarget::from(value.clone())));

        #[cfg(target_arch = "wasm32")]
        {
            let data_source = value.dyn_into::<DataSource>().ok();
            loaded_data_source.set(data_source);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = value;
            loaded_data_source.set(None);
        }

        set_focus_loaded_trigger.set(());
        set_track_clock_trigger.set(());
    });

    // Button handlers
    let on_satellites = move |_| {
        loaded_target.set(None);
        loaded_data_source.set(None);
        set_track_clock_trigger.set(());
        set_czml_url.set("SampleData/simple.czml".to_string());
    };

    let on_vehicle = move |_| {
        loaded_target.set(None);
        loaded_data_source.set(None);
        set_track_clock_trigger.set(());
        set_czml_url.set("SampleData/vehicle.czml".to_string());
    };

    let on_reset = move |_| {
        set_czml_url.set("".to_string());
        loaded_target.set(None);
        loaded_data_source.set(None);
        set_track_clock_trigger.set(());
        set_fly_home_trigger.set(());
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
                        <CzmlDataSource
                            url=url
                            clear_existing=true
                            on_loaded=on_loaded
                        />
                    })
                }}

                // Declaratively focus the most recently loaded data source.
                <ViewerFlyToTarget
                    trigger=focus_loaded_trigger
                    target=loaded_target
                    duration=2.2
                />

                // Declaratively wire viewer clock tracking to the current data source.
                <ViewerSetClockTrackedDataSource
                    trigger=track_clock_trigger
                    data_source=loaded_data_source
                />

                // Declaratively control camera - fly home when satellites button clicked
                <CameraFlyHome trigger=fly_home_trigger duration=0.0 />
            </ViewerContainer>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> });
}
