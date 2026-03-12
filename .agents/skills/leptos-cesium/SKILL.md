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
- Media surface: `BillboardGraphics`, `ImageMaterialPropertyBuilder`, `Material::image`, `MediaSource`
- CZML media support: `CzmlDataSource` auto-bridges flattened `properties.media_*` custom fields into billboard/rectangle/polygon media
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
│   ├── BillboardGraphics
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

// Image material (URL/data URL/HTML media)
Material::image(
    ImageMaterialPropertyBuilder::new()
        .image(MediaSource::Url("https://example.com/texture.png".to_string()))
        .build()
)
```

## Billboard + Media

Use `BillboardGraphics` for pinned image/media markers:

```rust
<Entity
    name="Pin".to_string()
    position=DVec3::new(-122.4786, 37.8194, 25.0)
>
    <BillboardGraphics
        image=Some(MediaSource::Url("pin.svg".to_string()))
        scale=Some(0.25)
        vertical_origin=Some(VerticalOrigin::Bottom)
    />
</Entity>
```

## Data Sources

### CZML

```rust
<CzmlDataSource
    url="satellite.czml"
    clear_existing=true  // Remove previous data sources
/>
```

### CZML Live Streaming Pattern

Use two datasource layers:

- Static layer (`Replace`/default): route geometry, pickup/dropoff markers, non-changing entities.
- Dynamic layer (`Append`): high-frequency telemetry updates for moving entities.

```rust
let static_packet = Some(build_static_packet_json());
let (delta_packet, set_delta_packet) = signal(Some(build_bootstrap_dynamic_packet_json()));
let (delta_trigger, set_delta_trigger) = signal(());
let (dynamic_mode, set_dynamic_mode) = signal(CzmlLoadMode::Replace);

view! {
    <ViewerContainer ion_token=token>
        // Static geometry loaded once.
        <CzmlDataSource
            data=static_packet
            clear_existing=false
            on_loaded=on_static_loaded
        />

        // Dynamic telemetry stream (bootstrap replace, then append deltas).
        <CzmlDataSource
            data=delta_packet
            mode=dynamic_mode
            clear_existing=false
            trigger=delta_trigger
            on_loaded=on_dynamic_loaded
            on_error=on_error
        />
    </ViewerContainer>
}

// On each telemetry tick:
set_dynamic_mode.set(CzmlLoadMode::Append);
set_delta_packet.set(Some(next_delta_json));
set_delta_trigger.set(());
```

Cesium does the interpolation automatically once enough samples are loaded. First-pass motion can still look stepped if the clock reaches the stream frontier before future samples are processed. For multipart/append CZML:

- preload future packets ahead of `viewer.clock.currentTime` like Cesium's own `Multi-part CZML` Sandcastle
- keep a persistent `CzmlDataSource` and append with `process()`
- repeat interpolation metadata on streamed sampled properties when emitting separate CZML parts for parity with Cesium examples:
  - `interpolationAlgorithm`
  - `interpolationDegree`
  - `forwardExtrapolationType`
  - `forwardExtrapolationDuration`

### Delta Trail Without Flicker

For moving trails, prefer sampled position deltas + `path` on the entity instead of replacing polyline positions every update.

- Bootstrap packet (replace): define entity `path` and initial `position` with `epoch`.
- Delta packet (append): send one new sampled tuple `[t, lon, lat, h]` for the same entity id.
- Keep static route polyline in the static datasource.

This avoids whole-polyline redraw flicker and reduces per-tick payload.

### Focus Wiring Parity

For Cesium-parity app flow, wire focus to `on_loaded` output:

```rust
let target = JsRwSignal::new_local(None::<ViewerTarget>);
let (focus_trigger, set_focus_trigger) = signal(());

let on_loaded = Callback::new(move |value: JsValue| {
    target.set(Some(ViewerTarget::from(value)));
    set_focus_trigger.set(());
});

view! {
    <CzmlDataSource on_loaded=on_loaded />
    <ViewerZoomToTarget trigger=focus_trigger target=target />
}
```

### Trigger Semantics

Treat trigger-driven components as edge-triggered actions.

- Trigger updates should execute even when payload/target is unchanged.
- Use trigger ticks for repeated actions against same target or same delta packet identity.
- Avoid relying on value changes alone for imperative Cesium actions.

### CZML Media Through `CzmlDataSource`

Use `CzmlDataSource` directly for CZML-driven image/video binding.

- Encode media intent with flattened custom properties such as `media_uri`, `media_kind`, and `media_target`.
- Do not rely on nested `properties.media = { ... }` objects for media metadata. In Cesium CZML parsing, objects containing typed keys like `uri` can be coerced into specialized property types, which loses the rest of the nested object shape.

```rust
let on_media_error = Callback::new(move |error: CzmlMediaError| {
    logging::error!("media error: {}", error.reason);
});

view! {
    <CzmlDataSource
        data=packet
        mode=packet_mode
        clear_existing=false
        on_media_error=on_media_error
    />
}
```

If a video rectangle is blank with no console error, inspect:

- `entity.rectangle.material.getValue(viewer.clock.currentTime).image`
- `HTMLVideoElement` means the bridge created a video texture correctly
- a string URL means the metadata was parsed as image/URI data instead of video data

For non-CZML media, prefer direct graphics/material APIs (`BillboardGraphics`, `RectangleGraphics` + `Material::image`) instead of CZML media plumbing.

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
| CZML line/path flickers in streaming | Do not resend full polyline geometry each tick; use sampled `position` deltas + `path` and keep static geometry separate |
| CZML video rectangle blank with no error | Inspect `rectangle.material.getValue(...).image`; if it is a string URL, flatten media metadata to `properties.media_*` fields |

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
  examples/czml-streaming \
  examples/pinned-image \
  examples/pinned-video-material \
  examples/czml-media-bridge \
  examples/geojson \
  examples/custom-selection \
  examples/google-3d-tiles \
  examples/camera-control
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
  - `examples/czml-streaming`
  - `examples/pinned-image`
  - `examples/pinned-video-material`
  - `examples/czml-media-bridge`
  - `examples/geojson`
  - `examples/custom-selection`
  - `examples/google-3d-tiles`
  - `examples/camera-control`
- Server example:
  - `examples/with-server` (`cargo leptos watch`)

## API Reference

See [references/components.md](references/components.md) for the current component/event prop reference.
