# leptos-cesium

`leptos-cesium` provides a CesiumJS component library for the [Leptos](https://github.com/leptos-rs/leptos) framework (0.8.15). It uses standard Rust types (glam, geo-types, palette) for SSR compatibility, exposing Cesium concepts (viewer, entities, data sources, 3D tiles) through idiomatic Leptos components.

![cesium-with-entities](docs/cesium-with-entities.jpg)

## Gallery

<table>
  <tr>
    <td width="33%" align="center">
      <img src="docs/cesium-satellites.jpg" alt="Satellite Orbits" width="100%">
      <br>
      <b>Satellite Tracking</b>
      <br>
      Visualize satellite orbits and trajectories
    </td>
    <td width="33%" align="center">
      <img src="docs/cesium-terrain.jpg" alt="3D Terrain" width="100%">
      <br>
      <b>3D Terrain</b>
      <br>
      High-resolution terrain and imagery
    </td>
    <td width="33%" align="center">
      <img src="docs/cesium-vehicle.jpg" alt="Vehicle Tracking" width="100%">
      <br>
      <b>Vehicle Paths</b>
      <br>
      Track vehicles and paths over terrain
    </td>
  </tr>
</table>

## Repository Layout

- `leptos-cesium/` – main library crate (bindings, components, core utilities)
- `examples/` – example Leptos apps showcasing Cesium usage

## Getting Started

### 1. Install prerequisites

- Rust toolchain with the `wasm32-unknown-unknown` target
- `trunk` CLI (`cargo install trunk`)

### 2. Configure your Cesium Ion token

Copy the template and add your token:

```bash
cp .env.example .env.local
# Edit .env.local and paste your token
```

Get your free token from: https://ion.cesium.com/tokens

### 3. Run examples

**Simple viewer (basic globe):**
```bash
cd examples/simple-viewer
trunk serve --open
```

**Entities example (shapes and materials):**
```bash
cd examples/with-entities
trunk serve --open
```
Demonstrates declarative entity components: 2D shapes (rectangles, polygons, ellipses), 3D primitives (boxes, spheres, cylinders), paths (polylines, corridors, walls), and various materials (colors, stripes, checkerboard, glow).

**CZML data loading (satellites, animations):**
```bash
cd examples/czml-viewer
trunk serve --open
```
Demonstrates CZML data source loading with automatic clock synchronization and camera controls.

**GeoJSON data loading (maps, geographic features):**
```bash
cd examples/geojson
trunk serve --open
```
Demonstrates GeoJSON data source loading with custom styling for polygons, polylines, and point markers. Features reactive layer switching and styling options.

**Camera controls (animated positioning):**
```bash
cd examples/camera-control
trunk serve --open
```
Demonstrates declarative camera controls including fly-to animations, view positioning, and camera movements.

**Custom selection UI (reactive selected entity panel):**
```bash
cd examples/custom-selection
trunk serve --open
```
Demonstrates replacing the default Cesium InfoBox with a Leptos-driven custom selection panel.

**Google Photorealistic 3D Tiles:**
```bash
cd examples/google-3d-tiles
trunk serve --open
```
Demonstrates loading Cesium Ion/Google 3D Tiles with cache and collision configuration.

**Server-side rendering:**
```bash
cd examples/with-server
cargo leptos watch
```
Visit http://localhost:3000

### What happens at build time?

1. **Environment loading**: Cargo reads `CESIUM_ION_TOKEN` from `.env.local` at build time
2. **Token passing**: Token is passed to `<ViewerContainer ion_token=... />` component prop
3. **Cesium loading**: HTML loads `Cesium.js` from CDN synchronously in `<head>`
4. **WASM loading**: Trunk injects the WASM module at the `<link data-trunk rel="rust">` location
5. **Viewer creation**: Component sets CDN base URL, `Cesium.Ion.defaultAccessToken`, and creates viewer instance

### Development Tips

- Run `cargo check --workspace` from the repository root for a full compile check
- Run `cargo test -p leptos-cesium --lib` for native unit tests
- Run `cargo test -p leptos-cesium --lib --target wasm32-unknown-unknown --no-run` to verify wasm test compilation
- Cesium is loaded from CDN (cesium.com) - no local assets required
- If you rotate Ion tokens, edit `.env.local` and rebuild
- For troubleshooting, see `CLAUDE.md`

## Features

### Declarative Components

Create Cesium entities with clean, type-safe Rust using standard ecosystem types:

```rust
use leptos::prelude::*;
use leptos_cesium::prelude::*;
use geo_types::coord;

view! {
    <ViewerContainer ion_token=token>
        <Entity name="My Rectangle">
            <RectangleGraphics
                coordinates=Rect::new(
                    coord! { x: -110.0, y: 20.0 },
                    coord! { x: -80.0, y: 25.0 }
                )
                material=Some(Material::color(Color::red().with_alpha(0.5)))
                outline=Some(true)
                outline_color=Some(Srgba::new(0.0, 0.0, 0.0, 1.0))
            />
        </Entity>
    </ViewerContainer>
}
```

### Supported Graphics

**2D Shapes:**
- **RectangleGraphics** - Rectangles on the globe surface
- **PolygonGraphics** - Polygons with optional holes
- **EllipseGraphics** - Ellipses with rotation support

**3D Primitives:**
- **BoxGraphics** - Cuboid shapes with customizable dimensions
- **EllipsoidGraphics** - Spheres and ellipsoids with radii control
- **CylinderGraphics** - Cylinders and cones with adjustable radii

**Paths & Volumes:**
- **PolylineGraphics** - Lines with width and material styling
- **WallGraphics** - Vertical walls with height control
- **CorridorGraphics** - Corridor paths with width and extrusion
- **PolylineVolumeGraphics** - Custom 2D shapes extruded along paths

**Points & Markers:**
- **PointGraphics** - Point markers with pixel size and color customization

### Materials

All materials use a fluent builder API for clean, type-safe configuration:

**Color Material:**
```rust
Material::color(Color::red().with_alpha(0.5))
```

**Stripe Material:**
```rust
Material::stripe(
    StripeOptions::new()
        .even_color(Color::white())
        .odd_color(Color::blue())
        .repeat(5.0)
        .build()
)
```

**Checkerboard Material:**
```rust
Material::checkerboard(
    CheckerboardOptions::new()
        .even_color(Color::white())
        .odd_color(Color::black())
        .repeat(Cartesian2::new(20.0, 6.0))
        .build()
)
```

**Polyline Glow Material:**
```rust
Material::polyline_glow(
    PolylineGlowOptions::new()
        .color(Color::deepskyblue())
        .glow_power(0.25)
        .build()
)
```

### Camera Controls

Declarative camera positioning and animation using `DVec3` for destinations:

```rust
use leptos_cesium::prelude::*;

view! {
    <ViewerContainer ion_token=token>
        // Instant camera positioning (DVec3: longitude, latitude, height)
        <CameraSetView
            destination=DVec3::new(-75.0, 40.0, 1000.0)
        />

        // Animated flight to location
        <CameraFlyTo
            destination=DVec3::new(-122.4, 37.8, 5000.0)
            duration=3.0
        />

        // Fly to home view
        <CameraFlyHome duration=2.0 />

        // Reset clock to current time
        <ClockReset />
    </ViewerContainer>
}
```

### Viewer Events

Attach viewer-level event handlers (selection/tracking) using `ViewerEvents`:

```rust
use leptos::prelude::*;
use leptos_cesium::prelude::*;

#[component]
fn ViewerEventHooks() -> impl IntoView {
    let viewer_context = use_cesium_context().expect("must be inside ViewerContainer");

    let events = ViewerEvents::new()
        .set_selected_entity_changed(|value| {
            leptos::logging::log!("selected_entity_changed: {:?}", value);
        })
        .set_tracked_entity_changed(|value| {
            leptos::logging::log!("tracked_entity_changed: {:?}", value);
        });

    let setup_events = events.clone();
    Effect::new(move |_| {
        let _ = viewer_context.with_viewer(|viewer| setup_events.setup(&viewer));
    });

    on_cleanup(move || events.teardown());

    ().into_view()
}

view! {
    <ViewerContainer ion_token=token>
        <ViewerEventHooks />
    </ViewerContainer>
}
```

### Data Sources

**CZML Data Source:**

Load dynamic data from CZML format:

```rust
use leptos_cesium::prelude::*;

view! {
    <ViewerContainer ion_token=token>
        <CzmlDataSource
            url="satellite-orbit.czml"
            clear_existing=true
        />
    </ViewerContainer>
}
```

CZML data sources automatically synchronize the viewer clock with animation timelines.

**GeoJSON Data Source:**

Load and style GeoJSON or TopoJSON data:

```rust
use leptos_cesium::prelude::*;

view! {
    <ViewerContainer ion_token=token>
        <GeoJsonDataSource
            url="data/countries.geojson"
            stroke=Color::blue()
            stroke_width=2.0
            fill=Color::cyan().with_alpha(0.3)
            marker_color=Color::red()
            marker_size=24.0
            clamp_to_ground=false
        />
    </ViewerContainer>
}
```

Supports extensive styling options for polygons, polylines, and point markers.

### 3D Tiles

Load high-resolution 3D tile datasets:

**Google Photorealistic 3D Tiles:**

```rust
use leptos_cesium::prelude::*;

view! {
    <ViewerContainer ion_token=token>
        <GooglePhotorealistic3DTiles
            google_api_key=None  // Uses Cesium Ion asset by default
            cache_bytes=Some(1536000000)
            enable_collision=Some(true)
        />
    </ViewerContainer>
}
```

Loads Google's photorealistic 3D tiles via Cesium Ion or directly with a Google Maps API key.

## Project Status

**Implemented:**
- ✅ **Leptos 0.8.15** compatibility with standard Rust types (glam, geo-types, palette)
- ✅ ViewerContainer with Ion token support and configurable UI widgets
- ✅ Entity component with declarative graphics
- ✅ 2D Graphics: Rectangle, Polygon, Ellipse
- ✅ 3D Primitives: Box, Ellipsoid, Cylinder
- ✅ Paths & Volumes: Polyline, Wall, Corridor, PolylineVolume
- ✅ Points: PointGraphics with pixel size and color control
- ✅ Materials: Color, Stripe, Checkerboard, PolylineGlow (all with builder APIs)
- ✅ Camera Controls: CameraFlyTo, CameraSetView, CameraFlyHome, CameraFlyToBoundingSphere
- ✅ Clock Controls: ClockReset for animation timeline management
- ✅ Data Sources: CZML with automatic clock synchronization, GeoJSON with extensive styling options
- ✅ 3D Tiles: Google Photorealistic 3D Tiles with cache and collision controls
- ✅ Coordinate Helpers: Cartesian2, Cartesian3, Rectangle, PolygonHierarchy
- ✅ Math Utilities: to_radians, to_degrees, HeadingPitchRoll, HeadingPitchRange
- ✅ Server-side rendering support with thread-safe JsValue wrappers
- ✅ Builder APIs for complex options (FlyToOptions, SetViewOptions, StripeOptions, GeoJsonLoadOptions, etc.)
- ✅ Viewer event builders via `cesium_events!` (currently `ViewerEvents` for selected/tracked entity changes)
- ✅ Strict lifecycle/resource ownership for data sources, primitives, and viewer event listeners

**Planned:**
- 🔲 Additional graphics types (Model, Billboard, Label, Path)
- 🔲 Additional data sources (KML, GPX)
- 🔲 Custom 3D Tileset loading (from URL or Ion asset ID)
- 🔲 Expanded event coverage (click, hover, camera/mouse interactions)
- 🔲 Additional camera controls (lookAt, viewer tracking)
- 🔲 Imagery providers (custom base layers)
- 🔲 Terrain providers (custom terrain data)
- 🔲 PostProcessing effects
- 🔲 Primitives API (low-level rendering)

Contributions are welcome!
