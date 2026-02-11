# Simple Viewer Example

This example demonstrates the smallest useful `leptos-cesium` setup: a viewer plus one point entity.

## Features Demonstrated

- `ViewerContainer` initialization with Cesium Ion token
- Minimal UI (`animation=false`, `timeline=false`)
- Declarative entity creation with `Entity` + `PointGraphics`
- Coordinate usage with `DVec3` (`longitude`, `latitude`, `height`)
- Color usage with `Srgba`

## Prerequisites

1. Configure a Cesium Ion token in `.env.local` at the repository root:

```bash
cp .env.example .env.local
# Edit .env.local and set CESIUM_ION_TOKEN=your_token_here
```

## Run

```bash
cd examples/simple-viewer
trunk serve --open
```

## Build Check

```bash
cd examples/simple-viewer
trunk build
```

## What You Should See

- A Cesium globe with a single red point near the Statue of Liberty
- No timeline or animation widget (intentionally disabled in this minimal setup)

## Code Highlights

```rust
<ViewerContainer
    ion_token=ion_token
    animation=false
    timeline=false
    style="width: 100%; height: 100%;".to_string()
>
    <Entity
        name=Some("Statue of Liberty".to_string())
        position=Some(DVec3::new(-74.0445, 40.6892, 150.0))
    >
        <PointGraphics
            pixel_size=12.0
            color=Some(Srgba::new(1.0, 0.0, 0.0, 1.0))
        />
    </Entity>
</ViewerContainer>
```

## Notes

- Cesium JS/CSS assets are loaded from the Cesium CDN via `index.html`.
- There is no local `public/Cesium` asset sync step in this repository.
