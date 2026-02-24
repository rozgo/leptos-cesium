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
            info_box=false
            selection_indicator=false
            style="width: 100%; height: 100%;".to_string()
        >
            <Entity
                name="Golden Gate".to_string()
                position=DVec3::new(-122.4786, 37.8194, 35.0)
            >
                <BillboardGraphics
                    image=Some(MediaSource::Url("pin.svg".to_string()))
                    scale=Some(0.25)
                    vertical_origin=Some(VerticalOrigin::Bottom)
                />
            </Entity>

            <CameraSetView destination=Some(DVec3::new(-122.4786, 37.8194, 3500.0).into()) />
        </ViewerContainer>
    }
}
