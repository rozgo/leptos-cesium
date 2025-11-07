use leptos::prelude::*;
use leptos_cesium::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());

    view! {
        <ViewerContainer ion_token=ion_token>
            // Red rectangle entity
            <Entity
                name=Signal::derive(|| Some("Red Rectangle".to_string()))
                description=Signal::derive(|| Some("A red semi-transparent rectangle".to_string()))
            >
                <RectangleGraphics
                    coordinates=create_js_signal(Rectangle::from_degrees(-110.0, 20.0, -80.0, 25.0))
                    material=create_js_signal(Some(Material::color(Color::red().with_alpha(0.5))))
                    outline=Signal::derive(|| Some(true))
                    outline_color=create_js_signal(Some(Color::black()))
                />
            </Entity>

            // Blue polygon entity with hole
            <Entity
                name=Signal::derive(|| Some("Blue Polygon".to_string()))
                description=Signal::derive(|| Some("A blue polygon with holes".to_string()))
            >
                <PolygonGraphics
                    hierarchy=create_js_signal({
                        let positions = cartesian3_from_degrees_array(&[
                            -115.0, 37.0,
                            -115.0, 32.0,
                            -107.0, 33.0,
                            -102.0, 31.0,
                            -102.0, 35.0,
                        ]);
                        PolygonHierarchy::new_simple(&positions.into())
                    })
                    material=create_js_signal(Some(Material::color(Color::blue().with_alpha(0.5))))
                    outline=Signal::derive(|| Some(true))
                    outline_color=create_js_signal(Some(Color::white()))
                />
            </Entity>

            // Green ellipse entity
            <Entity
                name=Signal::derive(|| Some("Green Ellipse".to_string()))
                description=Signal::derive(|| Some("A green ellipse".to_string()))
                position=create_js_signal(Some(cartesian3_from_degrees(-95.0, 40.0, 0.0)))
            >
                <EllipseGraphics
                    semi_minor_axis=Signal::derive(|| 300000.0)
                    semi_major_axis=Signal::derive(|| 500000.0)
                    material=create_js_signal(Some(Material::color(Color::green().with_alpha(0.5))))
                    outline=Signal::derive(|| Some(true))
                    outline_color=create_js_signal(Some(Color::black()))
                    rotation=Signal::derive(|| Some(to_radians(45.0)))
                />
            </Entity>

            // Striped rectangle entity
            <Entity
                name=Signal::derive(|| Some("Striped Rectangle".to_string()))
                description=Signal::derive(|| Some("A rectangle with stripe pattern".to_string()))
            >
                <RectangleGraphics
                    coordinates=create_js_signal(Rectangle::from_degrees(-92.0, 30.0, -76.0, 36.0))
                    material=create_js_signal(Some({
                        use js_sys::Object;
                        use wasm_bindgen::JsValue;
                        let options = Object::new();
                        let _ = js_sys::Reflect::set(&options, &JsValue::from_str("evenColor"), &JsValue::from(Color::white()));
                        let _ = js_sys::Reflect::set(&options, &JsValue::from_str("oddColor"), &JsValue::from(Color::blue()));
                        let _ = js_sys::Reflect::set(&options, &JsValue::from_str("repeat"), &JsValue::from_f64(5.0));
                        Material::stripe(StripeMaterialProperty::new(&options))
                    }))
                    outline=Signal::derive(|| Some(true))
                    outline_color=create_js_signal(Some(Color::black()))
                />
            </Entity>
        </ViewerContainer>
    }
}

/// Helper to create JsSignal from a value
fn create_js_signal<T: 'static>(value: T) -> leptos_cesium::core::JsSignal<T> {
    use leptos_cesium::core::JsRwSignal;
    JsRwSignal::new_local(value).read_only().into()
}
