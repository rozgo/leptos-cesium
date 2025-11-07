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
            <Entity
                name=Some("Statue of Liberty".to_string())
                position=Some(cartesian3_from_degrees(-74.0445, 40.6892, 150.0))
            >
                <PointGraphics
                    pixel_size=12.0
                    color=Some(Color::red())
                />
            </Entity>
        </ViewerContainer>
    }
}
