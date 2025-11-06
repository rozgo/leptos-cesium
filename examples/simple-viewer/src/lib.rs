use leptos::prelude::*;
use leptos_cesium::components::ViewerContainer;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ViewerContainer class="cesium-viewer".to_string() style="width: 100%; height: 400px;".to_string()>
            <p>{"Cesium viewer placeholder."}</p>
        </ViewerContainer>
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start_app() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    // Placeholder main for non-wasm targets so `cargo check` succeeds.
}
