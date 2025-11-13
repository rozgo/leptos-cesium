use leptos::prelude::*;
use leptos_cesium::prelude::*;

/// Helper component to clear data sources when a condition is true
#[component(transparent)]
fn ClearDataSources<F>(when: F) -> impl IntoView
where
    F: Fn() -> bool + 'static,
{
    #[cfg(target_arch = "wasm32")]
    {
        use leptos_cesium::components::use_cesium_context;

        Effect::new(move |_| {
            if when() {
                if let Some(viewer_ctx) = use_cesium_context() {
                    viewer_ctx.with_viewer(|viewer: Viewer| {
                        viewer.data_sources().remove_all();
                    });
                }
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = when;
    }
}

#[component]
fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());

    // Signals to control which GeoJSON file to load
    let (geojson_url, set_geojson_url) = signal("SampleData/countries.geojson".to_string());

    // Styling options
    let (use_custom_style, set_use_custom_style) = signal(true);
    let (clamp_to_ground, set_clamp_to_ground) = signal(false);

    // Button handlers
    let on_countries = move |_| {
        set_geojson_url.set("SampleData/countries.geojson".to_string());
    };

    let on_cities = move |_| {
        set_geojson_url.set("SampleData/cities.geojson".to_string());
    };

    let on_rivers = move |_| {
        set_geojson_url.set("SampleData/rivers.geojson".to_string());
    };

    let on_reset = move |_| {
        set_geojson_url.set("".to_string());
    };

    view! {
        <div style="width: 100%; height: 100%; position: relative;">
            <div class="controls">
                <h3>"GeoJSON Layers"</h3>
                <div class="button-group">
                    <button
                        class:active=move || geojson_url.get() == "SampleData/countries.geojson"
                        on:click=on_countries
                    >
                        "Countries"
                    </button>
                    <button
                        class:active=move || geojson_url.get() == "SampleData/cities.geojson"
                        on:click=on_cities
                    >
                        "Cities"
                    </button>
                    <button
                        class:active=move || geojson_url.get() == "SampleData/rivers.geojson"
                        on:click=on_rivers
                    >
                        "Rivers"
                    </button>
                    <button on:click=on_reset>"Clear"</button>
                </div>

                <div class="style-controls">
                    <label>"Custom Styling:"</label>
                    <input
                        type="checkbox"
                        checked=use_custom_style
                        on:change=move |ev| set_use_custom_style.set(event_target_checked(&ev))
                    />

                    <label>"Clamp to Ground:"</label>
                    <input
                        type="checkbox"
                        checked=clamp_to_ground
                        on:change=move |ev| set_clamp_to_ground.set(event_target_checked(&ev))
                    />
                </div>
            </div>

            <ViewerContainer
                ion_token=ion_token
                style="width: 100%; height: 100%;".to_string()
            >
                // Manual cleanup when URL is cleared
                <ClearDataSources when=move || geojson_url.get().is_empty() />

                // Declaratively load GeoJSON when URL changes
                {move || {
                    let url = geojson_url.get();
                    let use_style = use_custom_style.get();
                    let clamp = clamp_to_ground.get();

                    (!url.is_empty()).then(|| {
                        if use_style {
                            // Different styling based on the dataset
                            if url.contains("countries") {
                                view! {
                                    <GeoJsonDataSource
                                        url=url
                                        stroke=Color::blue()
                                        stroke_width=2.0
                                        fill=Color::cyan().with_alpha(0.3)
                                        clamp_to_ground=Some(clamp)
                                    />
                                }
                            } else if url.contains("cities") {
                                view! {
                                    <GeoJsonDataSource
                                        url=url
                                        marker_color=Color::red()
                                        marker_size=24.0
                                        clamp_to_ground=Some(clamp)
                                    />
                                }
                            } else {
                                view! {
                                    <GeoJsonDataSource
                                        url=url
                                        stroke=Color::deepskyblue()
                                        stroke_width=3.0
                                        clamp_to_ground=Some(clamp)
                                    />
                                }
                            }
                        } else {
                            // Default Cesium styling
                            view! {
                                <GeoJsonDataSource
                                    url=url
                                    clamp_to_ground=Some(clamp)
                                />
                            }
                        }
                    })
                }}

                // Fly to a nice initial view
                <CameraSetView
                    destination=Cartesian3::from_degrees(0.0, 30.0, 20000000.0)
                    orientation=Some(HeadingPitchRoll::new(0.0, -1.57, 0.0))
                />
            </ViewerContainer>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> });
}
