//! Viewer container component that owns the Cesium viewer instance.

use leptos::{html::Div, prelude::*};

use crate::components::provide_cesium_context;

#[cfg(target_arch = "wasm32")]
use crate::{
    bindings::{set_base_url, Viewer},
    core::JsRwSignal,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlElement;

/// Minimal Cesium viewer container component.
///
/// This sets up the viewer context for descendants. Actual Cesium viewer instantiation will be
/// wired in once bindings are available.
#[component]
pub fn ViewerContainer(
    #[prop(optional)] class: String,
    #[prop(optional)] style: String,
    #[prop(optional, default = NodeRef::new())] node_ref: NodeRef<Div>,
    #[prop(optional)] base_url: Option<String>,
    children: Children,
) -> impl IntoView {
    let viewer_context = provide_cesium_context();

    #[cfg(target_arch = "wasm32")]
    let viewer_handle: JsRwSignal<Option<Viewer>> = JsRwSignal::new_local(None);

    #[cfg(not(target_arch = "wasm32"))]
    let _ = &base_url;

    #[cfg(target_arch = "wasm32")]
    {
        let node_ref = node_ref.clone();
        let viewer_handle = viewer_handle.clone();
        let base_url = base_url.clone();

        Effect::new(move |_| {
            if viewer_handle.get_untracked().is_some() {
                return;
            }

            let Some(div) = node_ref.get() else {
                return;
            };

            let element: HtmlElement = div.into();
            set_base_url(base_url.as_deref().unwrap_or("/Cesium"));

            let viewer = Viewer::new(&element, &JsValue::UNDEFINED);
            viewer_context.set_viewer(&viewer);
            viewer_handle.set(Some(viewer));
        });
    }

    on_cleanup(move || {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(viewer) = viewer_handle.get_untracked() {
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
