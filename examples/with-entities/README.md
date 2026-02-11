# Entities Showcase Example

This example demonstrates declarative entity composition in `leptos-cesium` using standard Rust geometry/color types.

## Features Demonstrated

- Multiple entity and graphics components rendered together
- 2D graphics:
  - `RectangleGraphics`
  - `PolygonGraphics`
  - `EllipseGraphics`
- 3D graphics:
  - `BoxGraphics`
  - `EllipsoidGraphics`
  - `CylinderGraphics`
- Path and surface primitives:
  - `WallGraphics`
  - `CorridorGraphics`
  - `PolylineGraphics`
- Materials:
  - Solid color
  - Stripe
  - Checkerboard
  - Polyline glow
- Use of Rust-native types:
  - `geo_types::{Rect, Polygon, LineString}`
  - `glam::DVec3`
  - `palette::Srgba`

## Prerequisites

1. Configure a Cesium Ion token in `.env.local` at the repository root:

```bash
cp .env.example .env.local
# Edit .env.local and set CESIUM_ION_TOKEN=your_token_here
```

## Run

```bash
cd examples/with-entities
trunk serve --open
```

## Build Check

```bash
cd examples/with-entities
trunk build
```

## What You Should See

- A scene containing several entities with different geometry/material combinations:
  - colored rectangle and polygon
  - rotated ellipse
  - 3D box/sphere/cylinder forms
  - checkerboard wall
  - magenta corridor
  - glowing polyline

## Code Highlights

```rust
<Entity name=Some("Blue Polygon".to_string())>
    <PolygonGraphics
        hierarchy=Polygon::new(
            LineString::new(vec![
                coord! { x: -115.0, y: 37.0 },
                coord! { x: -115.0, y: 32.0 },
                coord! { x: -107.0, y: 33.0 },
                coord! { x: -102.0, y: 31.0 },
                coord! { x: -102.0, y: 35.0 },
                coord! { x: -115.0, y: 37.0 },
            ]),
            vec![]
        )
        material=Some(Material::color(Color::blue().with_alpha(0.5)))
        outline=Some(true)
        outline_color=Some(Srgba::new(1.0, 1.0, 1.0, 1.0))
    />
</Entity>
```

## Notes

- This example is focused on graphics breadth, not camera scripting or data-source loading.
- Cesium JS/CSS assets are loaded from CDN in `index.html`.
