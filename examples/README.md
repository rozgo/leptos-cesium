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
  examples/czml-streaming \
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
