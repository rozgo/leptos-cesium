use leptos::prelude::*;
use leptos_cesium::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());
    let anchor = DVec3::new(-122.4465, 37.8050, 20.0);

    view! {
        <div style="width: 100%; height: 100%; position: relative;">
            <div class="hud">
                "Official YouTube iframe pinned to a globe anchor"
                <br />
                <small>
                    "The iframe stays aligned to a lon/lat/height position while the globe moves."
                </small>
            </div>

            <ViewerContainer
                ion_token=ion_token
                animation=false
                timeline=false
                info_box=false
                selection_indicator=false
                style="width: 100%; height: 100%;".to_string()
            >
                <Entity name="YouTube anchor".to_string() position=anchor>
                    <PointGraphics
                        pixel_size=14.0
                        color=Some(Srgba::new(0.95, 0.29, 0.22, 1.0))
                    />
                </Entity>

                <YouTubeOverlay
                    video_id="M7lc1UVf-VE".to_string()
                    position=DVec3::new(anchor.x, anchor.y, 140.0)
                    width_px=420_u32
                    height_px=236_u32
                />

                <CameraSetView destination=Some(DVec3::new(anchor.x, anchor.y, 3800.0).into()) />
            </ViewerContainer>
        </div>
    }
}
