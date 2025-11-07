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

            // Blue polygon entity
            <Entity
                name=Some("Blue Polygon".to_string())
                description=Some("A blue polygon".to_string())
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

            // Box entity
            <Entity
                name=Some("Orange Box".to_string())
                description=Some("A 3D box shape".to_string())
                position=Some(cartesian3_from_degrees(-106.0, 45.0, 200000.0))
            >
                <BoxGraphics
                    dimensions=Cartesian3::new(90000.0, 90000.0, 90000.0)
                    material=Some(Material::color(Color::yellow().with_alpha(0.8)))
                    outline=Some(true)
                    outline_color=Some(Color::white())
                    outline_width=Some(2.0)
                />
            </Entity>

            // Ellipsoid (Sphere) entity
            <Entity
                name=Some("Purple Sphere".to_string())
                description=Some("A spherical shape".to_string())
                position=Some(cartesian3_from_degrees(-102.0, 45.0, 200000.0))
            >
                <EllipsoidGraphics
                    radii=Cartesian3::new(67500.0, 67500.0, 67500.0)
                    material=Some(Material::color(Color::purple().with_alpha(0.8)))
                    outline=Some(true)
                    outline_color=Some(Color::white())
                    outline_width=Some(2.0)
                />
            </Entity>

            // Cylinder entity
            <Entity
                name=Some("Cyan Cylinder".to_string())
                description=Some("A cylindrical shape".to_string())
                position=Some(cartesian3_from_degrees(-70.0, 40.0, 200000.0))
            >
                <CylinderGraphics
                    length=400000.0
                    top_radius=0.0
                    bottom_radius=200000.0
                    material=Some(Material::color(Color::cyan().with_alpha(0.8)))
                    outline=Some(true)
                    outline_color=Some(Color::white())
                    outline_width=Some(4.0)
                />
            </Entity>

            // Wall entity
            <Entity
                name=Some("Wall".to_string())
                description=Some("A vertical wall structure".to_string())
            >
                <WallGraphics
                    positions=cartesian3_from_degrees_array_heights(&[
                        -90.0, 43.0, 100000.0,
                        -87.5, 45.0, 100000.0,
                        -85.0, 43.0, 100000.0,
                        -87.5, 41.0, 100000.0,
                        -90.0, 43.0, 100000.0,
                    ])
                    material=Some(Material::checkerboard(
                        CheckerboardOptions::new()
                            .even_color(Color::white())
                            .odd_color(Color::black())
                            .repeat(Cartesian2::new(20.0, 6.0))
                            .build()
                    ))
                />
            </Entity>

            // Corridor entity
            <Entity
                name=Some("Corridor".to_string())
                description=Some("A corridor path".to_string())
            >
                <CorridorGraphics
                    positions=cartesian3_from_degrees_array(&[
                        -120.0, 45.0,
                        -125.0, 50.0,
                        -125.0, 55.0,
                    ])
                    width=100000.0
                    material=Some(Material::color(Color::magenta().with_alpha(0.7)))
                    outline=Some(true)
                    outline_color=Some(Color::white())
                    outline_width=Some(4.0)
                />
            </Entity>

            // Polyline with glow
            <Entity
                name=Some("Glowing Polyline".to_string())
                description=Some("A polyline with glow effect".to_string())
            >
                <PolylineGraphics
                    positions={
                        let mut positions = js_sys::Array::new();
                        for i in 0..40 {
                            positions.push(&cartesian3_from_degrees(-100.0 + i as f64, 15.0, 0.0).into());
                        }
                        positions
                    }
                    width=10.0
                    material=Some(Material::polyline_glow(
                        PolylineGlowOptions::new()
                            .color(Color::deepskyblue())
                            .glow_power(0.25)
                            .build()
                    ))
                />
            </Entity>
        </ViewerContainer>
    }
}
