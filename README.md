# leptos-cesium (WIP)

`leptos-cesium` will provide a CesiumJS component library for the [Leptos](https://github.com/leptos-rs/leptos) framework. The goal is to mirror the ergonomics of `leptos-leaflet` while exposing Cesium concepts (viewer, entities, data sources, events) through idiomatic Leptos components.

## Repository Layout

- `leptos-cesium/` – main library crate (bindings, components, core utilities)
- `examples/` – example Leptos apps showcasing Cesium usage (WIP)
- `vendor/Cesium/<version>/` – canonical location for downloaded Cesium bundles (populated via the helper script)
- `scripts/` – utility scripts (`sync_cesium_assets.sh`)

## Getting Started

### 1. Install prerequisites

- Rust toolchain with the `wasm32-unknown-unknown` target
- `trunk` CLI (`cargo install trunk`)

### 2. Configure your Cesium Ion token

Copy the template and add your token:

```bash
cp .env.example .env.local
# Edit .env.local and paste your token
```

Get your free token from: https://ion.cesium.com/tokens

### 3. Install Cesium vendor assets

Download and sync Cesium assets to examples:

```bash
# Download Cesium-1.135.zip from https://cesium.com/downloads/
# Extract to project root, then:
./scripts/sync_cesium_assets.sh
```

The script expects Cesium at `vendor/Cesium/1.135/Build/Cesium` and creates symlinks in each example's `public/Cesium` directory.

### 4. Run an example

```bash
cd examples/simple-viewer
trunk serve --open
```

### What happens at build time?

1. **Environment loading**: Cargo reads `CESIUM_ION_TOKEN` from `.env.local` at build time
2. **Token passing**: Token is passed to `<ViewerContainer ion_token=... />` component prop
3. **Asset copying**: Trunk mirrors `public/Cesium/` into dist directory via `copy-dir` directive
4. **Cesium loading**: HTML loads `Cesium.js` synchronously in `<head>`
5. **WASM loading**: Trunk injects the WASM module at the `<link data-trunk rel="rust">` location
6. **Viewer creation**: Component sets `Cesium.Ion.defaultAccessToken` and creates viewer instance

### Development Tips

- Run `cargo check` or `cargo test` from the repository root
- When updating the Cesium bundle, rerun `./scripts/sync_cesium_assets.sh` and restart Trunk
- If you rotate Ion tokens, edit `.env.local` and rebuild
- For troubleshooting, see `CLAUDE.md`

## Project Status

- The `simple-viewer` example renders a Cesium globe, sets the viewer context, and adds a sample entity.
- Core library work is ongoing (bindings, components, events). Contributions are welcome!
