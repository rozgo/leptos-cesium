---
name: leptos-cesium
description: CesiumJS integration for Leptos 0.8.x. Provides declarative Cesium components (viewer, entities, graphics, camera, data sources) using standard Rust types (glam, geo-types, palette). Use when working with leptos-cesium crate, editing Cesium components, or adding 3D globe/map functionality to a Leptos app. For general Leptos patterns, see the leptos skill.
---

# leptos-cesium Skill

Declarative CesiumJS components for Leptos. Uses standard Rust types (glam, geo-types, palette) for SSR compatibility.

**For general Leptos patterns** (signals, components, SSR setup, routing): see the `leptos` skill.

## Current Baseline

- Cesium CDN/runtime target: `1.138`
- Primary integration surface: `ViewerContainer`, `Entity` + graphics, camera controls, data sources, 3D tiles
- Viewer events available via `ViewerEvents` and `cesium_events!`
- Strict lifecycle ownership is implemented for async loads/listeners:
  - stale async requests are gated (`RequestGate`)
  - component-owned resources are cleaned up deterministically (`OwnedSlot`)

## Prop Style

Use the same ergonomic style as the real examples in this repo:

```rust
<Entity
    name="Red Box".to_string()
    description="A small red cube".to_string()
    position=DVec3::new(-75.59777, 40.03883, 1.0)
>
    <BoxGraphics
        dimensions=DVec3::new(2.0, 2.0, 2.0)
        material=Some(Material::color(Color::red().with_alpha(0.8)))
    />
</Entity>
```

`#[prop(optional, into)]` accepts both direct values and explicit `Some(...)`, but prefer direct values in examples for consistency with `examples/custom-selection`.

## Quick Start

```rust
use leptos::prelude::*;
use leptos_cesium::prelude::*;

#[component]
fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());

    view! {
        <ViewerContainer ion_token=ion_token style="width: 100%; height: 100%;".to_string()>
            <Entity name="My Entity".to_string() position=DVec3::new(-74.0, 40.7, 100.0)>
                <PointGraphics pixel_size=12.0 color=Some(Srgba::new(1.0, 0.0, 0.0, 1.0)) />
            </Entity>

            <CameraSetView destination=DVec3::new(-74.0, 40.7, 5000.0) />
        </ViewerContainer>
    }
}
```

## Type System

**Props use standard Rust types, NOT Cesium JS types.** Conversion happens inside Effects (WASM-only).

| Rust Type | Use Case | Example |
|-----------|----------|---------|
| `glam::DVec3` | Position (lon, lat, height) | `DVec3::new(-74.0, 40.7, 150.0)` |
| `geo_types::Rect<f64>` | Rectangle bounds | `Rect::new(coord!{x: -110.0, y: 20.0}, coord!{x: -80.0, y: 25.0})` |
| `geo_types::LineString<f64>` | Polyline positions | `LineString::new(vec![coord!{x: -90.0, y: 43.0}, ...])` |
| `geo_types::Polygon<f64>` | Polygon with holes | `Polygon::new(exterior_ring, vec![])` |
| `palette::Srgba<f32>` | RGBA color (0.0-1.0) | `Srgba::new(1.0, 0.0, 0.0, 1.0)` |
| `(f64, f64, f64)` | Orientation (heading, pitch, roll) | `(0.0, -1.57, 0.0)` radians |

All types re-exported from `leptos_cesium::prelude::*`.

## Component Hierarchy

```
ViewerContainer (root, provides context)
├── Entity (creates Cesium entity)
│   ├── PointGraphics
│   ├── RectangleGraphics
│   ├── PolygonGraphics
│   ├── PolylineGraphics
│   ├── BoxGraphics
│   ├── EllipsoidGraphics
│   ├── CylinderGraphics
│   ├── EllipseGraphics
│   ├── WallGraphics
│   ├── CorridorGraphics
│   └── PolylineVolumeGraphics
├── CameraSetView / CameraFlyTo / CameraFlyHome
├── CzmlDataSource / GeoJsonDataSource
└── GooglePhotorealistic3DTiles
```

## Core Components

### ViewerContainer

Root component. Creates Cesium Viewer and provides context.

```rust
<ViewerContainer
    ion_token=Some("your-token".to_string())  // Cesium Ion token
    animation=false       // Hide animation widget
    timeline=false        // Hide timeline widget
    info_box=false        // Hide info box on selection
    style="width: 100%; height: 100%;".to_string()
>
    // children
</ViewerContainer>
```

### Entity + Graphics

Entity wraps graphics components. Position uses DVec3.

```rust
<Entity
    name="Point of Interest".to_string()
    position=DVec3::new(-74.0445, 40.6892, 150.0)  // (lon, lat, height)
    description="The Statue of Liberty".to_string()
>
    <PointGraphics pixel_size=20.0 color=Some(Srgba::new(0.0, 1.0, 0.0, 1.0)) />
</Entity>
```

### Rectangle (geo_types::Rect)

```rust
use geo_types::coord;

<Entity name="Rectangle".to_string()>
    <RectangleGraphics
        coordinates=Rect::new(
            coord! { x: -110.0, y: 20.0 },  // min (west, south)
            coord! { x: -80.0, y: 25.0 }    // max (east, north)
        )
        material=Some(Material::color(Color::red().with_alpha(0.5)))
        outline=Some(true)
        outline_color=Some(Srgba::new(0.0, 0.0, 0.0, 1.0))
    />
</Entity>
```

### Polygon (geo_types::Polygon)

```rust
<Entity name="Polygon".to_string()>
    <PolygonGraphics
        hierarchy=Polygon::new(
            LineString::new(vec![
                coord! { x: -115.0, y: 37.0 },
                coord! { x: -115.0, y: 32.0 },
                coord! { x: -107.0, y: 33.0 },
                coord! { x: -115.0, y: 37.0 },  // Close ring
            ]),
            vec![]  // No holes
        )
        material=Some(Material::color(Color::blue().with_alpha(0.5)))
        extruded_height=Some(50000.0)
    />
</Entity>
```

### Polyline (geo_types::LineString)

```rust
<Entity name="Polyline".to_string()>
    <PolylineGraphics
        positions=LineString::new(vec![
            coord! { x: -75.0, y: 35.0 },
            coord! { x: -125.0, y: 35.0 },
            coord! { x: -125.0, y: 45.0 },
        ])
        width=5.0
        material=Some(Material::color(Color::yellow()))
    />
</Entity>
```

## Camera Controls

```rust
// Instant position (DVec3: lon, lat, height)
<CameraSetView
    destination=DVec3::new(-116.52, 35.02, 95000.0)
    orientation=Some((0.0, -1.57, 0.0))  // Optional (heading, pitch, roll)
/>

// Animated flight
<CameraFlyTo
    destination=DVec3::new(-122.4, 37.8, 5000.0)
    duration=3.0
/>

// Fly home (triggered by signal)
<CameraFlyHome trigger=go_home_signal duration=2.0 />

// Reset clock (for CZML animations)
<ClockReset trigger=reset_signal />
```

**Omit orientation** to let Cesium use default pitch (looking down at target).

## Materials

```rust
// Solid color
Material::color(Color::red().with_alpha(0.5))

// Stripe pattern
Material::stripe(
    StripeOptions::new()
        .even_color(Color::white())
        .odd_color(Color::blue())
        .repeat(5.0)
        .build()
)

// Checkerboard
Material::checkerboard(
    CheckerboardOptions::new()
        .even_color(Color::white())
        .odd_color(Color::black())
        .repeat(Cartesian2::new(20.0, 6.0))
        .build()
)

// Polyline glow (polylines only)
Material::polyline_glow(
    PolylineGlowOptions::new()
        .color(Color::deepskyblue())
        .glow_power(0.25)
        .build()
)
```

## Data Sources

### CZML

```rust
<CzmlDataSource
    url="satellite.czml"
    clear_existing=true  // Remove previous data sources
/>
```

### GeoJSON

```rust
<GeoJsonDataSource
    url="countries.geojson"
    stroke=Color::blue()
    stroke_width=2.0
    fill=Color::cyan().with_alpha(0.3)
    marker_color=Color::red()
    marker_size=24.0
    clamp_to_ground=false
/>
```

## Viewer Events

Use `ViewerEvents` (generated by `cesium_events!`) for typed viewer-level callbacks.

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
```

### Selection helper pattern

`ViewerContainer` updates selection context internally from Cesium events.
For reactive custom panels:

- use `let version = viewer_context.selection_version();`
- track via `version.get()` in `Show`/closures
- read selected entity via `viewer_context.selected_entity()`
- clear with `viewer_context.clear_selected_entity()`

## SSR/WASM Pattern

Graphics components must handle both SSR and WASM builds:

```rust
#[component(transparent)]
pub fn MyGraphics(
    #[prop(into)] value: Signal<f64>,
    #[prop(optional, into)] material: JsSignal<Option<Material>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let entity_context = use_entity_context().expect("Must be child of Entity");
        Effect::new(move |_| {
            let val = value.get();
            let mat = material.get();  // Track JsSignal reactively
            // ... set properties on entity
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (value, material);  // Consume to avoid warnings
    }
}
```

**Key rules:**
- `Signal<T>` for primitives → use `.get()`
- `JsSignal<T>` for JS types → use `.get()` when reactive updates are desired; use `.get_untracked()` only when you explicitly want to avoid re-triggers
- Always consume props in both `#[cfg]` branches

## Bindings Pattern

Cesium uses namespaces, not ES6 classes. Static methods require reflection:

```rust
// Access static property (e.g., Color.RED)
fn get_color_property(name: &str) -> Color {
    use js_sys::{global, Reflect};
    use wasm_bindgen::JsCast;

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium")).unwrap();
    let color_class = Reflect::get(&cesium, &JsValue::from_str("Color")).unwrap();
    Reflect::get(&color_class, &JsValue::from_str(name))
        .unwrap()
        .unchecked_into::<Color>()
}

// Call static method (e.g., Rectangle.fromDegrees())
fn from_degrees(west: f64, south: f64, east: f64, north: f64) -> Rectangle {
    use js_sys::{global, Function, Reflect};

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium")).unwrap();
    let rect_class = Reflect::get(&cesium, &JsValue::from_str("Rectangle")).unwrap();
    let fn_ref = Reflect::get(&rect_class, &JsValue::from_str("fromDegrees")).unwrap();
    let func: Function = fn_ref.dyn_into().unwrap();
    func.call4(&rect_class, &west.into(), &south.into(), &east.into(), &north.into())
        .unwrap()
        .unchecked_into::<Rectangle>()
}
```

## Common Issues

| Issue | Solution |
|-------|----------|
| Black screen | Check `Trunk.toml` doesn't have `inject_scripts = false` |
| Camera not looking at target | Remove `orientation` prop (let Cesium default) |
| SSR warnings "unused variable" | Add `let _ = (prop1, prop2);` in `#[cfg(not(wasm32))]` block |
| Signal not updating | Use `.get()` not `.get_untracked()` for reactive props |
| JsSignal in SSR | JsSignal uses LocalStorage, only works in WASM |

## Validation Commands

Use these commands when changing integrations/components:

```bash
# Workspace compile
cargo check --workspace

# Native unit tests
cargo test -p leptos-cesium --lib

# Wasm test compilation (harness wiring)
cargo test -p leptos-cesium --lib --target wasm32-unknown-unknown --no-run
```

Example build checks:

```bash
for d in \
  examples/simple-viewer \
  examples/with-entities \
  examples/czml-viewer \
  examples/geojson \
  examples/custom-selection \
  examples/google-3d-tiles
do
  (cd "$d" && trunk build)
done

cargo check --manifest-path examples/with-server/Cargo.toml --features ssr
cargo check --manifest-path examples/with-server/Cargo.toml --features hydrate
```

## Example Coverage (Current)

- Runnable browser examples:
  - `examples/simple-viewer`
  - `examples/with-entities`
  - `examples/czml-viewer`
  - `examples/geojson`
  - `examples/custom-selection`
  - `examples/google-3d-tiles`
- Server example:
  - `examples/with-server` (`cargo leptos watch`)
- `examples/camera-control` is currently a scaffold directory (docs/public only).

## API Reference

See [references/components.md](references/components.md) for the current component/event prop reference.
