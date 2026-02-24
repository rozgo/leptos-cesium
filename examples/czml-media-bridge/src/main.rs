mod scenario;

#[cfg(target_arch = "wasm32")]
mod stream_timer;
#[cfg(target_arch = "wasm32")]
mod video_material;

use leptos::prelude::*;
use leptos::wasm_bindgen::JsValue;
use leptos_cesium::prelude::*;

#[cfg(target_arch = "wasm32")]
use scenario::STREAM_INTERVAL_MS;
use scenario::{build_append_packet, initial_append_step, media_demo_czml};
#[cfg(target_arch = "wasm32")]
use stream_timer::StreamTimer;
#[cfg(target_arch = "wasm32")]
use video_material::apply_video_material_from_czml;

const LOOKAHEAD_PRIME_STEPS: usize = 4;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());

    let (packet, set_packet) = signal(Some(media_demo_czml()));
    let (packet_mode, set_packet_mode) = signal(CzmlLoadMode::Replace);
    let (packet_trigger, set_packet_trigger) = signal(());

    let loaded_target = JsRwSignal::new_local(None::<ViewerTarget>);
    let loaded_data_source = JsRwSignal::new_local(None::<JsValue>);

    let (focus_trigger, set_focus_trigger) = signal(());
    let (append_step, set_append_step) = signal(initial_append_step());
    let (is_streaming, set_is_streaming) = signal(false);

    let (video_status, set_video_status) = signal("Waiting for CZML load".to_string());
    let (video_error, set_video_error) = signal(Option::<String>::None);

    #[cfg(target_arch = "wasm32")]
    let stream_interval = StoredValue::new_local(None::<StreamTimer>);

    let apply_video_now = {
        move |data_source: &JsValue, context: &str| {
            #[cfg(target_arch = "wasm32")]
            {
                match apply_video_material_from_czml(data_source) {
                    Ok(()) => {
                        set_video_error.set(None);
                        set_video_status.set(format!("Video material applied ({context})"));
                    }
                    Err(message) => {
                        set_video_error.set(Some(message.clone()));
                        set_video_status.set(format!("Video assignment failed ({context})"));
                    }
                }
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (data_source, context);
                set_video_status.set("Video assignment is wasm-only".to_string());
            }
        }
    };

    let on_packet_loaded = Callback::new(move |value: JsValue| {
        let first_load = loaded_data_source.get_untracked().is_none();
        loaded_data_source.set(Some(value.clone()));

        if first_load {
            loaded_target.set(Some(ViewerTarget::from(value.clone())));
            apply_video_now(&value, "initial load");
            set_focus_trigger.set(());
        } else {
            set_video_status.set(format!(
                "Append packet processed (step {})",
                append_step.get_untracked()
            ));
        }
    });

    let push_packet = {
        move || {
            if loaded_data_source.get_untracked().is_none() {
                set_video_status.set("Waiting for initial load before append".to_string());
                return;
            }

            let next_step = append_step.get_untracked() + 1;
            set_append_step.set(next_step);
            set_packet_mode.set(CzmlLoadMode::Append);
            set_packet.set(Some(build_append_packet(next_step)));
            set_packet_trigger.set(());
        }
    };

    let step_once = {
        let push_packet = push_packet.clone();
        move |_| {
            push_packet();
        }
    };

    let start_stream = {
        let push_packet = push_packet.clone();
        move |_| {
            #[cfg(target_arch = "wasm32")]
            {
                if is_streaming.get_untracked() {
                    return;
                }

                if loaded_data_source.get_untracked().is_none() {
                    set_video_status
                        .set("Waiting for initial load before starting stream".to_string());
                    return;
                }

                let push_packet_for_timer = push_packet.clone();
                match StreamTimer::start(STREAM_INTERVAL_MS, move || {
                    push_packet_for_timer();
                }) {
                    Ok(timer) => {
                        stream_interval.update_value(|slot| {
                            *slot = Some(timer);
                        });
                    }
                    Err(message) => {
                        set_video_error
                            .set(Some(format!("Failed to start stream timer: {}", message)));
                        return;
                    }
                }

                // Prime a future buffer so Cesium mostly interpolates between known samples.
                for _ in 0..LOOKAHEAD_PRIME_STEPS {
                    push_packet();
                }

                set_is_streaming.set(true);
                set_video_status.set("Streaming append packets".to_string());
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = push_packet;
            }
        }
    };

    let stop_stream = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            stream_interval.update_value(stream_timer::clear_timer_slot);
        }

        set_is_streaming.set(false);
    };

    let on_reapply_video = move |_| {
        if let Some(data_source) = loaded_data_source.get_untracked() {
            apply_video_now(&data_source, "manual reapply");
        }
    };

    on_cleanup(move || {
        #[cfg(target_arch = "wasm32")]
        {
            stream_interval.update_value(stream_timer::clear_timer_slot);
        }
    });

    view! {
        <div style="width: 100%; height: 100%; position: relative;">
            <div class="panel">
                "CZML Media Stream"
                <div class="status">{move || video_status.get()}</div>
                <div class="status">
                    {move || {
                        format!(
                            "stream: {} | append step: {}",
                            if is_streaming.get() { "on" } else { "off" },
                            append_step.get()
                        )
                    }}
                </div>
                {move || {
                    video_error.get().map(|message| {
                        view! { <div class="error">{message}</div> }
                    })
                }}
                <button on:click=step_once disabled=move || is_streaming.get()>
                    "Append Step"
                </button>
                <button on:click=start_stream disabled=move || is_streaming.get()>
                    "Start Stream"
                </button>
                <button on:click=stop_stream disabled=move || !is_streaming.get()>
                    "Stop Stream"
                </button>
                <button on:click=on_reapply_video>
                    "Reapply Video"
                </button>
            </div>

            <ViewerContainer
                ion_token=ion_token
                animation=true
                timeline=true
                style="width: 100%; height: 100%;".to_string()
            >
                <CzmlDataSource
                    data=packet
                    mode=packet_mode
                    clear_existing=true
                    trigger=packet_trigger
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
