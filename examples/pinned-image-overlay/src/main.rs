use leptos::prelude::*;
use leptos_cesium::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());
    let anchor = DVec3::new(-122.4786, 37.8194, 25.0);

    view! {
        <div style="width: 100%; height: 100%; position: relative;">
            <div class="hud">
                "Native HTML image pinned to a globe anchor"
                <br />
                <small>
                    "The browser renders the <img> element while Cesium keeps the anchor aligned to lon/lat/height."
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
                <Entity name="Image anchor".to_string() position=anchor>
                    <PointGraphics
                        pixel_size=14.0
                        color=Some(Srgba::new(0.31, 0.85, 0.56, 1.0))
                    />
                </Entity>

                <ImageOverlay
                    src="pin.svg".to_string()
                    position=DVec3::new(anchor.x, anchor.y, 150.0)
                    width_px=160_u32
                    height_px=200_u32
                    alt=Some("Gradient location pin".to_string())
                />

                <CameraSetView destination=Some(DVec3::new(anchor.x, anchor.y, 3500.0).into()) />
            </ViewerContainer>
        </div>
    }
}
