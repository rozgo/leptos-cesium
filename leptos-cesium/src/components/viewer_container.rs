//! Viewer container component that owns the Cesium viewer instance.

use leptos::{html::Div, prelude::*};

use crate::components::{provide_cesium_context, provide_cesium_overlay_context};
#[cfg(target_arch = "wasm32")]
use crate::core::{JsStoredValue, OwnedSlot};

/// CDN base URL for Cesium assets (Workers, Assets, etc.)
#[cfg(all(target_arch = "wasm32", not(feature = "ssr")))]
const CESIUM_CDN_BASE: &str = "https://cesium.com/downloads/cesiumjs/releases/1.140/Build/Cesium/";

#[cfg(target_arch = "wasm32")]
use crate::bindings::Event;
#[cfg(all(target_arch = "wasm32", not(feature = "ssr")))]
use crate::bindings::{Viewer, set_base_url, set_default_access_token};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(all(target_arch = "wasm32", not(feature = "ssr")))]
use web_sys::HtmlElement;

/// Minimal Cesium viewer container component.
///
/// This sets up the viewer context for descendants and creates a Cesium Viewer instance.
///
/// # Props
///
/// * `ion_token` - Optional Cesium Ion access token. If provided, sets the default access token
///   before creating the viewer.
/// * `class` - Optional CSS class for the container div
/// * `style` - Optional inline styles for the container div
/// * `node_ref` - Optional node reference to access the underlying DOM element
/// * `animation` - Whether to show animation widget. Defaults to true.
/// * `timeline` - Whether to show timeline widget. Defaults to true.
/// * `geocoder` - Whether to show geocoder/search widget. Defaults to true.
/// * `base_layer_picker` - Whether to show base layer picker. Defaults to true.
/// * `home_button` - Whether to show home button. Defaults to true.
/// * `scene_mode_picker` - Whether to show scene mode picker. Defaults to true.
/// * `navigation_help_button` - Whether to show navigation help button. Defaults to true.
/// * `fullscreen_button` - Whether to show fullscreen button. Defaults to true.
/// * `info_box` - Whether to show the default InfoBox widget when entities are selected. Defaults to true.
/// * `selection_indicator` - Whether to show the green selection indicator when entities are selected. Defaults to true.
/// * `should_animate` - Whether animations should play automatically. Defaults to true. Required for CZML animations.
/// * `automatically_track_data_source_clocks` - Whether viewer clock auto-tracks newly added data sources. Defaults to true.
/// * `allow_data_sources_to_suspend_animation` - Whether data sources may temporarily suspend animation while loading. Defaults to true.
/// * `children` - Child components (entities, data sources, etc.)
#[component]
pub fn ViewerContainer(
    #[prop(optional, into)] ion_token: Signal<Option<String>>,
    #[prop(optional)] class: String,
    #[prop(optional)] style: String,
    #[prop(optional, default = NodeRef::new())] node_ref: NodeRef<Div>,
    #[prop(optional, default = true)] animation: bool,
    #[prop(optional, default = true)] timeline: bool,
    #[prop(optional, default = true)] geocoder: bool,
    #[prop(optional, default = true)] base_layer_picker: bool,
    #[prop(optional, default = true)] home_button: bool,
    #[prop(optional, default = true)] scene_mode_picker: bool,
    #[prop(optional, default = true)] navigation_help_button: bool,
    #[prop(optional, default = true)] fullscreen_button: bool,
    #[prop(optional, default = true)] info_box: bool,
    #[prop(optional, default = true)] selection_indicator: bool,
    #[prop(optional, default = true)] should_animate: bool,
    #[prop(optional, default = true)] automatically_track_data_source_clocks: bool,
    #[prop(optional, default = true)] allow_data_sources_to_suspend_animation: bool,
    #[prop(optional, into, default = true.into())] globe: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let viewer_context = provide_cesium_context();
    let overlay_host_ref = NodeRef::<Div>::new();
    let _overlay_context = provide_cesium_overlay_context(overlay_host_ref);
    #[cfg(target_arch = "wasm32")]
    let selected_entity_listener = JsStoredValue::new_local(OwnedSlot::<(
        Event,
        wasm_bindgen::closure::Closure<dyn FnMut(JsValue)>,
    )>::default());

    // Create viewer once (doesn't re-run when signals change due to untracked access)
    #[cfg(not(feature = "ssr"))]
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            if viewer_context.viewer_untracked().is_some() {
                return;
            }

            let Some(div) = node_ref.get() else {
                return;
            };

            let element: HtmlElement = div.into();

            // Set base URL for Cesium assets (Workers, Assets) from CDN
            set_base_url(CESIUM_CDN_BASE);

            // Set Ion token if provided (untracked so changes don't recreate viewer)
            if let Some(token) = ion_token.get_untracked() {
                set_default_access_token(&token);
            }

            // Build viewer options (always start with globe visible)
            let options = js_sys::Object::new();
            let _ = js_sys::Reflect::set(
                &options,
                &JsValue::from_str("animation"),
                &JsValue::from_bool(animation),
            );
            let _ = js_sys::Reflect::set(
                &options,
                &JsValue::from_str("timeline"),
                &JsValue::from_bool(timeline),
            );
            let _ = js_sys::Reflect::set(
                &options,
                &JsValue::from_str("geocoder"),
                &JsValue::from_bool(geocoder),
            );
            let _ = js_sys::Reflect::set(
                &options,
                &JsValue::from_str("baseLayerPicker"),
                &JsValue::from_bool(base_layer_picker),
            );
            let _ = js_sys::Reflect::set(
                &options,
                &JsValue::from_str("homeButton"),
                &JsValue::from_bool(home_button),
            );
            let _ = js_sys::Reflect::set(
                &options,
                &JsValue::from_str("sceneModePicker"),
                &JsValue::from_bool(scene_mode_picker),
            );
            let _ = js_sys::Reflect::set(
                &options,
                &JsValue::from_str("navigationHelpButton"),
                &JsValue::from_bool(navigation_help_button),
            );
            let _ = js_sys::Reflect::set(
                &options,
                &JsValue::from_str("fullscreenButton"),
                &JsValue::from_bool(fullscreen_button),
            );
            let _ = js_sys::Reflect::set(
                &options,
                &JsValue::from_str("infoBox"),
                &JsValue::from_bool(info_box),
            );
            let _ = js_sys::Reflect::set(
                &options,
                &JsValue::from_str("selectionIndicator"),
                &JsValue::from_bool(selection_indicator),
            );
            let _ = js_sys::Reflect::set(
                &options,
                &JsValue::from_str("shouldAnimate"),
                &JsValue::from_bool(should_animate),
            );
            let _ = js_sys::Reflect::set(
                &options,
                &JsValue::from_str("automaticallyTrackDataSourceClocks"),
                &JsValue::from_bool(automatically_track_data_source_clocks),
            );

            let viewer = Viewer::new(&element, &options.into());
            viewer.set_allow_data_sources_to_suspend_animation(
                allow_data_sources_to_suspend_animation,
            );
            viewer_context.set_viewer(viewer);

            // Remove cesium-viewer-bottom
            if let Some(document) = web_sys::window().and_then(|w| w.document())
                && let Some(bottom_bar) = document
                    .query_selector(".cesium-viewer-bottom")
                    .ok()
                    .flatten()
            {
                bottom_bar.remove();
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (
                ion_token,
                animation,
                timeline,
                geocoder,
                base_layer_picker,
                home_button,
                scene_mode_picker,
                navigation_help_button,
                fullscreen_button,
                info_box,
                selection_indicator,
                should_animate,
                automatically_track_data_source_clocks,
                allow_data_sources_to_suspend_animation,
            );
        }
    });

    // Set up selection event listener
    #[cfg(not(feature = "ssr"))]
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            // React to viewer availability.
            let _ = viewer_context.viewer();

            viewer_context.with_viewer(|viewer: Viewer| {
                if selected_entity_listener.with_value(|listener| listener.is_set()) {
                    return;
                }

                let event = viewer.selected_entity_changed();
                let ctx = viewer_context;

                // Create closure that updates the context when selection changes
                let closure =
                    wasm_bindgen::closure::Closure::wrap(Box::new(move |entity: JsValue| {
                        ctx.set_selected_entity_from_js(entity);
                    })
                        as Box<dyn FnMut(JsValue)>);

                // Add event listener
                event.add_event_listener(closure.as_ref().unchecked_ref());

                // Store event + closure for cleanup.
                selected_entity_listener.update_value(|listener| {
                    listener.replace_with(
                        (event, closure),
                        |(existing_event, existing_closure)| {
                            existing_event
                                .remove_event_listener(existing_closure.as_ref().unchecked_ref());
                        },
                    );
                });
            });
        }
    });

    // Separate effect to control globe visibility dynamically
    #[cfg(not(feature = "ssr"))]
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let show_globe = globe.get();
            viewer_context.with_viewer(|viewer: Viewer| {
                let scene = viewer.scene();
                // Access scene.globe and set its show property
                if let Ok(globe_obj) = js_sys::Reflect::get(&scene, &JsValue::from_str("globe"))
                    && !globe_obj.is_undefined()
                    && !globe_obj.is_null()
                {
                    let _ = js_sys::Reflect::set(
                        &globe_obj,
                        &JsValue::from_str("show"),
                        &JsValue::from_bool(show_globe),
                    );
                }
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = globe;
        }
    });

    on_cleanup(move || {
        #[cfg(target_arch = "wasm32")]
        {
            selected_entity_listener.update_value(|listener| {
                listener.clear_with(|(event, closure)| {
                    event.remove_event_listener(closure.as_ref().unchecked_ref());
                });
            });

            if let Some(viewer) = viewer_context.viewer_untracked() {
                viewer.destroy();
            }
        }
        viewer_context.clear_viewer();
    });

    // Silence unused variable warnings in SSR mode (props only used in hydrate Effects)
    #[cfg(feature = "ssr")]
    {
        let _ = (
            ion_token,
            animation,
            timeline,
            geocoder,
            base_layer_picker,
            home_button,
            scene_mode_picker,
            navigation_help_button,
            fullscreen_button,
            info_box,
            selection_indicator,
            should_animate,
            automatically_track_data_source_clocks,
            allow_data_sources_to_suspend_animation,
            globe,
        );
    }

    let container_style = if style.trim().is_empty() {
        "position: relative;".to_string()
    } else {
        format!("{style}; position: relative;")
    };

    view! {
        <div node_ref=node_ref class=class style=container_style>
            {children.map(|c| c())}
            <div
                node_ref=overlay_host_ref
                class="leptos-cesium-overlay-host"
                style="position: absolute; inset: 0; overflow: hidden; pointer-events: none; z-index: 20;"
            ></div>
        </div>
    }
}
