# leptos-cesium (WIP)

`leptos-cesium` will provide a CesiumJS component library for the [Leptos](https://github.com/leptos-rs/leptos) framework. The goal is to mirror the ergonomics of `leptos-leaflet` while exposing Cesium concepts (viewer, entities, data sources, events) through idiomatic Leptos components.

## Repository Layout

- `leptos-cesium/` – main library crate (bindings, components, core utilities)
- `examples/` – example Leptos apps showcasing Cesium usage (WIP)
- `vendor/Cesium/<version>/` – canonical location for downloaded Cesium bundles (populated via the helper script)
- `scripts/` – utility scripts (`sync_cesium_assets.sh`)

## Getting Started

The workspace currently compiles a placeholder crate while bindings and components are being implemented.

```bash
# Check formatting and linting
cargo fmt
cargo clippy --all-targets --all-features

# Build the library (no examples yet)
cargo build
```

### Managing Cesium Assets

Cesium requires its runtime assets (Workers, Assets, Widgets, etc.) to be served alongside your application. This repository keeps a single vendor copy per version and syncs it into each example's `public/Cesium` directory via a helper script:

```bash
./scripts/sync_cesium_assets.sh            # defaults to version 1.135
./scripts/sync_cesium_assets.sh 1.136      # sync a different version
```

The script will vendorize an existing `Cesium-<version>` directory if present locally. Otherwise, download the official Cesium archive and extract it so that `vendor/Cesium/<version>/Build/Cesium` exists before running the script.

Each example will set `package.metadata.leptos.assets-dir = "public"` so that `cargo leptos build`/`watch` serve the synced Cesium files automatically.

### Running Examples (soon)

Example applications are being scaffolded now. Once implemented you will be able to:

```bash
cd examples/simple-viewer
cargo leptos watch
```

Make sure `scripts/sync_cesium_assets.sh` has been executed beforehand so `public/Cesium` exists.

## Development Notes

- Stick to the Leptos conventions used in `leptos-leaflet` (contexts, SSR-safe signals, event builders).
- Bindings will be generated via `ts-bindgen` against `Cesium-1.135/Source/Cesium.d.ts` into `src/bindings/generated.rs`.
- `wasm-bindgen-test` smoke tests will validate bindings via `cargo test --target wasm32-unknown-unknown`.

Contributions are welcome once the base architecture settles.
