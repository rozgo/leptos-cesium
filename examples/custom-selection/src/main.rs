use leptos::prelude::*;
use leptos_cesium::prelude::*;

#[component]
fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());

    view! {
        <ViewerContainer
            ion_token=ion_token
            info_box=false  // Disable Cesium's default InfoBox
            selection_indicator=true  // Keep the green selection indicator
            style="width: 100vw; height: 100vh;".to_string()
        >
            <CameraSetup />
            <Instructions />
            <CustomSelectionPanel />

            // Create small entities clustered together
            // Using very small degree differences (0.00001 deg ≈ 1 meter)

            <Entity
                name="Red Box".to_string()
                description="A small red cube".to_string()
                position=Cartesian3::from_degrees(-75.59777, 40.03883, 1.0)
            >
                <BoxGraphics
                    dimensions=Cartesian3::new(2.0, 2.0, 2.0)
                    material=Some(Material::color(Color::red().with_alpha(0.8)))
                    outline=Some(true)
                    outline_color=Some(Color::black())
                />
            </Entity>

            <Entity
                name="Blue Sphere".to_string()
                description="A small blue sphere".to_string()
                position=Cartesian3::from_degrees(-75.59775, 40.03883, 1.0)
            >
                <EllipsoidGraphics
                    radii=Cartesian3::new(1.0, 1.0, 1.0)
                    material=Some(Material::color(Color::blue().with_alpha(0.9)))
                    outline=Some(true)
                    outline_color=Some(Color::white())
                />
            </Entity>

            <Entity
                name="Green Cylinder".to_string()
                description="A small green cylinder".to_string()
                position=Cartesian3::from_degrees(-75.59779, 40.03885, 1.5)
            >
                <CylinderGraphics
                    length=3.0
                    top_radius=0.8
                    bottom_radius=0.8
                    material=Some(Material::color(Color::green().with_alpha(0.85)))
                    outline=Some(true)
                    outline_color=Some(Color::gray())
                />
            </Entity>

            <Entity
                name="Yellow Rectangle".to_string()
                description="A small flat rectangle".to_string()
            >
                <RectangleGraphics
                    coordinates=Rectangle::from_degrees(
                        -75.59780, 40.03881,
                        -75.59774, 40.03885
                    )
                    material=Some(Material::checkerboard(
                        CheckerboardOptions::new()
                            .even_color(Color::yellow())
                            .odd_color(Color::white())
                            .repeat(Cartesian2::new(2.0, 2.0))
                            .build()
                    ))
                    height=Some(0.0)
                    outline=Some(true)
                    outline_color=Some(Color::black())
                />
            </Entity>

            <Entity
                name="Purple Square".to_string()
                description="A square path around the entities".to_string()
            >
                <PolylineGraphics
                    positions=Cartesian3::from_degrees_array_heights(&[
                        // Southwest corner
                        -75.59782, 40.03879, 0.5,
                        // Southeast corner
                        -75.59772, 40.03879, 0.5,
                        // Northeast corner
                        -75.59772, 40.03887, 0.5,
                        // Northwest corner
                        -75.59782, 40.03887, 0.5,
                        // Back to Southwest to close the square
                        -75.59782, 40.03879, 0.5,
                    ])
                    width=3.0
                    material=Some(Material::polyline_glow(
                        PolylineGlowOptions::new()
                            .color(Color::purple())
                            .glow_power(0.2)
                            .build()
                    ))
                />
            </Entity>

            // Position camera close to the entities
            <CameraSetView
                destination=Cartesian3::from_degrees(-75.59770, 40.03880, 15.0)
                orientation=HeadingPitchRoll::new(0.3, -0.5, 0.0)
            />
        </ViewerContainer>
    }
}

#[component]
fn Instructions() -> impl IntoView {
    let (show, set_show) = signal(true);

    view! {
        <Show when=move || show.get()>
            <div class="instructions">
                <button
                    class="close-button"
                    on:click=move |_| set_show.set(false)
                    style="position: absolute; top: 8px; right: 8px;"
                >
                    "×"
                </button>
                <h3>"Custom Selection Panel"</h3>
                <p>"Click on any colored shape in the center of the view to see its details."</p>
                <ul>
                    <li>"Red Box"</li>
                    <li>"Blue Sphere"</li>
                    <li>"Green Cylinder"</li>
                    <li>"Yellow Rectangle"</li>
                    <li>"Purple Path"</li>
                </ul>
                <p style="margin-top: 12px; font-size: 12px; opacity: 0.8;">
                    "The default InfoBox is disabled - we're using a custom panel instead!"
                </p>
            </div>
        </Show>
    }
}

#[component]
fn CustomSelectionPanel() -> impl IntoView {
    let viewer_context = use_cesium_context().expect("Must be inside ViewerContainer");

    // Get reactive signal that tracks when selection changes
    let selection_version = viewer_context.selection_version();

    view! {
        <Show when=move || {
            // Track selection changes
            selection_version.get();
            // Check if there's a selected entity
            viewer_context.selected_entity().is_some()
        }>
            <div class="custom-selection-panel">
                <button
                    class="close-button"
                    on:click=move |_| {
                        // Clear selection when close is clicked
                        viewer_context.clear_selected_entity();
                    }
                >
                    "×"
                </button>

                <div class="panel-header">
                    "Entity Details"
                </div>

                <div class="panel-content">
                    {move || {
                        // Track selection changes and get entity
                        selection_version.get();
                        viewer_context.selected_entity().map(|entity| {
                            // Extract entity properties using the properly-typed API
                            let name = entity.name().unwrap_or_else(|| "N/A".to_string());
                            let description = entity.description()
                                .and_then(|prop| prop.as_string())
                                .unwrap_or_else(|| "N/A".to_string());
                            let id = entity.id();
                            let position_str = entity.position()
                                .and_then(|pos| pos.value())
                                .map(|cart| format_cartesian3(&cart))
                                .unwrap_or_else(|| "N/A".to_string());

                            view! {
                                <div>
                                    <div class="property-row">
                                        <div class="property-label">"Name"</div>
                                        <div class="property-value">{name}</div>
                                    </div>

                                    <div class="property-row">
                                        <div class="property-label">"Description"</div>
                                        <div class="property-value">{description}</div>
                                    </div>

                                    <div class="property-row">
                                        <div class="property-label">"Entity ID"</div>
                                        <div class="property-value" style="font-family: monospace; font-size: 12px;">
                                            {id}
                                        </div>
                                    </div>

                                    <div class="property-row">
                                        <div class="property-label">"Position"</div>
                                        <div class="property-value">
                                            {position_str}
                                        </div>
                                    </div>
                                </div>
                            }
                        })
                    }}
                </div>
            </div>
        </Show>
    }
}

// Helper function to format Cartesian3 position
fn format_cartesian3(cart: &leptos_cesium::bindings::Cartesian3) -> String {
    // Use the proper getters instead of reflection
    let x = cart.x();
    let y = cart.y();
    let z = cart.z();
    format!("({:.2}, {:.2}, {:.2})", x, y, z)
}

#[component]
fn CameraSetup() -> impl IntoView {
    let viewer_context = use_cesium_context().expect("CameraSetup must be inside ViewerContainer");

    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            viewer_context.with_viewer(|viewer| {
                // Use zoomTo to automatically frame all entities
                let entities = viewer.entities();
                let _ = viewer.zoom_to(&entities.into());
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = viewer_context;
        }
    });
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> });
}
