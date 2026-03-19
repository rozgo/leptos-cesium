//! HTML overlays that stay aligned with globe positions.

use glam::DVec3;
use leptos::{html::Div, prelude::*};

#[cfg(target_arch = "wasm32")]
use crate::bindings::{Cartesian3, Event, SceneTransforms, Viewer};
#[cfg(target_arch = "wasm32")]
use crate::components::{use_cesium_context, use_cesium_overlay_context};
#[cfg(target_arch = "wasm32")]
use crate::core::{JsStoredValue, OwnedSlot};
#[cfg(target_arch = "wasm32")]
use leptos::portal::Portal;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue};

/// Screen-space HTML overlay pinned to a Cesium world position.
#[component]
pub fn GeoAnchoredHtmlOverlay(
    /// World anchor as (longitude, latitude, height).
    #[prop(into)]
    position: Signal<DVec3>,
    /// Show or hide the overlay.
    #[prop(optional, into, default = true.into())]
    show: Signal<bool>,
    /// Screen-space pixel offset from the projected anchor.
    #[prop(optional, into, default = (0.0, 0.0).into())]
    offset_px: Signal<(f64, f64)>,
    /// Hide when the projected center moves outside the viewer bounds.
    #[prop(optional, into, default = true.into())]
    hide_when_offscreen: Signal<bool>,
    /// Hide when the anchor falls behind the globe horizon.
    #[prop(optional, into, default = true.into())]
    hide_when_behind_globe: Signal<bool>,
    /// Allow pointer interaction on the overlay wrapper.
    #[prop(optional, into, default = false.into())]
    pointer_events: Signal<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let overlay_ref = NodeRef::<Div>::new();

    #[cfg(target_arch = "wasm32")]
    let viewer_context =
        use_cesium_context().expect("GeoAnchoredHtmlOverlay must be inside ViewerContainer");
    #[cfg(target_arch = "wasm32")]
    let overlay_context = use_cesium_overlay_context()
        .expect("GeoAnchoredHtmlOverlay must be inside ViewerContainer");
    #[cfg(target_arch = "wasm32")]
    let post_render_listener = JsStoredValue::new_local(OwnedSlot::<(
        Event,
        wasm_bindgen::closure::Closure<dyn FnMut(JsValue, JsValue)>,
    )>::default());

    #[cfg(not(feature = "ssr"))]
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = viewer_context.viewer();
            let Some(host) = overlay_context.host().get() else {
                return;
            };
            let Some(overlay) = overlay_ref.get() else {
                return;
            };

            viewer_context.with_viewer(|viewer: Viewer| {
                if post_render_listener.with_value(|slot| slot.is_set()) {
                    update_overlay_position(
                        &viewer,
                        &host,
                        &overlay,
                        position.get_untracked(),
                        show.get_untracked(),
                        offset_px.get_untracked(),
                        hide_when_offscreen.get_untracked(),
                        hide_when_behind_globe.get_untracked(),
                        pointer_events.get_untracked(),
                    );
                    return;
                }

                let scene = viewer.scene();
                let event = scene.post_render();
                let host_ref = overlay_context.host();
                let overlay_ref = overlay_ref;
                let position = position;
                let show = show;
                let offset_px = offset_px;
                let hide_when_offscreen = hide_when_offscreen;
                let hide_when_behind_globe = hide_when_behind_globe;
                let pointer_events = pointer_events;
                let viewer_context = viewer_context;

                let closure = wasm_bindgen::closure::Closure::wrap(Box::new(
                    move |_scene: JsValue, _time: JsValue| {
                        let Some(viewer) = viewer_context.viewer_untracked() else {
                            return;
                        };
                        let Some(host) = host_ref.get_untracked() else {
                            return;
                        };
                        let Some(overlay) = overlay_ref.get_untracked() else {
                            return;
                        };

                        update_overlay_position(
                            &viewer,
                            &host,
                            &overlay,
                            position.get_untracked(),
                            show.get_untracked(),
                            offset_px.get_untracked(),
                            hide_when_offscreen.get_untracked(),
                            hide_when_behind_globe.get_untracked(),
                            pointer_events.get_untracked(),
                        );
                    },
                )
                    as Box<dyn FnMut(JsValue, JsValue)>);

                event.add_event_listener(closure.as_ref().unchecked_ref());

                post_render_listener.update_value(|slot| {
                    slot.replace_with((event, closure), |(existing_event, existing_closure)| {
                        existing_event
                            .remove_event_listener(existing_closure.as_ref().unchecked_ref());
                    });
                });
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (
                position,
                show,
                offset_px,
                hide_when_offscreen,
                hide_when_behind_globe,
                pointer_events,
            );
        }
    });

    #[cfg(not(feature = "ssr"))]
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let anchor = position.get();
            let visible = show.get();
            let offset = offset_px.get();
            let clip_offscreen = hide_when_offscreen.get();
            let clip_behind_globe = hide_when_behind_globe.get();
            let allow_pointer_events = pointer_events.get();
            let Some(host) = overlay_context.host().get() else {
                return;
            };
            let Some(overlay) = overlay_ref.get() else {
                return;
            };

            viewer_context.with_viewer(|viewer: Viewer| {
                update_overlay_position(
                    &viewer,
                    &host,
                    &overlay,
                    anchor,
                    visible,
                    offset,
                    clip_offscreen,
                    clip_behind_globe,
                    allow_pointer_events,
                );
                viewer.scene().request_render();
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (
                position,
                show,
                offset_px,
                hide_when_offscreen,
                hide_when_behind_globe,
                pointer_events,
            );
        }
    });

    on_cleanup(move || {
        #[cfg(target_arch = "wasm32")]
        {
            post_render_listener.update_value(|slot| {
                slot.clear_with(|(event, closure)| {
                    event.remove_event_listener(closure.as_ref().unchecked_ref());
                });
            });
        }
    });

    #[cfg(feature = "ssr")]
    {
        let _ = (
            position,
            show,
            offset_px,
            hide_when_offscreen,
            hide_when_behind_globe,
            pointer_events,
        );
    }

    #[cfg(target_arch = "wasm32")]
    {
        let children = children.clone();

        view! {
            {move || {
                let children = children.clone();
                overlay_context.host().get().map(|mount| {
                    let children = children.clone();
                    let mount: web_sys::Element = mount.into();
                    view! {
                        <Portal mount=mount>
                            <div node_ref=overlay_ref>
                                {children()}
                            </div>
                        </Portal>
                    }
                })
            }}
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (overlay_ref, children);
    }
}

/// YouTube iframe overlay pinned to a Cesium world position.
#[component]
pub fn YouTubeOverlay(
    /// YouTube video id used for the embed source.
    #[prop(into)]
    video_id: Signal<String>,
    /// World anchor as (longitude, latitude, height).
    #[prop(into)]
    position: Signal<DVec3>,
    /// Embedded player width in CSS pixels.
    #[prop(optional, into, default = 480.into())]
    width_px: Signal<u32>,
    /// Embedded player height in CSS pixels.
    #[prop(optional, into, default = 270.into())]
    height_px: Signal<u32>,
    /// Show or hide the overlay.
    #[prop(optional, into, default = true.into())]
    show: Signal<bool>,
    /// Enable autoplay in the player URL.
    #[prop(optional, into, default = false.into())]
    autoplay: Signal<bool>,
    /// Start muted in the player URL.
    #[prop(optional, into, default = false.into())]
    mute: Signal<bool>,
    /// Show YouTube player controls.
    #[prop(optional, into, default = true.into())]
    controls: Signal<bool>,
    /// Optional start offset in seconds.
    #[prop(optional, into, default = None.into())]
    start_seconds: Signal<Option<u32>>,
) -> impl IntoView {
    view! {
        <GeoAnchoredHtmlOverlay position=position show=show pointer_events=true>
            <iframe
                width=move || width_px.get().to_string()
                height=move || height_px.get().to_string()
                title=move || format!("YouTube video {}", video_id.get())
                src=move || {
                    build_youtube_embed_url(
                        &video_id.get(),
                        autoplay.get(),
                        mute.get(),
                        controls.get(),
                        start_seconds.get(),
                    )
                }
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                referrerpolicy="strict-origin-when-cross-origin"
                allowfullscreen=true
                style="border: 0; border-radius: 14px; background: #000; box-shadow: 0 18px 48px rgba(0, 0, 0, 0.38);"
            ></iframe>
        </GeoAnchoredHtmlOverlay>
    }
}

fn build_youtube_embed_url(
    video_id: &str,
    autoplay: bool,
    mute: bool,
    controls: bool,
    start_seconds: Option<u32>,
) -> String {
    let mut serializer = url::form_urlencoded::Serializer::new(String::new());
    serializer.append_pair("playsinline", "1");

    if autoplay {
        serializer.append_pair("autoplay", "1");
    }

    if mute {
        serializer.append_pair("mute", "1");
    }

    if !controls {
        serializer.append_pair("controls", "0");
    }

    if let Some(start_seconds) = start_seconds {
        serializer.append_pair("start", &start_seconds.to_string());
    }

    format!(
        "https://www.youtube.com/embed/{}?{}",
        video_id,
        serializer.finish()
    )
}

#[cfg(target_arch = "wasm32")]
fn update_overlay_position(
    viewer: &Viewer,
    host: &web_sys::HtmlDivElement,
    overlay: &web_sys::HtmlDivElement,
    position: DVec3,
    show: bool,
    offset_px: (f64, f64),
    hide_when_offscreen: bool,
    hide_when_behind_globe: bool,
    pointer_events: bool,
) {
    if !show {
        apply_overlay_style(overlay, 0.0, 0.0, false, pointer_events);
        return;
    }

    let world_position = Cartesian3::from_degrees(position.x, position.y, position.z);

    if hide_when_behind_globe
        && !is_anchor_visible_from_camera(&viewer.camera().position_wc(), &world_position)
    {
        apply_overlay_style(overlay, 0.0, 0.0, false, pointer_events);
        return;
    }

    let Some(window_position) =
        SceneTransforms::world_to_window_coordinates(&viewer.scene(), &world_position)
    else {
        apply_overlay_style(overlay, 0.0, 0.0, false, pointer_events);
        return;
    };

    let left = window_position.x() + offset_px.0;
    let top = window_position.y() + offset_px.1;
    let within_bounds = left >= 0.0
        && top >= 0.0
        && left <= f64::from(host.client_width())
        && top <= f64::from(host.client_height());

    let visible = !hide_when_offscreen || within_bounds;
    apply_overlay_style(overlay, left, top, visible, pointer_events);
}

#[cfg(target_arch = "wasm32")]
fn apply_overlay_style(
    overlay: &web_sys::HtmlDivElement,
    left: f64,
    top: f64,
    visible: bool,
    pointer_events: bool,
) {
    let visibility = if visible { "visible" } else { "hidden" };
    let pointer_events = if visible && pointer_events {
        "auto"
    } else {
        "none"
    };
    let style = format!(
        "position: absolute; left: {left:.2}px; top: {top:.2}px; transform: translate(-50%, -50%); visibility: {visibility}; pointer-events: {pointer_events}; will-change: transform;"
    );
    let _ = overlay.set_attribute("style", &style);
}

#[cfg(target_arch = "wasm32")]
fn is_anchor_visible_from_camera(
    camera_position: &crate::bindings::Cartesian3,
    world_position: &Cartesian3,
) -> bool {
    let px = world_position.x();
    let py = world_position.y();
    let pz = world_position.z();

    let view_x = camera_position.x() - px;
    let view_y = camera_position.y() - py;
    let view_z = camera_position.z() - pz;

    (view_x * px) + (view_y * py) + (view_z * pz) > 0.0
}

#[cfg(test)]
mod tests {
    use super::build_youtube_embed_url;

    #[test]
    fn youtube_embed_url_includes_default_inline_playback() {
        let url = build_youtube_embed_url("M7lc1UVf-VE", false, false, true, None);

        assert_eq!(
            url,
            "https://www.youtube.com/embed/M7lc1UVf-VE?playsinline=1"
        );
    }

    #[test]
    fn youtube_embed_url_serializes_optional_flags() {
        let url = build_youtube_embed_url("M7lc1UVf-VE", true, true, false, Some(42));

        assert_eq!(
            url,
            "https://www.youtube.com/embed/M7lc1UVf-VE?playsinline=1&autoplay=1&mute=1&controls=0&start=42"
        );
    }
}
