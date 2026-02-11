# leptos-cesium Component Reference

This reference prioritizes current, high-use props and integration patterns.

Ergonomics note: for many `#[prop(optional, into)]` props, pass direct values (for example `name="Red Box".to_string()`, `position=DVec3::new(...)`) instead of explicit `Some(...)` to match this repository's preferred example style.

## ViewerContainer

Root component that creates the Cesium Viewer.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `ion_token` | `Signal<Option<String>>` | `None` | Cesium Ion access token |
| `class` | `String` | `""` | CSS class |
| `style` | `String` | `""` | Inline styles |
| `node_ref` | `NodeRef<leptos::html::Div>` | fresh node ref | Optional handle to the container div |
| `animation` | `bool` | `true` | Show animation widget |
| `timeline` | `bool` | `true` | Show timeline widget |
| `base_layer_picker` | `bool` | `true` | Show base layer picker |
| `home_button` | `bool` | `true` | Show home button |
| `scene_mode_picker` | `bool` | `true` | Show scene mode picker |
| `navigation_help_button` | `bool` | `true` | Show navigation help |
| `fullscreen_button` | `bool` | `true` | Show fullscreen button |
| `info_box` | `bool` | `true` | Show info box on selection |
| `selection_indicator` | `bool` | `true` | Show selection indicator |
| `should_animate` | `bool` | `true` | Auto-play animations |
| `globe` | `Signal<bool>` | `true` | Show/hide globe |

## Entity

Container for graphics components.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `name` | `Signal<Option<String>>` | `None` | Entity name |
| `position` | `Signal<Option<DVec3>>` | `None` | Position (lon, lat, height) |
| `description` | `Signal<Option<String>>` | `None` | Description for info box |
| `show` | `Signal<Option<bool>>` | `None` | Visibility |

## Graphics Components

### PointGraphics

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `pixel_size` | `Signal<f64>` | required | Point size in pixels |
| `color` | `Signal<Option<Srgba<f32>>>` | `None` | Point color |
| `outline` | `Signal<Option<bool>>` | `None` | Show outline |
| `outline_color` | `Signal<Option<Srgba<f32>>>` | `None` | Outline color |
| `outline_width` | `Signal<Option<f64>>` | `None` | Outline width |
| `show` | `Signal<Option<bool>>` | `None` | Point visibility |
| `height_reference` | `Signal<Option<f64>>` | `None` | Height reference enum value |
| `disable_depth_test_distance` | `Signal<Option<f64>>` | `None` | Disable depth test within distance |

### RectangleGraphics

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `coordinates` | `Signal<Rect<f64>>` | required | Rectangle bounds |
| `material` | `JsSignal<Option<Material>>` | `None` | Fill material |
| `outline` | `Signal<Option<bool>>` | `None` | Show outline |
| `outline_color` | `Signal<Option<Srgba<f32>>>` | `None` | Outline color |
| `outline_width` | `Signal<Option<f64>>` | `None` | Outline width |
| `height` | `Signal<Option<f64>>` | `None` | Height above ground |
| `extruded_height` | `Signal<Option<f64>>` | `None` | Extrusion height |
| `rotation` | `Signal<Option<f64>>` | `None` | Rotation in radians |
| `st_rotation` | `Signal<Option<f64>>` | `None` | Texture rotation |

### PolygonGraphics

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `hierarchy` | `Signal<Polygon<f64>>` | required | Polygon with holes |
| `material` | `JsSignal<Option<Material>>` | `None` | Fill material |
| `outline` | `Signal<Option<bool>>` | `None` | Show outline |
| `outline_color` | `Signal<Option<Srgba<f32>>>` | `None` | Outline color |
| `outline_width` | `Signal<Option<f64>>` | `None` | Outline width |
| `height` | `Signal<Option<f64>>` | `None` | Height above ground |
| `extruded_height` | `Signal<Option<f64>>` | `None` | Extrusion height |

### PolylineGraphics

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `positions` | `Signal<LineString<f64>>` | required | Line positions |
| `width` | `Signal<f64>` | required | Line width in pixels |
| `material` | `JsSignal<Option<Material>>` | `None` | Line material |
| `clamp_to_ground` | `Signal<Option<bool>>` | `None` | Clamp to terrain |
| `show` | `Signal<Option<bool>>` | `None` | Polyline visibility |
| `granularity` | `Signal<Option<f64>>` | `None` | Sampling granularity |
| `follow_surface` | `Signal<Option<bool>>` | `None` | Follow ellipsoid surface |
| `depth_fail_material` | `Signal<Option<Srgba<f32>>>` | `None` | Color used when depth testing fails |

### EllipseGraphics

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `semi_major_axis` | `Signal<f64>` | required | Major axis in meters |
| `semi_minor_axis` | `Signal<f64>` | required | Minor axis in meters |
| `material` | `JsSignal<Option<Material>>` | `None` | Fill material |
| `outline` | `Signal<Option<bool>>` | `None` | Show outline |
| `outline_color` | `Signal<Option<Srgba<f32>>>` | `None` | Outline color |
| `outline_width` | `Signal<Option<f64>>` | `None` | Outline width |
| `rotation` | `Signal<Option<f64>>` | `None` | Rotation in radians |
| `st_rotation` | `Signal<Option<f64>>` | `None` | Texture rotation in radians |
| `extruded_height` | `Signal<Option<f64>>` | `None` | Extrusion height |
| `height` | `Signal<Option<f64>>` | `None` | Height above ground |

### BoxGraphics

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `dimensions` | `Signal<DVec3>` | required | Box dimensions (x, y, z) |
| `material` | `JsSignal<Option<Material>>` | `None` | Fill material |
| `outline` | `Signal<Option<bool>>` | `None` | Show outline |
| `outline_color` | `Signal<Option<Srgba<f32>>>` | `None` | Outline color |
| `outline_width` | `Signal<Option<f64>>` | `None` | Outline width |
| `fill` | `Signal<Option<bool>>` | `None` | Whether box is filled |
| `show` | `Signal<Option<bool>>` | `None` | Box visibility |

### EllipsoidGraphics

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `radii` | `Signal<DVec3>` | required | Radii (x, y, z) |
| `material` | `JsSignal<Option<Material>>` | `None` | Fill material |
| `outline` | `Signal<Option<bool>>` | `None` | Show outline |
| `outline_color` | `Signal<Option<Srgba<f32>>>` | `None` | Outline color |
| `outline_width` | `Signal<Option<f64>>` | `None` | Outline width |
| `fill` | `Signal<Option<bool>>` | `None` | Whether ellipsoid is filled |
| `show` | `Signal<Option<bool>>` | `None` | Ellipsoid visibility |
| `stack_partitions` | `Signal<Option<f64>>` | `None` | Number of vertical outline lines |
| `slice_partitions` | `Signal<Option<f64>>` | `None` | Number of horizontal outline lines |
| `subdivision_divisions` | `Signal<Option<f64>>` | `None` | Subdivision density for outline rings |

### CylinderGraphics

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `length` | `Signal<f64>` | required | Cylinder length |
| `top_radius` | `Signal<f64>` | required | Top radius |
| `bottom_radius` | `Signal<f64>` | required | Bottom radius |
| `material` | `JsSignal<Option<Material>>` | `None` | Fill material |
| `outline` | `Signal<Option<bool>>` | `None` | Show outline |
| `outline_color` | `Signal<Option<Srgba<f32>>>` | `None` | Outline color |
| `outline_width` | `Signal<Option<f64>>` | `None` | Outline width |
| `fill` | `Signal<Option<bool>>` | `None` | Whether cylinder is filled |
| `show` | `Signal<Option<bool>>` | `None` | Cylinder visibility |
| `number_of_vertical_lines` | `Signal<Option<f64>>` | `None` | Vertical outline lines |
| `slices` | `Signal<Option<f64>>` | `None` | Perimeter edge count |

### WallGraphics

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `positions` | `Signal<LineString<f64>>` | required | Wall base positions |
| `maximum_heights` | `Signal<Option<Vec<f64>>>` | `None` | Top heights |
| `minimum_heights` | `Signal<Option<Vec<f64>>>` | `None` | Bottom heights |
| `material` | `JsSignal<Option<Material>>` | `None` | Fill material |
| `outline` | `Signal<Option<bool>>` | `None` | Show outline |
| `outline_color` | `Signal<Option<Srgba<f32>>>` | `None` | Outline color |
| `outline_width` | `Signal<Option<f64>>` | `None` | Outline width |
| `fill` | `Signal<Option<bool>>` | `None` | Whether wall is filled |
| `show` | `Signal<Option<bool>>` | `None` | Wall visibility |
| `granularity` | `Signal<Option<f64>>` | `None` | Sampling granularity |

### CorridorGraphics

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `positions` | `Signal<LineString<f64>>` | required | Corridor centerline |
| `width` | `Signal<f64>` | required | Corridor width |
| `material` | `JsSignal<Option<Material>>` | `None` | Fill material |
| `outline` | `Signal<Option<bool>>` | `None` | Show outline |
| `outline_color` | `Signal<Option<Srgba<f32>>>` | `None` | Outline color |
| `outline_width` | `Signal<Option<f64>>` | `None` | Outline width |
| `height` | `Signal<Option<f64>>` | `None` | Height above ground |
| `extruded_height` | `Signal<Option<f64>>` | `None` | Extrusion height |
| `corner_type` | `Signal<Option<f64>>` | `None` | Corner style enum value (Cesium numeric constant) |
| `granularity` | `Signal<Option<f64>>` | `None` | Sampling granularity |
| `fill` | `Signal<Option<bool>>` | `None` | Whether corridor is filled |
| `show` | `Signal<Option<bool>>` | `None` | Corridor visibility |

### PolylineVolumeGraphics

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `positions` | `Signal<LineString<f64>>` | required | Volume centerline |
| `shape` | `Signal<Vec<DVec2>>` | required | 2D cross-section shape |
| `material` | `JsSignal<Option<Material>>` | `None` | Fill material |
| `outline` | `Signal<Option<bool>>` | `None` | Show outline |
| `outline_color` | `Signal<Option<Srgba<f32>>>` | `None` | Outline color |
| `outline_width` | `Signal<Option<f64>>` | `None` | Outline width |
| `fill` | `Signal<Option<bool>>` | `None` | Whether volume is filled |
| `show` | `Signal<Option<bool>>` | `None` | Volume visibility |
| `granularity` | `Signal<Option<f64>>` | `None` | Sampling granularity |
| `corner_type` | `Signal<Option<f64>>` | `None` | Corner style enum value (Cesium numeric constant) |

## Camera Components

### CameraSetView

Instant camera positioning.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `destination` | `Signal<DVec3>` | required | Position (lon, lat, height) |
| `orientation` | `Signal<Option<(f64, f64, f64)>>` | `None` | (heading, pitch, roll) radians |

### CameraFlyTo

Animated camera flight.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `destination` | `Signal<DVec3>` | required | Target position |
| `orientation` | `Signal<Option<(f64, f64, f64)>>` | `None` | (heading, pitch, roll) radians |
| `duration` | `Signal<f64>` | `3.0` | Flight duration in seconds |
| `offset` | `Signal<Option<(f64, f64, f64)>>` | `None` | (heading, pitch, range) offset |

### CameraFlyHome

Fly to home view.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `trigger` | `Signal<()>` | required | Trigger signal |
| `duration` | `Signal<f64>` | `0.0` | Flight duration |

### CameraFlyToBoundingSphere

Fly to fit a bounding sphere.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `target` | `JsSignal<BoundingSphere>` | required | Target sphere |
| `offset` | `JsSignal<Option<HeadingPitchRange>>` | `None` | Camera offset |
| `duration` | `Signal<f64>` | `3.0` | Flight duration |

### ClockReset

Reset viewer clock.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `trigger` | `Signal<()>` | required | Trigger signal |

## Data Sources

### CzmlDataSource

Load CZML data.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `url` | `Signal<String>` | required | CZML file URL |
| `clear_existing` | `Signal<bool>` | `true` | Eagerly remove this component's previously loaded source before re-load |

### GeoJsonDataSource

Load GeoJSON/TopoJSON data.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `url` | `Signal<String>` | required | GeoJSON file URL |
| `clear_existing` | `Signal<bool>` | `true` | Eagerly remove this component's previously loaded source before re-load |
| `stroke` | `JsSignal<Option<Color>>` | `None` | Line stroke color |
| `stroke_width` | `Signal<Option<f64>>` | `None` | Line stroke width |
| `fill` | `JsSignal<Option<Color>>` | `None` | Polygon fill color |
| `marker_color` | `JsSignal<Option<Color>>` | `None` | Point marker color |
| `marker_size` | `Signal<Option<f64>>` | `None` | Point marker size |
| `marker_symbol` | `Signal<Option<String>>` | `None` | Marker symbol (Maki id or single char) |
| `clamp_to_ground` | `Signal<Option<bool>>` | `None` | Clamp to terrain |
| `credit` | `Signal<Option<String>>` | `None` | Data attribution string |

## 3D Tiles

### GooglePhotorealistic3DTiles

Load Google's 3D tiles.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `google_api_key` | `Option<Signal<Option<String>>>` | `None` | Optional Google Maps API key signal (uses Ion fallback if not provided) |
| `cache_bytes` | `Option<u32>` | `None` | Tile cache size in bytes |
| `maximum_cache_overflow_bytes` | `Option<u32>` | `None` | Overflow cache size in bytes |
| `enable_collision` | `Option<bool>` | `None` | Enable camera collision |

## Event Builders

### ViewerEvents

Generated by `cesium_events!` and re-exported from `leptos_cesium::prelude::*`.

| Method | Payload Type | Description |
|--------|--------------|-------------|
| `set_selected_entity_changed` | `JsValue` | Handler for selected entity changes |
| `set_tracked_entity_changed` | `JsValue` | Handler for tracked entity changes |

Lifecycle methods:

- `setup(&viewer)` attaches configured handlers
- `teardown()` detaches listeners attached by this builder

## Material Types

### Color Material

```rust
Material::color(Color::red().with_alpha(0.5))
```

### Stripe Material

```rust
Material::stripe(
    StripeOptions::new()
        .even_color(Color::white())
        .odd_color(Color::blue())
        .repeat(5.0)
        .build()
)
```

### Checkerboard Material

```rust
Material::checkerboard(
    CheckerboardOptions::new()
        .even_color(Color::white())
        .odd_color(Color::black())
        .repeat(Cartesian2::new(20.0, 6.0))
        .build()
)
```

### Polyline Glow Material

```rust
Material::polyline_glow(
    PolylineGlowOptions::new()
        .color(Color::deepskyblue())
        .glow_power(0.25)
        .taper_power(1.0)
        .build()
)
```

## Color Constants

Available via `Color::method()`:

- `Color::white()`, `Color::black()`
- `Color::red()`, `Color::green()`, `Color::blue()`
- `Color::yellow()`, `Color::cyan()`, `Color::magenta()`
- `Color::orange()`, `Color::pink()`, `Color::purple()`
- `Color::deepskyblue()`, `Color::gold()`, `Color::lime()`

Modify with `.with_alpha(f64)` for transparency.
