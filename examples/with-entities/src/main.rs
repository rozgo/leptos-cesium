use geo_types::{LineString, Polygon, coord};
use leptos::prelude::*;
use leptos_cesium::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());

    // Define some reusable colors for outline (materials still use Cesium Color for now)
    let black = Srgba::new(0.0, 0.0, 0.0, 1.0);
    let white = Srgba::new(1.0, 1.0, 1.0, 1.0);

    view! {
        <ViewerContainer ion_token=ion_token>
            // Red rectangle entity - using geo_types::Rect
            <Entity
                name=Some("Red Rectangle".to_string())
                description=Some("A red semi-transparent rectangle".to_string())
            >
                <RectangleGraphics
                    // Rect: (min_x, min_y) to (max_x, max_y) in degrees
                    coordinates=Rect::new(
                        coord! { x: -110.0, y: 20.0 },
                        coord! { x: -80.0, y: 25.0 }
                    )
                    material=Some(Material::color(Color::red().with_alpha(0.5)))
                    outline=Some(true)
                    outline_color=Some(black)
                />
            </Entity>

            // Blue polygon entity - using geo_types::Polygon
            <Entity
                name=Some("Blue Polygon".to_string())
                description=Some("A blue polygon".to_string())
            >
                <PolygonGraphics
                    hierarchy=Polygon::new(
                        LineString::new(vec![
                            coord! { x: -115.0, y: 37.0 },
                            coord! { x: -115.0, y: 32.0 },
                            coord! { x: -107.0, y: 33.0 },
                            coord! { x: -102.0, y: 31.0 },
                            coord! { x: -102.0, y: 35.0 },
                            coord! { x: -115.0, y: 37.0 }, // Close the ring
                        ]),
                        vec![] // No holes
                    )
                    material=Some(Material::color(Color::blue().with_alpha(0.5)))
                    outline=Some(true)
                    outline_color=Some(white)
                />
            </Entity>

            // Green ellipse entity
            <Entity
                name=Some("Green Ellipse".to_string())
                description=Some("A green ellipse".to_string())
                position=Some(DVec3::new(-95.0, 40.0, 0.0))
            >
                <EllipseGraphics
                    semi_minor_axis=300000.0
                    semi_major_axis=500000.0
                    material=Some(Material::color(Color::green().with_alpha(0.5)))
                    outline=Some(true)
                    outline_color=Some(black)
                    rotation=Some(to_radians(45.0))
                />
            </Entity>

            // Striped rectangle entity
            <Entity
                name=Some("Striped Rectangle".to_string())
                description=Some("A rectangle with stripe pattern".to_string())
            >
                <RectangleGraphics
                    coordinates=Rect::new(
                        coord! { x: -92.0, y: 30.0 },
                        coord! { x: -76.0, y: 36.0 }
                    )
                    material=Some(Material::stripe(
                        StripeOptions::new()
                            .even_color(Color::white())
                            .odd_color(Color::blue())
                            .repeat(5.0)
                            .build()
                    ))
                    outline=Some(true)
                    outline_color=Some(black)
                />
            </Entity>

            // Box entity - using DVec3 for dimensions
            <Entity
                name=Some("Orange Box".to_string())
                description=Some("A 3D box shape".to_string())
                position=Some(DVec3::new(-106.0, 45.0, 200000.0))
            >
                <BoxGraphics
                    // DVec3 for dimensions: x=width, y=height, z=depth in meters
                    dimensions=DVec3::new(90000.0, 90000.0, 90000.0)
                    material=Some(Material::color(Color::yellow().with_alpha(0.8)))
                    outline=Some(true)
                    outline_color=Some(white)
                    outline_width=Some(2.0)
                />
            </Entity>

            // Ellipsoid (Sphere) entity - using DVec3 for radii
            <Entity
                name=Some("Purple Sphere".to_string())
                description=Some("A spherical shape".to_string())
                position=Some(DVec3::new(-102.0, 45.0, 200000.0))
            >
                <EllipsoidGraphics
                    // DVec3 for radii: x, y, z radii in meters
                    radii=DVec3::new(67500.0, 67500.0, 67500.0)
                    material=Some(Material::color(Color::purple().with_alpha(0.8)))
                    outline=Some(true)
                    outline_color=Some(white)
                    outline_width=Some(2.0)
                />
            </Entity>

            // Cylinder entity (cone shape)
            <Entity
                name=Some("Cyan Cylinder".to_string())
                description=Some("A cylindrical shape".to_string())
                position=Some(DVec3::new(-70.0, 40.0, 200000.0))
            >
                <CylinderGraphics
                    length=400000.0
                    top_radius=0.0
                    bottom_radius=200000.0
                    material=Some(Material::color(Color::cyan().with_alpha(0.8)))
                    outline=Some(true)
                    outline_color=Some(white)
                    outline_width=Some(4.0)
                />
            </Entity>

            // Wall entity - using LineString for positions
            <Entity
                name=Some("Wall".to_string())
                description=Some("A vertical wall structure".to_string())
            >
                <WallGraphics
                    positions=LineString::new(vec![
                        coord! { x: -90.0, y: 43.0 },
                        coord! { x: -87.5, y: 45.0 },
                        coord! { x: -85.0, y: 43.0 },
                        coord! { x: -87.5, y: 41.0 },
                        coord! { x: -90.0, y: 43.0 },
                    ])
                    maximum_heights=Some(vec![100000.0, 100000.0, 100000.0, 100000.0, 100000.0])
                    material=Some(Material::checkerboard(
                        CheckerboardOptions::new()
                            .even_color(Color::white())
                            .odd_color(Color::black())
                            .repeat(Cartesian2::new(20.0, 6.0))
                            .build()
                    ))
                />
            </Entity>

            // Corridor entity - using LineString for positions
            <Entity
                name=Some("Corridor".to_string())
                description=Some("A corridor path".to_string())
            >
                <CorridorGraphics
                    positions=LineString::new(vec![
                        coord! { x: -120.0, y: 45.0 },
                        coord! { x: -125.0, y: 50.0 },
                        coord! { x: -125.0, y: 55.0 },
                    ])
                    width=100000.0
                    material=Some(Material::color(Color::magenta().with_alpha(0.7)))
                    outline=Some(true)
                    outline_color=Some(white)
                    outline_width=Some(4.0)
                />
            </Entity>

            // Polyline with glow - using LineString for positions
            <Entity
                name=Some("Glowing Polyline".to_string())
                description=Some("A polyline with glow effect".to_string())
            >
                <PolylineGraphics
                    positions={
                        let coords: Vec<_> = (0..40)
                            .map(|i| coord! { x: -100.0 + i as f64, y: 15.0 })
                            .collect();
                        LineString::new(coords)
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
