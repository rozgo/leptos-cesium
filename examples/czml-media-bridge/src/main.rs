mod scenario;

#[cfg(target_arch = "wasm32")]
mod stream_timer;

use leptos::prelude::*;
use leptos::wasm_bindgen::JsValue;
use leptos_cesium::prelude::*;

#[cfg(target_arch = "wasm32")]
use scenario::STREAM_INTERVAL_MS;
use scenario::{build_append_packet, initial_append_step, media_demo_czml};
#[cfg(target_arch = "wasm32")]
use stream_timer::StreamTimer;

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

    let (data_status, set_data_status) = signal("Waiting for CZML load".to_string());
    let (media_status, set_media_status) = signal("Waiting for media reconciliation".to_string());
    let (data_error, set_data_error) = signal(Option::<String>::None);
    let (media_error, set_media_error) = signal(Option::<String>::None);

    #[cfg(target_arch = "wasm32")]
    let stream_interval = StoredValue::new_local(None::<StreamTimer>);

    let on_packet_loading = Callback::new(move |loading: bool| {
        if loading {
            set_data_error.set(None);
            let message = if loaded_data_source.get_untracked().is_some() {
                "Processing CZML packet"
            } else {
                "Loading base CZML"
            };
            set_data_status.set(message.to_string());
        }
    });

    let on_media_loading = Callback::new(move |loading: bool| {
        if loading {
            set_media_error.set(None);
            set_media_status.set("Reconciling media".to_string());
        } else if media_error.get_untracked().is_none() {
            set_media_status.set("Media ready".to_string());
        }
    });

    let on_packet_error = Callback::new(move |message: String| {
        set_data_error.set(Some(message));
        set_data_status.set("CZML loading failed".to_string());
    });

    let on_media_error = Callback::new(move |error: CzmlMediaError| {
        let prefix = error
            .entity_id
            .as_deref()
            .map(|id| format!("[{}] ", id))
            .unwrap_or_default();
        set_media_error.set(Some(format!("{}{}", prefix, error.reason)));
        set_media_status.set("Media assignment failed".to_string());
    });

    let on_packet_loaded = Callback::new(move |value: JsValue| {
        let first_load = loaded_data_source.get_untracked().is_none();
        loaded_data_source.set(Some(value.clone()));

        if first_load {
            loaded_target.set(Some(ViewerTarget::from(value.clone())));
            set_focus_trigger.set(());
            set_data_status.set("Base CZML loaded".to_string());
        } else {
            set_data_status.set(format!(
                "Append packet processed (step {})",
                append_step.get_untracked()
            ));
        }
    });

    let push_packet = {
        move || {
            if loaded_data_source.get_untracked().is_none() {
                set_data_status.set("Waiting for initial load before append".to_string());
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
                    set_data_status
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
                        set_data_error
                            .set(Some(format!("Failed to start stream timer: {}", message)));
                        return;
                    }
                }

                // Prime a future buffer so Cesium mostly interpolates between known samples.
                for _ in 0..LOOKAHEAD_PRIME_STEPS {
                    push_packet();
                }

                set_is_streaming.set(true);
                set_data_status.set("Streaming append packets".to_string());
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
                <div class="status">{move || data_status.get()}</div>
                <div class="status">{move || media_status.get()}</div>
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
                    data_error.get().map(|message| {
                        view! { <div class="error">{message}</div> }
                    })
                }}
                {move || {
                    media_error.get().map(|message| {
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
                    clear_existing=false
                    trigger=packet_trigger
                    on_loading=on_packet_loading
                    on_error=on_packet_error
                    on_media_loading=on_media_loading
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
