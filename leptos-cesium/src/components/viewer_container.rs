//! Viewer container component that owns the Cesium viewer instance.

use leptos::{html::Div, prelude::*};

use crate::components::provide_cesium_context;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{set_default_access_token, Viewer};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use web_sys::{console, HtmlElement};

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
/// * `children` - Child components (entities, data sources, etc.)
#[component]
pub fn ViewerContainer(
    #[prop(optional, into)] ion_token: Signal<Option<String>>,
    #[prop(optional)] class: String,
    #[prop(optional)] style: String,
    #[prop(optional, default = NodeRef::new())] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    let viewer_context = provide_cesium_context();

    #[cfg(target_arch = "wasm32")]
    {
        Effect::new(move |_| {
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

            // Set Ion token if provided
            if let Some(token) = ion_token.get() {
                console::debug_1(&JsValue::from_str(
                    "ViewerContainer: setting Cesium Ion access token.",
                ));
                set_default_access_token(&token);
            }

            console::debug_1(&JsValue::from_str(
                "ViewerContainer: constructing Cesium.Viewer instance.",
            ));

            let viewer = Viewer::new(&element, &JsValue::UNDEFINED);
            console::debug_1(&JsValue::from_str(
                "ViewerContainer: viewer created; storing in context.",
            ));
            viewer_context.set_viewer(viewer);
        });
    }

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
            {children()}
        </div>
    }
}
