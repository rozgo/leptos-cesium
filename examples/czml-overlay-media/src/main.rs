mod scenario;

use leptos::prelude::*;
use leptos::wasm_bindgen::JsValue;
use leptos_cesium::prelude::*;

use scenario::media_demo_czml;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());
    let packet = media_demo_czml();
    let media_pointer_events = RwSignal::new(false);
    let loaded_target = JsRwSignal::new_local(None::<ViewerTarget>);
    let (focus_trigger, set_focus_trigger) = signal(());

    let on_packet_error = Callback::new(move |message: String| {
        leptos::logging::error!("CZML overlay media example failed to load: {}", message);
    });

    let on_media_error = Callback::new(move |error: CzmlMediaError| {
        let prefix = error
            .entity_id
            .as_deref()
            .map(|id| format!("[{}] ", id))
            .unwrap_or_default();
        leptos::logging::error!(
            "CZML overlay media example failed to reconcile media: {}{}",
            prefix,
            error.reason
        );
    });

    let on_packet_loaded = Callback::new(move |value: JsValue| {
        if loaded_target.get_untracked().is_none() {
            loaded_target.set(Some(ViewerTarget::from(value.clone())));
            set_focus_trigger.set(());
        }
    });

    view! {
        <div style="width: 100%; height: 100%; position: relative;">
            <div class="panel">
                "Overlay media driven by CZML entity positions"
                <br />
                <small>
                    "Flattened properties.media_* fields describe image, native video, YouTube, and Rerun overlays."
                </small>
                <br />
                <small>
                    "Each overlay follows the entity.position samples instead of mutating Cesium billboard or rectangle graphics."
                </small>
                <br />
                <small>
                    {move || {
                        if media_pointer_events.get() {
                            "Overlay pointer is on: resize handles are visible and draggable."
                        } else {
                            "Overlay pointer is off: handles are hidden so map drag stays unobstructed."
                        }
                    }}
                </small>
                <br />
                <button
                    on:click=move |_| media_pointer_events.update(|value| *value = !*value)
                    style="margin-top: 8px;"
                >
                    {move || {
                        if media_pointer_events.get() {
                            "Disable overlay pointer"
                        } else {
                            "Enable overlay pointer"
                        }
                    }}
                </button>
            </div>

            <ViewerContainer
                ion_token=ion_token
                animation=true
                timeline=true
                style="width: 100%; height: 100%;".to_string()
            >
                <CzmlDataSource
                    data=Some(packet)
                    media_overlay_pointer_events=media_pointer_events
                    on_error=on_packet_error
                    on_media_error=on_media_error
                    on_loaded=on_packet_loaded
                />

                <ViewerFlyToTarget
                    trigger=focus_trigger
                    target=loaded_target
                    duration=1.8
                />
            </ViewerContainer>
        </div>
    }
}
