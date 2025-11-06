use leptos::prelude::*;
use leptos_cesium::components::ViewerContainer;

#[cfg(target_arch = "wasm32")]
use leptos_cesium::components::use_cesium_context;

#[cfg(target_arch = "wasm32")]
use js_sys::{Object, Reflect};
#[cfg(target_arch = "wasm32")]
use leptos_cesium::bindings::cartesian3_from_degrees;
#[cfg(target_arch = "wasm32")]
use leptos_cesium::cesium::Viewer;
#[cfg(target_arch = "wasm32")]
use leptos_cesium::core::JsRwSignal;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(target_arch = "wasm32")]
use web_sys::console;

#[component]
pub fn App() -> impl IntoView {
    // Load Cesium Ion token from environment variable at build time
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());

    view! {
        <ViewerContainer
            ion_token=ion_token
            class="cesium-viewer".to_string()
            style="width: 100%; height: 100%;".to_string()
        >
            <SceneBootstrap/>
        </ViewerContainer>
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
fn SceneBootstrap() -> impl IntoView {
    let viewer_context = use_cesium_context().expect("Cesium viewer context");
    let entity_handle: JsRwSignal<Option<JsValue>> = JsRwSignal::new_local(None);
    let (is_ready, set_ready) = signal(false);

    Effect::new(move |_| {
        console::debug_1(&JsValue::from_str("SceneBootstrap: effect tick"));
        if entity_handle.get().is_some() {
            console::debug_1(&JsValue::from_str(
                "SceneBootstrap: entity already created; skipping.",
            ));
            return;
        }

        let Some(entity_js) = viewer_context.with_viewer(|viewer: Viewer| {
            console::debug_1(&JsValue::from_str(
                "SceneBootstrap: viewer available; creating entity.",
            ));
            let entities = viewer.entities();

            let options = Object::new();
            let position = cartesian3_from_degrees(-74.0445, 40.6892, 150.0);
            Reflect::set(
                &options,
                &JsValue::from_str("position"),
                &JsValue::from(position),
            )
            .expect("position set");

            let point = Object::new();
            Reflect::set(
                &point,
                &JsValue::from_str("pixelSize"),
                &JsValue::from_f64(12.0),
            )
            .expect("point pixelSize");
            Reflect::set(&options, &JsValue::from_str("point"), &JsValue::from(point))
                .expect("point set");

            Reflect::set(
                &options,
                &JsValue::from_str("name"),
                &JsValue::from_str("Statue of Liberty"),
            )
            .expect("name set");

            let options_value: JsValue = options.into();
            let entity = entities.add_with_options(&options_value);
            console::debug_1(&JsValue::from_str(
                "SceneBootstrap: entity added to viewer.",
            ));
            JsValue::from(entity)
        }) else {
            console::debug_1(&JsValue::from_str(
                "SceneBootstrap: viewer not ready yet; waiting next tick.",
            ));
            return;
        };

        entity_handle.set(Some(entity_js));
        set_ready.set(true);
    });

    let cleanup_context = viewer_context;
    on_cleanup(move || {
        console::debug_1(&JsValue::from_str(
            "SceneBootstrap: cleanup invoked; removing entity if present.",
        ));
        if let Some(entity_js) = entity_handle.get_untracked() {
            cleanup_context.with_viewer(|viewer: Viewer| {
                let entity = entity_js.clone().unchecked_into();
                viewer.entities().remove(&entity);
                console::debug_1(&JsValue::from_str(
                    "SceneBootstrap: entity removed from viewer.",
                ));
            });
        }
        set_ready.set(false);
    });

    view! {
        <Show
            when=move || is_ready.get()
            fallback=|| view! { <p>"Loading Cesium..."</p> }
        >
            {|| view! { <></> }}
        </Show>
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[component]
fn SceneBootstrap() -> impl IntoView {
    view! { <p>"Cesium viewer is only available in the browser."</p> }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> });
}
