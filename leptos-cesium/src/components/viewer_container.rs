//! Viewer container component that owns the Cesium viewer instance.

use leptos::{html::Div, prelude::*};

use crate::components::provide_cesium_context;

#[cfg(target_arch = "wasm32")]
use crate::bindings::Viewer;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use web_sys::{console, HtmlElement};

/// Minimal Cesium viewer container component.
///
/// This sets up the viewer context for descendants. Actual Cesium viewer instantiation will be
/// wired in once bindings are available.
#[component]
pub fn ViewerContainer(
    #[prop(optional)] class: String,
    #[prop(optional)] style: String,
    #[prop(optional, default = NodeRef::new())] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    let viewer_context = provide_cesium_context();

    #[cfg(target_arch = "wasm32")]
    {
        let node_ref = node_ref.clone();
        let viewer_context = viewer_context;

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
