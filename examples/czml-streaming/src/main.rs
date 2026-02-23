use leptos::prelude::*;
use leptos::wasm_bindgen::JsValue;
use leptos_cesium::prelude::*;
use serde_json::json;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, closure::Closure};

#[cfg(target_arch = "wasm32")]
const STREAM_INTERVAL_MS: i32 = 700;
const STREAM_INTERVAL_SEC: f64 = 0.7;

const CLOCK_START_ISO: &str = "2026-01-01T00:00:00Z";
const CLOCK_INTERVAL_ISO: &str = "2026-01-01T00:00:00Z/2026-01-01T12:00:00Z";

const TRACK_ID_DRIVER: &str = "ride_live_driver";
const TRACK_ID_ROUTE: &str = "ride_live_route";
const TRACK_ID_PICKUP: &str = "ride_live_pickup";
const TRACK_ID_DROPOFF: &str = "ride_live_dropoff";

const ROUTE_POINTS: &[(f64, f64, f64)] = &[
    (-122.4786, 37.8194, 10.0),
    (-122.4758, 37.8078, 5.0),
    (-122.4700, 37.8065, 5.0),
    (-122.4652, 37.8045, 5.0),
    (-122.4550, 37.8020, 5.0),
    (-122.4505, 37.8008, 5.0),
    (-122.4400, 37.7998, 5.0),
    (-122.4300, 37.7995, 5.0),
    (-122.4220, 37.7998, 5.0),
    (-122.4196, 37.8022, 5.0),
    (-122.4189, 37.80175, 5.0),
    (-122.41925, 37.80135, 5.0),
    (-122.4187, 37.80095, 5.0),
    (-122.41905, 37.80055, 5.0),
    (-122.4185, 37.80015, 5.0),
    (-122.41885, 37.79975, 5.0),
    (-122.4183, 37.7994, 5.0),
    (-122.4140, 37.7995, 5.0),
    (-122.4118, 37.8010, 5.0),
    (-122.4112, 37.8045, 5.0),
    (-122.4105, 37.8080, 5.0),
];

#[cfg(target_arch = "wasm32")]
struct StreamInterval {
    id: i32,
    _callback: Closure<dyn FnMut()>,
}

#[cfg(not(target_arch = "wasm32"))]
struct StreamInterval;

fn route_start() -> (f64, f64, f64) {
    ROUTE_POINTS[0]
}

fn route_end() -> (f64, f64, f64) {
    ROUTE_POINTS[ROUTE_POINTS.len() - 1]
}

fn flatten_positions(points: &[(f64, f64, f64)]) -> Vec<f64> {
    let mut out = Vec::with_capacity(points.len() * 3);
    for (lon, lat, height) in points {
        out.push(*lon);
        out.push(*lat);
        out.push(*height);
    }
    out
}

fn haversine_m(from: (f64, f64, f64), to: (f64, f64, f64)) -> f64 {
    let r = 6_371_000.0_f64;
    let (lon1, lat1) = (from.0.to_radians(), from.1.to_radians());
    let (lon2, lat2) = (to.0.to_radians(), to.1.to_radians());
    let dlon = lon2 - lon1;
    let dlat = lat2 - lat1;

    let a = (dlat * 0.5).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon * 0.5).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    r * c
}

fn cumulative_route_distances() -> Vec<f64> {
    let mut cumulative = Vec::with_capacity(ROUTE_POINTS.len());
    cumulative.push(0.0);

    for index in 1..ROUTE_POINTS.len() {
        let prev = ROUTE_POINTS[index - 1];
        let current = ROUTE_POINTS[index];
        let segment = haversine_m(prev, current);
        let running = cumulative[index - 1] + segment;
        cumulative.push(running);
    }

    cumulative
}

fn sample_route_position(distance_m: f64, cumulative: &[f64]) -> (f64, f64, f64) {
    if ROUTE_POINTS.is_empty() {
        return (0.0, 0.0, 0.0);
    }

    if distance_m <= 0.0 {
        return route_start();
    }

    let total = cumulative.last().copied().unwrap_or(0.0);
    if distance_m >= total {
        return route_end();
    }

    for index in 1..cumulative.len() {
        if cumulative[index] >= distance_m {
            let start = ROUTE_POINTS[index - 1];
            let end = ROUTE_POINTS[index];
            let start_d = cumulative[index - 1];
            let end_d = cumulative[index];
            let segment_d = (end_d - start_d).max(1e-9);
            let t = ((distance_m - start_d) / segment_d).clamp(0.0, 1.0);

            return (
                start.0 + (end.0 - start.0) * t,
                start.1 + (end.1 - start.1) * t,
                start.2 + (end.2 - start.2) * t,
            );
        }
    }

    route_end()
}

fn heading_degrees(from: (f64, f64, f64), to: (f64, f64, f64)) -> f64 {
    let (lon1, lat1) = (from.0.to_radians(), from.1.to_radians());
    let (lon2, lat2) = (to.0.to_radians(), to.1.to_radians());
    let dlon = lon2 - lon1;

    let y = dlon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * dlon.cos();
    let mut heading = y.atan2(x).to_degrees();
    if heading < 0.0 {
        heading += 360.0;
    }
    heading
}

fn format_eta(seconds: f64) -> String {
    let clamped = seconds.max(0.0);
    let minutes = (clamped / 60.0).floor() as u64;
    let rem_seconds = (clamped as u64) % 60;
    format!("{:02}:{:02}", minutes, rem_seconds)
}

fn build_static_packet() -> String {
    let route_positions = flatten_positions(ROUTE_POINTS);
    let pickup = route_start();
    let dropoff = route_end();

    json!([
        { "id": "document", "version": "1.0" },
        {
            "id": TRACK_ID_ROUTE,
            "name": "Route Preview",
            "polyline": {
                "positions": { "cartographicDegrees": route_positions },
                "width": 3,
                "material": {
                    "polylineGlow": {
                        "color": { "rgba": [90, 150, 230, 170] },
                        "glowPower": 0.18
                    }
                },
                "clampToGround": false
            }
        },
        {
            "id": TRACK_ID_PICKUP,
            "name": "Pickup",
            "point": {
                "pixelSize": 12,
                "color": { "rgba": [72, 219, 131, 255] },
                "outlineColor": { "rgba": [8, 12, 18, 255] },
                "outlineWidth": 2
            },
            "label": {
                "text": "pickup",
                "font": "12px sans-serif",
                "style": "FILL_AND_OUTLINE",
                "outlineWidth": 2,
                "pixelOffset": { "cartesian2": [0, -16] }
            },
            "position": { "cartographicDegrees": [pickup.0, pickup.1, pickup.2] }
        },
        {
            "id": TRACK_ID_DROPOFF,
            "name": "Dropoff",
            "point": {
                "pixelSize": 12,
                "color": { "rgba": [255, 188, 92, 255] },
                "outlineColor": { "rgba": [8, 12, 18, 255] },
                "outlineWidth": 2
            },
            "label": {
                "text": "dropoff",
                "font": "12px sans-serif",
                "style": "FILL_AND_OUTLINE",
                "outlineWidth": 2,
                "pixelOffset": { "cartesian2": [0, -16] }
            },
            "position": { "cartographicDegrees": [dropoff.0, dropoff.1, dropoff.2] }
        }
    ])
    .to_string()
}

fn build_dynamic_bootstrap_packet(total_route_m: f64) -> String {
    let start = route_start();
    let eta_text = format_eta(total_route_m / 9.0);
    let label = format!("ride #1\\n0.0 km/h | ETA {eta_text}\\nhdg 0 deg");

    json!([
        {
            "id": "document",
            "version": "1.0",
            "clock": {
                "interval": CLOCK_INTERVAL_ISO,
                "currentTime": CLOCK_START_ISO,
                "multiplier": 1,
                "range": "CLAMPED",
                "step": "SYSTEM_CLOCK_MULTIPLIER"
            }
        },
        {
            "id": TRACK_ID_DRIVER,
            "name": "Live Driver",
            "point": {
                "pixelSize": 14,
                "color": { "rgba": [255, 96, 96, 255] },
                "outlineColor": { "rgba": [8, 12, 18, 255] },
                "outlineWidth": 2
            },
            "label": {
                "text": label,
                "font": "13px sans-serif",
                "style": "FILL_AND_OUTLINE",
                "outlineWidth": 2,
                "pixelOffset": { "cartesian2": [0, -22] }
            },
            "path": {
                "show": true,
                "leadTime": 0,
                "trailTime": 86400,
                "width": 4,
                "material": {
                    "solidColor": {
                        "color": { "rgba": [255, 200, 98, 240] }
                    }
                },
                "resolution": 1
            },
            "position": {
                "epoch": CLOCK_START_ISO,
                "forwardExtrapolationType": "HOLD",
                "backwardExtrapolationType": "HOLD",
                "interpolationAlgorithm": "LAGRANGE",
                "interpolationDegree": 1,
                "cartographicDegrees": [0.0, start.0, start.1, start.2]
            }
        }
    ])
    .to_string()
}

fn build_dynamic_delta_packet(
    driver: (f64, f64, f64),
    speed_mps: f64,
    remaining_m: f64,
    heading_deg: f64,
    pulse_tick: u64,
    sample_time_sec: f64,
) -> String {
    let speed_kmh = speed_mps * 3.6;
    let eta_text = if speed_mps > 0.01 {
        format_eta(remaining_m / speed_mps)
    } else {
        "--:--".to_string()
    };

    let driver_label =
        format!("ride #1\\n{speed_kmh:.1} km/h | ETA {eta_text}\\nhdg {heading_deg:.0} deg");
    let pulse_size = 14.0 + (pulse_tick % 5) as f64;

    json!([
        { "id": "document", "version": "1.0" },
        {
            "id": TRACK_ID_DRIVER,
            "point": {
                "pixelSize": pulse_size,
                "color": { "rgba": [255, 96, 96, 255] },
                "outlineColor": { "rgba": [8, 12, 18, 255] },
                "outlineWidth": 2
            },
            "label": {
                "text": driver_label,
                "font": "13px sans-serif",
                "style": "FILL_AND_OUTLINE",
                "outlineWidth": 2,
                "pixelOffset": { "cartesian2": [0, -22] }
            },
            "position": {
                "epoch": CLOCK_START_ISO,
                "cartographicDegrees": [sample_time_sec, driver.0, driver.1, driver.2]
            }
        }
    ])
    .to_string()
}

#[component]
fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());
    let start = route_start();

    let route_cumulative = cumulative_route_distances();
    let total_route_m = route_cumulative.last().copied().unwrap_or(0.0);

    let static_packet = Some(build_static_packet());

    let (is_streaming, set_is_streaming) = signal(false);
    let (loading, set_loading) = signal(false);
    let (packet_count, set_packet_count) = signal(0_u64);
    let (step, set_step) = signal(0_u64);
    let (distance_along_m, set_distance_along_m) = signal(0.0_f64);
    let (sample_time_sec, set_sample_time_sec) = signal(0.0_f64);
    let (last_heading_deg, set_last_heading_deg) = signal(0.0_f64);
    let (last_position, set_last_position) = signal((start.0, start.1));
    let (last_speed_mps, set_last_speed_mps) = signal(0.0_f64);
    let (last_eta_sec, set_last_eta_sec) = signal(total_route_m / 9.0);
    let (progress_ratio, set_progress_ratio) = signal(0.0_f64);
    let (last_error, set_last_error) = signal(String::new());

    let (packet, set_packet) = signal(Some(build_dynamic_bootstrap_packet(total_route_m)));
    let (packet_trigger, set_packet_trigger) = signal(());
    let (dynamic_mode, set_dynamic_mode) = signal(CzmlLoadMode::Replace);

    let loaded_target = JsRwSignal::new_local(None::<ViewerTarget>);
    let (focus_trigger, set_focus_trigger) = signal(());
    let (home_trigger, set_home_trigger) = signal(());

    let stream_interval = StoredValue::new_local(None::<StreamInterval>);

    let emit_packet = Callback::new(move |_| {
        let next_step = step.get_untracked() + 1;
        set_step.set(next_step);

        // Simulate variable traffic speed.
        let speed_mps = 7.5 + 5.5 * ((next_step as f64) * 0.21).sin().abs();

        let prev_distance = distance_along_m.get_untracked();
        let next_distance = (prev_distance + speed_mps * STREAM_INTERVAL_SEC).min(total_route_m);
        set_distance_along_m.set(next_distance);

        let next_time = sample_time_sec.get_untracked() + STREAM_INTERVAL_SEC;
        set_sample_time_sec.set(next_time);

        let prev_position = sample_route_position(prev_distance, &route_cumulative);
        let driver_position = sample_route_position(next_distance, &route_cumulative);

        let effective_speed_mps = if (next_distance - prev_distance).abs() < f64::EPSILON {
            0.0
        } else {
            speed_mps
        };

        let remaining_m = (total_route_m - next_distance).max(0.0);
        let eta_sec = if effective_speed_mps > 0.01 {
            remaining_m / effective_speed_mps
        } else {
            0.0
        };

        set_last_position.set((driver_position.0, driver_position.1));
        set_last_speed_mps.set(effective_speed_mps);
        set_last_eta_sec.set(eta_sec);
        set_progress_ratio.set(if total_route_m > 0.0 {
            next_distance / total_route_m
        } else {
            0.0
        });

        let heading = if (next_distance - prev_distance).abs() < f64::EPSILON {
            last_heading_deg.get_untracked()
        } else {
            heading_degrees(prev_position, driver_position)
        };
        set_last_heading_deg.set(heading);

        let delta = build_dynamic_delta_packet(
            driver_position,
            effective_speed_mps,
            remaining_m,
            heading,
            next_step,
            next_time,
        );
        set_packet.set(Some(delta));
        set_dynamic_mode.set(CzmlLoadMode::Append);

        set_packet_trigger.set(());
        set_packet_count.update(|count| *count += 1);
    });

    let on_static_loaded = Callback::new(move |value: JsValue| {
        if loaded_target.get_untracked().is_none() {
            loaded_target.set(Some(ViewerTarget::from(value)));
            set_focus_trigger.set(());
        }
    });

    let on_dynamic_loaded = Callback::new(move |_value: JsValue| {
        if dynamic_mode.get_untracked() == CzmlLoadMode::Replace {
            set_dynamic_mode.set(CzmlLoadMode::Append);
        }
    });

    let on_loading = Callback::new(move |value: bool| {
        set_loading.set(value);
    });

    let on_error = Callback::new(move |message: String| {
        set_last_error.set(message);
    });

    let start_stream = move |_| {
        if is_streaming.get_untracked() {
            return;
        }

        #[cfg(target_arch = "wasm32")]
        {
            if stream_interval.with_value(|slot| slot.is_some()) {
                set_is_streaming.set(true);
                return;
            }

            let emit = emit_packet;
            let callback = Closure::wrap(Box::new(move || {
                emit.run(());
            }) as Box<dyn FnMut()>);

            if let Some(window) = web_sys::window() {
                match window.set_interval_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    STREAM_INTERVAL_MS,
                ) {
                    Ok(id) => {
                        stream_interval.update_value(|slot| {
                            *slot = Some(StreamInterval {
                                id,
                                _callback: callback,
                            });
                        });
                        set_is_streaming.set(true);
                    }
                    Err(_) => {
                        set_last_error.set("Failed to start stream interval".to_string());
                        set_is_streaming.set(false);
                    }
                }
            } else {
                set_last_error.set("window is not available".to_string());
                set_is_streaming.set(false);
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = stream_interval;
            set_is_streaming.set(true);
        }
    };

    let stop_stream = move |_| {
        set_is_streaming.set(false);

        #[cfg(target_arch = "wasm32")]
        {
            stream_interval.update_value(|slot| {
                if let Some(interval) = slot.take()
                    && let Some(window) = web_sys::window()
                {
                    window.clear_interval_with_handle(interval.id);
                }
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = stream_interval;
        }
    };

    let step_once = move |_| {
        emit_packet.run(());
    };

    let burst_ten = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                for i in 0..10_i32 {
                    let emit = emit_packet;
                    let callback = Closure::once_into_js(move || {
                        emit.run(());
                    });

                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        callback.unchecked_ref(),
                        i * 35,
                    );
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            emit_packet.run(());
        }
    };

    let reset_stream = move |_| {
        set_is_streaming.set(false);

        #[cfg(target_arch = "wasm32")]
        {
            stream_interval.update_value(|slot| {
                if let Some(interval) = slot.take()
                    && let Some(window) = web_sys::window()
                {
                    window.clear_interval_with_handle(interval.id);
                }
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = stream_interval;
        }

        let start = route_start();
        set_step.set(0);
        set_packet_count.set(0);
        set_distance_along_m.set(0.0);
        set_sample_time_sec.set(0.0);
        set_last_heading_deg.set(0.0);
        set_last_position.set((start.0, start.1));
        set_last_speed_mps.set(0.0);
        set_last_eta_sec.set(total_route_m / 9.0);
        set_progress_ratio.set(0.0);
        set_last_error.set(String::new());

        set_dynamic_mode.set(CzmlLoadMode::Replace);
        set_packet.set(Some(build_dynamic_bootstrap_packet(total_route_m)));
        set_packet_trigger.set(());
        set_focus_trigger.set(());
    };

    on_cleanup(move || {
        #[cfg(target_arch = "wasm32")]
        {
            stream_interval.update_value(|slot| {
                if let Some(interval) = slot.take()
                    && let Some(window) = web_sys::window()
                {
                    window.clear_interval_with_handle(interval.id);
                }
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = stream_interval;
        }
    });

    view! {
        <div style="width: 100%; height: 100%; position: relative;">
            <div class="controls">
                <h2>"CZML Streaming Demo - SF Ride"</h2>

                <div class="row">
                    <button on:click=start_stream disabled=move || is_streaming.get()>
                        "Start"
                    </button>
                    <button on:click=stop_stream disabled=move || !is_streaming.get()>
                        "Stop"
                    </button>
                    <button on:click=step_once>
                        "Step"
                    </button>
                    <button on:click=burst_ten>
                        "Burst x10"
                    </button>
                    <button on:click=move |_| set_focus_trigger.set(())>
                        "Recenter"
                    </button>
                    <button on:click=move |_| set_home_trigger.set(())>
                        "Home"
                    </button>
                    <button on:click=reset_stream>
                        "Reset"
                    </button>
                </div>

                <div class="stats">
                    <div>{format!("route length: {:.2} km", total_route_m / 1000.0)}</div>
                    <div>{move || format!("streaming: {}", is_streaming.get())}</div>
                    <div>{move || format!("loading: {}", loading.get())}</div>
                    <div>{move || format!("packets sent: {}", packet_count.get())}</div>
                    <div>{move || format!("progress: {:.1}%", progress_ratio.get() * 100.0)}</div>
                    <div>{move || format!("speed: {:.1} km/h", last_speed_mps.get() * 3.6)}</div>
                    <div>{move || format!("eta: {}", format_eta(last_eta_sec.get()))}</div>
                    <div>{move || {
                        let (lon, lat) = last_position.get();
                        format!("last position: {:.5}, {:.5}", lon, lat)
                    }}</div>
                </div>

                {move || {
                    let err = last_error.get();
                    (!err.is_empty()).then(|| view! {
                        <div class="error">{format!("error: {}", err)}</div>
                    })
                }}
            </div>

            <ViewerContainer
                ion_token=ion_token
                animation=false
                timeline=false
                geocoder=false
                style="width: 100%; height: 100%;".to_string()
            >
                <CzmlDataSource
                    data=static_packet
                    clear_existing=false
                    on_loaded=on_static_loaded
                />

                <CzmlDataSource
                    data=packet
                    mode=dynamic_mode
                    clear_existing=false
                    trigger=packet_trigger
                    on_loaded=on_dynamic_loaded
                    on_loading=on_loading
                    on_error=on_error
                />

                <ViewerZoomToTarget
                    trigger=focus_trigger
                    target=loaded_target
                />

                <CameraFlyHome trigger=home_trigger duration=0.0 />
            </ViewerContainer>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> });
}
