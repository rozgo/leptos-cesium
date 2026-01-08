---
name: leptos-cesium
description: CesiumJS integration for Leptos 0.8.x. Provides declarative Cesium components (viewer, entities, graphics, camera, data sources) using standard Rust types (glam, geo-types, palette). Use when working with leptos-cesium crate, editing Cesium components, or adding 3D globe/map functionality to a Leptos app. For general Leptos patterns, see the leptos skill.
---

# leptos-cesium Skill

Declarative CesiumJS components for Leptos. Uses standard Rust types (glam, geo-types, palette) for SSR compatibility.

**For general Leptos patterns** (signals, components, SSR setup, routing): see the `leptos` skill.

## Quick Start

```rust
use leptos::prelude::*;
use leptos_cesium::prelude::*;
use geo_types::coord;

#[component]
fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());

    view! {
        <ViewerContainer ion_token=ion_token style="width: 100%; height: 100%;".to_string()>
            <Entity name="My Entity" position=Some(DVec3::new(-74.0, 40.7, 100.0))>
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
    name="Point of Interest"
    position=Some(DVec3::new(-74.0445, 40.6892, 150.0))  // (lon, lat, height)
    description="The Statue of Liberty"
>
    <PointGraphics pixel_size=20.0 color=Some(Srgba::new(0.0, 1.0, 0.0, 1.0)) />
</Entity>
```

### Rectangle (geo_types::Rect)

```rust
use geo_types::coord;

<Entity name="Rectangle">
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
<Entity name="Polygon">
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
<Entity name="Polyline">
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
    clamp_to_ground=Some(false)
/>
```

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
            let mat = material.get_untracked();  // JsSignal uses get_untracked()
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
- `JsSignal<T>` for JS types → use `.get_untracked()`
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

## API Reference

See [references/components.md](references/components.md) for full component prop reference.
