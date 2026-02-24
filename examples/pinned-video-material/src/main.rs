use leptos::prelude::*;
use leptos_cesium::prelude::*;

#[cfg(target_arch = "wasm32")]
use leptos::wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use leptos::web_sys;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());
    let video_material = JsRwSignal::new_local(None::<Material>);
    let (video_error, set_video_error) = signal(Option::<String>::None);

    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            if video_material.get_untracked().is_some() {
                return;
            }

            let Some(document) = web_sys::window().and_then(|w| w.document()) else {
                set_video_error.set(Some("Window/document not available".to_string()));
                return;
            };

            let Ok(element) = document.create_element("video") else {
                set_video_error.set(Some("Failed to create video element".to_string()));
                return;
            };

            let Ok(video) = element.dyn_into::<web_sys::HtmlVideoElement>() else {
                set_video_error.set(Some("Failed to cast video element".to_string()));
                return;
            };

            video.set_autoplay(true);
            video.set_loop(true);
            video.set_muted(true);
            let _ = video.set_attribute("playsinline", "");
            let _ = video.set_attribute("crossorigin", "anonymous");
            video.set_src(
                "https://cesium.com/public/SandcastleSampleData/big-buck-bunny_trailer.mp4",
            );
            video.load();

            if video.play().is_err() {
                set_video_error.set(Some(
                    "Video autoplay was blocked; click the globe and try again".to_string(),
                ));
            }

            let image_material = ImageMaterialPropertyBuilder::new()
                .image(MediaSource::HtmlVideo(video))
                .build();
            video_material.set(Some(Material::image(image_material)));
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (video_material, set_video_error);
        }
    });

    view! {
        <div style="width: 100%; height: 100%; position: relative;">
            <div class="overlay">
                "Video texture material on RectangleGraphics"
                <br />
                <small>
                    "Source: Cesium Sandcastle Big Buck Bunny trailer"
                </small>
                {move || {
                    video_error.get().map(|message| {
                        view! { <div class="error">{message}</div> }
                    })
                }}
            </div>

            <ViewerContainer
                ion_token=ion_token
                animation=false
                timeline=false
                info_box=false
                selection_indicator=false
                style="width: 100%; height: 100%;".to_string()
            >
                <Entity name="Video rectangle".to_string()>
                    <RectangleGraphics
                        coordinates=Rect::new(
                            Coord {
                                x: -122.455,
                                y: 37.800,
                            },
                            Coord {
                                x: -122.438,
                                y: 37.810,
                            },
                        )
                        height=Some(8.0)
                        material=video_material
                        outline=Some(true)
                        outline_color=Some(Srgba::new(0.9, 0.9, 0.9, 0.9))
                    />
                </Entity>

                <CameraSetView destination=Some(DVec3::new(-122.4465, 37.8050, 3500.0).into()) />
            </ViewerContainer>
        </div>
    }
}
