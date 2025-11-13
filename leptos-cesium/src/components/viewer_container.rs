//! Viewer container component that owns the Cesium viewer instance.

use leptos::{html::Div, prelude::*};

use crate::components::provide_cesium_context;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{Viewer, set_default_access_token};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(target_arch = "wasm32")]
use web_sys::{HtmlElement, console};

/// Minimal Cesium viewer container component.
///
/// This sets up the viewer context for descendants and creates a Cesium Viewer instance.
///
/// # Props
///
/// * `ion_token` - Optional Cesium Ion access token. If provided, sets the default access token
///   before creating the viewer. Use `env!("CESIUM_ION_TOKEN")` to load from environment.
/// * `class` - Optional CSS class for the container div
/// * `style` - Optional inline styles for the container div
/// * `node_ref` - Optional node reference to access the underlying DOM element
/// * `animation` - Whether to show animation widget. Defaults to true.
/// * `timeline` - Whether to show timeline widget. Defaults to true.
/// * `base_layer_picker` - Whether to show base layer picker. Defaults to true.
/// * `home_button` - Whether to show home button. Defaults to true.
/// * `scene_mode_picker` - Whether to show scene mode picker. Defaults to true.
/// * `navigation_help_button` - Whether to show navigation help button. Defaults to true.
/// * `fullscreen_button` - Whether to show fullscreen button. Defaults to true.
/// * `info_box` - Whether to show the default InfoBox widget when entities are selected. Defaults to true.
/// * `selection_indicator` - Whether to show the green selection indicator when entities are selected. Defaults to true.
/// * `should_animate` - Whether animations should play automatically. Defaults to true. Required for CZML animations.
/// * `children` - Child components (entities, data sources, etc.)
#[component]
pub fn ViewerContainer(
    #[prop(optional, into)] ion_token: Signal<Option<String>>,
    #[prop(optional)] class: String,
    #[prop(optional)] style: String,
    #[prop(optional, default = NodeRef::new())] node_ref: NodeRef<Div>,
    #[prop(optional, default = true)] animation: bool,
    #[prop(optional, default = true)] timeline: bool,
    #[prop(optional, default = true)] base_layer_picker: bool,
    #[prop(optional, default = true)] home_button: bool,
    #[prop(optional, default = true)] scene_mode_picker: bool,
    #[prop(optional, default = true)] navigation_help_button: bool,
    #[prop(optional, default = true)] fullscreen_button: bool,
    #[prop(optional, default = true)] info_box: bool,
    #[prop(optional, default = true)] selection_indicator: bool,
    #[prop(optional, default = true)] should_animate: bool,
    #[prop(optional, into, default = true.into())] globe: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let viewer_context = provide_cesium_context();

    // Create viewer once (doesn't re-run when signals change due to untracked access)
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            console::debug_1(&JsValue::from_str("ViewerContainer: effect tick"));
            if viewer_context.viewer_untracked().is_some() {
                console::debug_1(&JsValue::from_str(
                    "ViewerContainer: viewer already exists; skipping instantiation.",
                ));
                return;
            }

            let Some(div) = node_ref.get() else {
                console::debug_1(&JsValue::from_str(
                    "ViewerContainer: node_ref empty; waiting for next tick.",
                ));
                return;
            };

            let element: HtmlElement = div.into();

            // Set Ion token if provided (untracked so changes don't recreate viewer)
            if let Some(token) = ion_token.get_untracked() {
                console::debug_1(&JsValue::from_str(
                    "ViewerContainer: setting Cesium Ion access token.",
                ));
                set_default_access_token(&token);
            }

            console::debug_1(&JsValue::from_str(
                "ViewerContainer: constructing Cesium.Viewer instance.",
            ));

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

            let viewer = Viewer::new(&element, &options.into());
            console::debug_1(&JsValue::from_str(
                "ViewerContainer: viewer created; storing in context.",
            ));
            viewer_context.set_viewer(viewer);

            // Remove cesium-viewer-bottom
            if let Some(document) = web_sys::window().and_then(|w| w.document())
                && let Some(bottom_bar) = document
                    .query_selector(".cesium-viewer-bottom")
                    .ok()
                    .flatten()
            {
                bottom_bar.remove();
                console::debug_1(&JsValue::from_str(
                    "ViewerContainer: removed .cesium-viewer-bottom",
                ));
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (
                ion_token,
                animation,
                timeline,
                base_layer_picker,
                home_button,
                scene_mode_picker,
                navigation_help_button,
                fullscreen_button,
                info_box,
                selection_indicator,
                should_animate,
            );
        }
    });

    // Set up selection event listener
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::closure::Closure;

            viewer_context.with_viewer(|viewer: Viewer| {
                let event = viewer.selected_entity_changed();
                let ctx = viewer_context;

                // Create closure that updates the context when selection changes
                let closure = Closure::wrap(Box::new(move |entity: JsValue| {
                    ctx.set_selected_entity_from_js(entity);
                }) as Box<dyn FnMut(JsValue)>);

                // Add event listener
                event.add_event_listener(closure.as_ref().unchecked_ref());

                // Store closure so it's not dropped (it will be cleaned up when the component unmounts)
                closure.forget();

                console::debug_1(&JsValue::from_str(
                    "ViewerContainer: selectedEntityChanged event listener attached.",
                ));
            });
        }
    });

    // Separate effect to control globe visibility dynamically
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
                    console::log_1(&JsValue::from_str(&format!(
                        "ViewerContainer: globe visibility set to {}",
                        show_globe
                    )));
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
            if let Some(viewer) = viewer_context.viewer_untracked() {
                console::debug_1(&JsValue::from_str(
                    "ViewerContainer: destroying Cesium viewer on cleanup.",
                ));
                viewer.destroy();
            }
        }
        viewer_context.clear_viewer();
    });

    view! {
        <div node_ref=node_ref class=class style=style>
            {children.map(|c| c())}
        </div>
    }
}
