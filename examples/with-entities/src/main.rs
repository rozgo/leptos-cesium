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
                name=Some("Red Rectangle".to_string())
                description=Some("A red semi-transparent rectangle".to_string())
            >
                <RectangleGraphics
                    coordinates=Rectangle::from_degrees(-110.0, 20.0, -80.0, 25.0)
                    material=Some(Material::color(Color::red().with_alpha(0.5)))
                    outline=Some(true)
                    outline_color=Some(Color::black())
                />
            </Entity>

            // Blue polygon entity with hole
            <Entity
                name=Some("Blue Polygon".to_string())
                description=Some("A blue polygon with holes".to_string())
            >
                <PolygonGraphics
                    hierarchy={
                        let positions = cartesian3_from_degrees_array(&[
                            -115.0, 37.0,
                            -115.0, 32.0,
                            -107.0, 33.0,
                            -102.0, 31.0,
                            -102.0, 35.0,
                        ]);
                        PolygonHierarchy::new_simple(&positions.into())
                    }
                    material=Some(Material::color(Color::blue().with_alpha(0.5)))
                    outline=Some(true)
                    outline_color=Some(Color::white())
                />
            </Entity>

            // Green ellipse entity
            <Entity
                name=Some("Green Ellipse".to_string())
                description=Some("A green ellipse".to_string())
                position=Some(cartesian3_from_degrees(-95.0, 40.0, 0.0))
            >
                <EllipseGraphics
                    semi_minor_axis=300000.0
                    semi_major_axis=500000.0
                    material=Some(Material::color(Color::green().with_alpha(0.5)))
                    outline=Some(true)
                    outline_color=Some(Color::black())
                    rotation=Some(to_radians(45.0))
                />
            </Entity>

            // Striped rectangle entity
            <Entity
                name=Some("Striped Rectangle".to_string())
                description=Some("A rectangle with stripe pattern".to_string())
            >
                <RectangleGraphics
                    coordinates=Rectangle::from_degrees(-92.0, 30.0, -76.0, 36.0)
                    material=Some(Material::stripe(
                        StripeOptions::new()
                            .even_color(Color::white())
                            .odd_color(Color::blue())
                            .repeat(5.0)
                            .build()
                    ))
                    outline=Some(true)
                    outline_color=Some(Color::black())
                />
            </Entity>
        </ViewerContainer>
    }
}
