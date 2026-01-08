use leptos::prelude::*;
use leptos_cesium::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());

    view! {
        <ViewerContainer
            ion_token=ion_token
            animation=false
            timeline=false
            class="cesium-viewer".to_string()
            style="width: 100%; height: 100%;".to_string()
        >
            // Load Google Photorealistic 3D Tiles
            // Uses Cesium Ion asset 2275207 by default (requires Ion token with Google 3D Tiles access)
            <GooglePhotorealistic3DTiles
                cache_bytes=1536000000
                enable_collision=true
            />

            // Fly to San Francisco for a nice view of the 3D buildings
            // DVec3: x=longitude, y=latitude, z=height (degrees/meters)
            <CameraFlyTo
                destination=DVec3::new(-122.4194, 37.7749, 800.0)
                duration=3.0
            />
        </ViewerContainer>
    }
}
