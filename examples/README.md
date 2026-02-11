# Examples

Example applications live in the subdirectories of this folder.

Cesium JS/CSS assets are loaded from the Cesium CDN by each example's `index.html`, so no local `public/Cesium` sync step is required.

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
  examples/geojson \
  examples/custom-selection \
  examples/google-3d-tiles
do
  (cd "$d" && trunk build)
done
```

`examples/camera-control` is currently a scaffold directory (docs/public only), so it is not included in the build loop above.

Validate the SSR example:

```bash
cargo check --manifest-path examples/with-server/Cargo.toml --features ssr
cargo check --manifest-path examples/with-server/Cargo.toml --features hydrate
```
