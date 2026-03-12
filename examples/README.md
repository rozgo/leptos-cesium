# Examples

Example applications live in the subdirectories of this folder.

Cesium JS/CSS assets are loaded from the Cesium CDN by each example's `index.html`, so no local `public/Cesium` sync step is required.

## Example Catalog

- `simple-viewer`: minimal globe with `ViewerContainer`.
- `with-entities`: declarative entity/graphics/material coverage.
- `czml-viewer`: baseline CZML loading + camera focus behavior.
- `czml-streaming`: append-style CZML deltas for moving entities.
- `pinned-image`: non-CZML billboard image pin via `BillboardGraphics`.
- `pinned-video-material`: non-CZML video texture on rectangle material.
- `czml-media-bridge`: automatic CZML `properties.media_*` media binding + append streaming.
- `geojson`: GeoJSON loading and styling.
- `custom-selection`: custom Leptos selection panel replacing default InfoBox.
- `google-3d-tiles`: Google photorealistic 3D tiles.
- `camera-control`: camera action/control parity surface.
- `with-server`: SSR/hydrate example.

Run a specific example:

```bash
cd examples/simple-viewer
trunk serve --open
```

Validate all browser examples compile:

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
```

Validate the SSR example:

```bash
cargo check --manifest-path examples/with-server/Cargo.toml --features ssr
cargo check --manifest-path examples/with-server/Cargo.toml --features hydrate
```
