//! HTML overlays that stay aligned with globe positions.

use glam::DVec3;
use leptos::{
    html::{Div, Video},
    prelude::*,
};
#[cfg(feature = "rerun")]
use leptos_rerun::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{Cartesian3, Entity, Event, SceneTransforms, Viewer};
#[cfg(target_arch = "wasm32")]
use crate::components::{use_cesium_context, use_cesium_overlay_context};
#[cfg(target_arch = "wasm32")]
use crate::core::{JsStoredValue, OwnedSlot, ThreadSafeJsValue};
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

    render_overlay_portal(overlay_ref, children)
}

/// Native HTML image overlay pinned to a Cesium world position.
#[component]
pub fn ImageOverlay(
    #[prop(into)] src: Signal<String>,
    #[prop(into)] position: Signal<DVec3>,
    #[prop(optional, into, default = 320.into())] width_px: Signal<u32>,
    #[prop(optional, into, default = 180.into())] height_px: Signal<u32>,
    #[prop(optional, into, default = true.into())] show: Signal<bool>,
    #[prop(optional, into)] alt: Signal<Option<String>>,
    #[prop(optional, into)] cross_origin: Signal<Option<String>>,
) -> impl IntoView {
    view! {
        <GeoAnchoredHtmlOverlay position=position show=show>
            <ImageOverlayBody
                src=src
                width_px=width_px
                height_px=height_px
                alt=alt
                cross_origin=cross_origin
            />
        </GeoAnchoredHtmlOverlay>
    }
}

/// Native HTML video overlay pinned to a Cesium world position.
#[component]
pub fn VideoOverlay(
    #[prop(into)] src: Signal<String>,
    #[prop(into)] position: Signal<DVec3>,
    #[prop(optional, into, default = 480.into())] width_px: Signal<u32>,
    #[prop(optional, into, default = 270.into())] height_px: Signal<u32>,
    #[prop(optional, into, default = true.into())] show: Signal<bool>,
    #[prop(optional, into, default = false.into())] autoplay: Signal<bool>,
    #[prop(optional, into, default = false.into())] loop_video: Signal<bool>,
    #[prop(optional, into, default = false.into())] muted: Signal<bool>,
    #[prop(optional, into, default = true.into())] plays_inline: Signal<bool>,
    #[prop(optional, into, default = false.into())] controls: Signal<bool>,
    #[prop(optional, into)] cross_origin: Signal<Option<String>>,
    #[prop(optional, into)] poster: Signal<Option<String>>,
    #[prop(optional, into)] preload: Signal<Option<String>>,
) -> impl IntoView {
    view! {
        <GeoAnchoredHtmlOverlay position=position show=show pointer_events=true>
            <VideoOverlayBody
                src=src
                width_px=width_px
                height_px=height_px
                autoplay=autoplay
                loop_video=loop_video
                muted=muted
                plays_inline=plays_inline
                controls=controls
                cross_origin=cross_origin
                poster=poster
                preload=preload
            />
        </GeoAnchoredHtmlOverlay>
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
            <YouTubeOverlayBody
                video_id=video_id
                width_px=width_px
                height_px=height_px
                autoplay=autoplay
                mute=mute
                controls=controls
                start_seconds=start_seconds
            />
        </GeoAnchoredHtmlOverlay>
    }
}

/// Rerun viewer overlay pinned to a Cesium world position.
#[cfg(feature = "rerun")]
#[component]
pub fn RerunOverlay(
    /// Source URL for a `.rrd` recording or supported Rerun HTTP-backed source.
    #[prop(into)]
    src: Signal<String>,
    /// World anchor as (longitude, latitude, height).
    #[prop(into)]
    position: Signal<DVec3>,
    /// Embedded viewer width in CSS pixels.
    #[prop(optional, into, default = 480.into())]
    width_px: Signal<u32>,
    /// Embedded viewer height in CSS pixels.
    #[prop(optional, into, default = 270.into())]
    height_px: Signal<u32>,
    /// Show or hide the overlay.
    #[prop(optional, into, default = true.into())]
    show: Signal<bool>,
) -> impl IntoView {
    view! {
        <GeoAnchoredHtmlOverlay position=position show=show pointer_events=true>
            <RerunOverlayBody
                src=src
                width_px=width_px
                height_px=height_px
            />
        </GeoAnchoredHtmlOverlay>
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
pub(crate) fn TrackedEntityImageOverlay(
    entity: ThreadSafeJsValue<Entity>,
    #[prop(into)] show: Signal<bool>,
    src: String,
    width_px: u32,
    height_px: u32,
    cross_origin: Option<String>,
) -> impl IntoView {
    let src = RwSignal::new(src);
    let alt = RwSignal::new(None::<String>);
    let cross_origin = RwSignal::new(cross_origin);

    view! {
        <TrackedEntityHtmlOverlay entity=entity show=show>
            <ImageOverlayBody
                src=src
                width_px=width_px
                height_px=height_px
                alt=alt
                cross_origin=cross_origin
            />
        </TrackedEntityHtmlOverlay>
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
pub(crate) fn TrackedEntityVideoOverlay(
    entity: ThreadSafeJsValue<Entity>,
    #[prop(into)] show: Signal<bool>,
    src: String,
    width_px: u32,
    height_px: u32,
    autoplay: bool,
    loop_video: bool,
    muted: bool,
    plays_inline: bool,
    controls: bool,
    cross_origin: Option<String>,
    poster: Option<String>,
    preload: Option<String>,
) -> impl IntoView {
    let src = RwSignal::new(src);
    let cross_origin = RwSignal::new(cross_origin);
    let poster = RwSignal::new(poster);
    let preload = RwSignal::new(preload);

    view! {
        <TrackedEntityHtmlOverlay entity=entity show=show pointer_events=true>
            <VideoOverlayBody
                src=src
                width_px=width_px
                height_px=height_px
                autoplay=autoplay
                loop_video=loop_video
                muted=muted
                plays_inline=plays_inline
                controls=controls
                cross_origin=cross_origin
                poster=poster
                preload=preload
            />
        </TrackedEntityHtmlOverlay>
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
pub(crate) fn TrackedEntityYouTubeOverlay(
    entity: ThreadSafeJsValue<Entity>,
    #[prop(into)] show: Signal<bool>,
    video_id: String,
    width_px: u32,
    height_px: u32,
    autoplay: bool,
    mute: bool,
    controls: bool,
    start_seconds: Option<u32>,
) -> impl IntoView {
    let video_id = RwSignal::new(video_id);

    view! {
        <TrackedEntityHtmlOverlay entity=entity show=show pointer_events=true>
            <YouTubeOverlayBody
                video_id=video_id
                width_px=width_px
                height_px=height_px
                autoplay=autoplay
                mute=mute
                controls=controls
                start_seconds=start_seconds
            />
        </TrackedEntityHtmlOverlay>
    }
}

#[cfg(all(target_arch = "wasm32", feature = "rerun"))]
#[component]
pub(crate) fn TrackedEntityRerunOverlay(
    entity: ThreadSafeJsValue<Entity>,
    #[prop(into)] show: Signal<bool>,
    src: String,
    width_px: u32,
    height_px: u32,
) -> impl IntoView {
    let src = RwSignal::new(src);

    view! {
        <TrackedEntityHtmlOverlay entity=entity show=show pointer_events=true>
            <RerunOverlayBody
                src=src
                width_px=width_px
                height_px=height_px
            />
        </TrackedEntityHtmlOverlay>
    }
}

#[component]
fn ImageOverlayBody(
    #[prop(into)] src: Signal<String>,
    #[prop(optional, into, default = 320.into())] width_px: Signal<u32>,
    #[prop(optional, into, default = 180.into())] height_px: Signal<u32>,
    #[prop(optional, into)] alt: Signal<Option<String>>,
    #[prop(optional, into)] cross_origin: Signal<Option<String>>,
) -> impl IntoView {
    view! {
        <img
            width=move || width_px.get().to_string()
            height=move || height_px.get().to_string()
            alt=move || alt.get().unwrap_or_default()
            crossorigin=move || cross_origin.get()
            src=move || src.get()
            style="display: block; border: 0; border-radius: 14px; background: rgba(8, 17, 29, 0.92); box-shadow: 0 18px 48px rgba(0, 0, 0, 0.38);"
        />
    }
}

#[component]
fn VideoOverlayBody(
    #[prop(into)] src: Signal<String>,
    #[prop(optional, into, default = 480.into())] width_px: Signal<u32>,
    #[prop(optional, into, default = 270.into())] height_px: Signal<u32>,
    #[prop(optional, into, default = false.into())] autoplay: Signal<bool>,
    #[prop(optional, into, default = false.into())] loop_video: Signal<bool>,
    #[prop(optional, into, default = false.into())] muted: Signal<bool>,
    #[prop(optional, into, default = true.into())] plays_inline: Signal<bool>,
    #[prop(optional, into, default = false.into())] controls: Signal<bool>,
    #[prop(optional, into)] cross_origin: Signal<Option<String>>,
    #[prop(optional, into)] poster: Signal<Option<String>>,
    #[prop(optional, into)] preload: Signal<Option<String>>,
) -> impl IntoView {
    let hover_controls = RwSignal::new(false);
    let video_ref = NodeRef::<Video>::new();

    #[cfg(not(feature = "ssr"))]
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let Some(video) = video_ref.get() else {
                return;
            };

            let inline_enabled = plays_inline.get();
            if inline_enabled {
                let _ = video.set_attribute("playsinline", "");
            } else {
                let _ = video.remove_attribute("playsinline");
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = plays_inline;
        }
    });

    #[cfg(feature = "ssr")]
    {
        let _ = plays_inline;
    }

    view! {
        <video
            node_ref=video_ref
            width=move || width_px.get().to_string()
            height=move || height_px.get().to_string()
            autoplay=move || autoplay.get()
            controls=move || controls.get() || hover_controls.get()
            muted=move || muted.get()
            playsinline=move || plays_inline.get()
            r#loop=move || loop_video.get()
            crossorigin=move || cross_origin.get()
            poster=move || poster.get()
            preload=move || preload.get()
            src=move || src.get()
            on:mouseenter=move |_| hover_controls.set(true)
            on:mouseleave=move |_| hover_controls.set(false)
            style="border: 0; border-radius: 14px; background: #000; box-shadow: 0 18px 48px rgba(0, 0, 0, 0.38);"
        ></video>
    }
}

#[component]
fn YouTubeOverlayBody(
    #[prop(into)] video_id: Signal<String>,
    #[prop(optional, into, default = 480.into())] width_px: Signal<u32>,
    #[prop(optional, into, default = 270.into())] height_px: Signal<u32>,
    #[prop(optional, into, default = false.into())] autoplay: Signal<bool>,
    #[prop(optional, into, default = false.into())] mute: Signal<bool>,
    #[prop(optional, into, default = true.into())] controls: Signal<bool>,
    #[prop(optional, into, default = None.into())] start_seconds: Signal<Option<u32>>,
) -> impl IntoView {
    view! {
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
    }
}

#[cfg(feature = "rerun")]
#[component]
fn RerunOverlayBody(
    #[prop(into)] src: Signal<String>,
    #[prop(optional, into, default = 480.into())] width_px: Signal<u32>,
    #[prop(optional, into, default = 270.into())] height_px: Signal<u32>,
) -> impl IntoView {
    let follow_if_http = Signal::derive(move || rerun_follow_if_http(&src.get()));

    view! {
        <div
            style=move || format!(
                "width:{}px;height:{}px;overflow:hidden;border:1px solid rgba(160, 198, 214, 0.14);border-radius:14px;background:rgba(8, 17, 29, 0.94);box-shadow:0 18px 48px rgba(0, 0, 0, 0.38);",
                width_px.get(),
                height_px.get()
            )
        >
            <RerunViewer
                class="leptos-cesium-rerun-overlay".to_string()
                style="width:100%;height:100%;min-height:0;".to_string()
                rrd=Signal::derive(move || {
                    let value = src.get();
                    if value.trim().is_empty() {
                        Vec::<String>::new()
                    } else {
                        vec![value]
                    }
                })
                panel_state_overrides=rerun_overlay_panel_overrides()
                hide_welcome_screen=true
                theme=Theme::Dark
                render_backend=RenderBackend::Webgl
                allow_fullscreen=false
                follow_if_http=follow_if_http
            />
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
fn TrackedEntityHtmlOverlay(
    entity: ThreadSafeJsValue<Entity>,
    #[prop(optional, into, default = true.into())] show: Signal<bool>,
    #[prop(optional, into, default = (0.0, 0.0).into())] offset_px: Signal<(f64, f64)>,
    #[prop(optional, into, default = true.into())] hide_when_offscreen: Signal<bool>,
    #[prop(optional, into, default = true.into())] hide_when_behind_globe: Signal<bool>,
    #[prop(optional, into, default = false.into())] pointer_events: Signal<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let overlay_ref = NodeRef::<Div>::new();
    let viewer_context =
        use_cesium_context().expect("TrackedEntityHtmlOverlay must be inside ViewerContainer");
    let overlay_context = use_cesium_overlay_context()
        .expect("TrackedEntityHtmlOverlay must be inside ViewerContainer");
    let post_render_listener = JsStoredValue::new_local(OwnedSlot::<(
        Event,
        wasm_bindgen::closure::Closure<dyn FnMut(JsValue, JsValue)>,
    )>::default());
    let entity_for_setup = entity.clone();
    let entity_for_updates = entity.clone();

    Effect::new(move |_| {
        let _ = viewer_context.viewer();
        let Some(host) = overlay_context.host().get() else {
            return;
        };
        let Some(overlay) = overlay_ref.get() else {
            return;
        };

        viewer_context.with_viewer(|viewer: Viewer| {
            if post_render_listener.with_value(|slot| slot.is_set()) {
                update_tracked_entity_overlay(
                    &viewer,
                    &host,
                    &overlay,
                    &entity_for_setup,
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
            let entity = entity_for_setup.clone();
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

                    update_tracked_entity_overlay(
                        &viewer,
                        &host,
                        &overlay,
                        &entity,
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
                    existing_event.remove_event_listener(existing_closure.as_ref().unchecked_ref());
                });
            });
        });
    });

    Effect::new(move |_| {
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
            update_tracked_entity_overlay(
                &viewer,
                &host,
                &overlay,
                &entity_for_updates,
                visible,
                offset,
                clip_offscreen,
                clip_behind_globe,
                allow_pointer_events,
            );
            viewer.scene().request_render();
        });
    });

    on_cleanup(move || {
        post_render_listener.update_value(|slot| {
            slot.clear_with(|(event, closure)| {
                event.remove_event_listener(closure.as_ref().unchecked_ref());
            });
        });
    });

    render_overlay_portal(overlay_ref, children)
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

#[cfg(feature = "rerun")]
fn rerun_overlay_panel_overrides() -> [(Panel, PanelState); 4] {
    [
        (Panel::Top, PanelState::Hidden),
        (Panel::Blueprint, PanelState::Hidden),
        (Panel::Selection, PanelState::Hidden),
        (Panel::Time, PanelState::Collapsed),
    ]
}

#[cfg(feature = "rerun")]
fn rerun_follow_if_http(url: &str) -> bool {
    let value = url.trim().to_ascii_lowercase();
    let value = value.split('?').next().unwrap_or(value.as_str());
    value.ends_with(".mcap")
}

#[cfg(target_arch = "wasm32")]
fn render_overlay_portal(overlay_ref: NodeRef<Div>, children: ChildrenFn) -> impl IntoView {
    let overlay_context = use_cesium_overlay_context()
        .expect("Overlay portal renderer must be inside ViewerContainer");
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
fn render_overlay_portal(overlay_ref: NodeRef<Div>, children: ChildrenFn) -> impl IntoView {
    let _ = (overlay_ref, children);
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
    let world_position = Cartesian3::from_degrees(position.x, position.y, position.z);
    update_overlay_world_position(
        viewer,
        host,
        overlay,
        Some(world_position),
        show,
        offset_px,
        hide_when_offscreen,
        hide_when_behind_globe,
        pointer_events,
    );
}

#[cfg(target_arch = "wasm32")]
fn update_tracked_entity_overlay(
    viewer: &Viewer,
    host: &web_sys::HtmlDivElement,
    overlay: &web_sys::HtmlDivElement,
    entity: &ThreadSafeJsValue<Entity>,
    show: bool,
    offset_px: (f64, f64),
    hide_when_offscreen: bool,
    hide_when_behind_globe: bool,
    pointer_events: bool,
) {
    let entity_visible = show && entity.value().show();
    let world_position = sample_entity_world_position(entity.value(), viewer);
    update_overlay_world_position(
        viewer,
        host,
        overlay,
        world_position,
        entity_visible,
        offset_px,
        hide_when_offscreen,
        hide_when_behind_globe,
        pointer_events,
    );
}

#[cfg(target_arch = "wasm32")]
fn sample_entity_world_position(entity: &Entity, viewer: &Viewer) -> Option<Cartesian3> {
    let time = viewer.clock().current_time();
    entity
        .position()
        .and_then(|property| property.get_value(Some(&time)))
}

#[cfg(target_arch = "wasm32")]
fn update_overlay_world_position(
    viewer: &Viewer,
    host: &web_sys::HtmlDivElement,
    overlay: &web_sys::HtmlDivElement,
    world_position: Option<Cartesian3>,
    show: bool,
    offset_px: (f64, f64),
    hide_when_offscreen: bool,
    hide_when_behind_globe: bool,
    pointer_events: bool,
) {
    if !show {
        apply_overlay_style(overlay, 0.0, 0.0, false, pointer_events, 0);
        return;
    }

    let Some(world_position) = world_position else {
        apply_overlay_style(overlay, 0.0, 0.0, false, pointer_events, 0);
        return;
    };

    let camera = viewer.camera();
    let camera_position = camera.position_wc();

    if hide_when_behind_globe && !is_anchor_visible_from_camera(&camera_position, &world_position) {
        apply_overlay_style(overlay, 0.0, 0.0, false, pointer_events, 0);
        return;
    }

    let view_depth = camera_view_depth(&camera_position, &camera.direction_wc(), &world_position);
    if !view_depth.is_finite() || view_depth <= 0.0 {
        apply_overlay_style(overlay, 0.0, 0.0, false, pointer_events, 0);
        return;
    }

    let Some(window_position) =
        SceneTransforms::world_to_window_coordinates(&viewer.scene(), &world_position)
    else {
        apply_overlay_style(overlay, 0.0, 0.0, false, pointer_events, 0);
        return;
    };

    let left = window_position.x() + offset_px.0;
    let top = window_position.y() + offset_px.1;
    let within_bounds = left >= 0.0
        && top >= 0.0
        && left <= f64::from(host.client_width())
        && top <= f64::from(host.client_height());

    let visible = !hide_when_offscreen || within_bounds;
    let z_index = if visible {
        overlay_z_index_from_view_depth(view_depth)
    } else {
        0
    };
    apply_overlay_style(overlay, left, top, visible, pointer_events, z_index);
}

#[cfg(target_arch = "wasm32")]
fn apply_overlay_style(
    overlay: &web_sys::HtmlDivElement,
    left: f64,
    top: f64,
    visible: bool,
    pointer_events: bool,
    z_index: i32,
) {
    let visibility = if visible { "visible" } else { "hidden" };
    let pointer_events = if visible && pointer_events {
        "auto"
    } else {
        "none"
    };
    let style = format!(
        "position: absolute; left: {left:.2}px; top: {top:.2}px; transform: translate(-50%, -50%); visibility: {visibility}; pointer-events: {pointer_events}; z-index: {z_index}; will-change: transform;"
    );
    let _ = overlay.set_attribute("style", &style);
}

#[cfg(target_arch = "wasm32")]
fn camera_view_depth(
    camera_position: &crate::bindings::Cartesian3,
    camera_direction: &crate::bindings::Cartesian3,
    world_position: &Cartesian3,
) -> f64 {
    let offset_x = world_position.x() - camera_position.x();
    let offset_y = world_position.y() - camera_position.y();
    let offset_z = world_position.z() - camera_position.z();

    (offset_x * camera_direction.x())
        + (offset_y * camera_direction.y())
        + (offset_z * camera_direction.z())
}

#[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
fn overlay_z_index_from_view_depth(view_depth: f64) -> i32 {
    if !view_depth.is_finite() || view_depth <= 0.0 {
        return 0;
    }

    let clamped_depth = view_depth.floor().clamp(0.0, f64::from(i32::MAX - 1));
    i32::MAX - clamped_depth as i32
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
    use super::{build_youtube_embed_url, overlay_z_index_from_view_depth};

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

    #[test]
    fn nearer_view_depth_gets_higher_overlay_z_index() {
        assert!(overlay_z_index_from_view_depth(10.0) > overlay_z_index_from_view_depth(100.0));
    }

    #[test]
    fn non_positive_view_depth_hides_overlay_z_index() {
        assert_eq!(overlay_z_index_from_view_depth(0.0), 0);
        assert_eq!(overlay_z_index_from_view_depth(-15.0), 0);
    }
}
