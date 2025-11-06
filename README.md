# leptos-cesium (WIP)

`leptos-cesium` will provide a CesiumJS component library for the [Leptos](https://github.com/leptos-rs/leptos) framework. The goal is to mirror the ergonomics of `leptos-leaflet` while exposing Cesium concepts (viewer, entities, data sources, events) through idiomatic Leptos components.

## Repository Layout

- `leptos-cesium/` – main library crate (bindings, components, core utilities)
- `examples/` – example Leptos apps showcasing Cesium usage (WIP)
- `vendor/Cesium/<version>/` – canonical location for downloaded Cesium bundles (populated via the helper script)
- `scripts/` – utility scripts (`sync_cesium_assets.sh`)

## Getting Started

Most day-to-day development happens in the `examples/simple-viewer` crate. The example is built and served with [Trunk](https://trunkrs.dev) so that the Cesium assets, WASM bundle, and environment variables are wired together automatically.

### 1. Install prerequisites

- Rust toolchain with the `wasm32-unknown-unknown` target
- `trunk` CLI (`cargo install trunk`)
- A Cesium bundle extracted somewhere on disk (see below)

### 2. Vendor Cesium

Download the official Cesium archive (`Cesium-<version>.zip`) and use the helper script to place it under `public/Cesium` for every example:

```bash
./scripts/sync_cesium_assets.sh            # defaults to version 1.135
./scripts/sync_cesium_assets.sh 1.136      # sync a different version
```

The script looks for `Cesium-<version>/Build/Cesium` in the repository root. If it exists locally it is copied into `vendor/Cesium/<version>/Build/Cesium` and then linked into each example’s `public/Cesium`.

### 3. Provide an Ion token

Trunk run hooks read an `.env.local` file before each build and emit a small JavaScript shim consumed by the bootstrap script. Create your own copy (this file stays out of git):

```bash
cp examples/simple-viewer/.env.example examples/simple-viewer/.env.local
```

Edit `.env.local` and paste a valid Cesium Ion token:

```
CESIUM_ION_TOKEN=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### 4. Run the example

```bash
cd examples/simple-viewer
trunk serve --open
```

Trunk’s `pre_build` hook (`scripts/generate_cesium_env.js`) reads the token and generates `public/cesium-env.js`. Both the public folder and the generated staging/dist directories receive the same file so the watch loop remains stable. The HTML page loads this script before the WASM bootstrap so `window.CESIUM_ION_TOKEN` is defined before the viewer starts.

### What happens at build time?

1. **Pre-build hook**: `scripts/generate_cesium_env.js` reads `.env.local` and generates `cesium-env.js` with the Ion token
2. **Asset copying**: `copy-dir` mirrors `public/Cesium/**` into Trunk's dist directory
3. **Cesium loading**: The HTML loads `cesium-env.js` and `Cesium.js` synchronously in `<head>`
4. **Token setup**: Inline script sets `Cesium.Ion.defaultAccessToken` from `window.CESIUM_ION_TOKEN`
5. **WASM loading**: Trunk injects the WASM module loader at the `<link data-trunk rel="rust">` location
6. **App mounting**: `main()` calls `mount_to_body()` which creates the Leptos app and Cesium viewer

### Development Tips

- Run `cargo check` or `cargo test` from the repository root as usual; the example crate depends directly on the workspace library.
- When updating the Cesium bundle, rerun `./scripts/sync_cesium_assets.sh` and restart Trunk so the `copy-dir` asset picks up the new files.
- If you rotate Ion tokens, edit `.env.local`; the build hook rewrites `cesium-env.js` on the next build.
- For troubleshooting build/runtime issues, see `CLAUDE.md`

## Project Status

- The `simple-viewer` example renders a Cesium globe, sets the viewer context, and adds a sample entity.
- Core library work is ongoing (bindings, components, events). Contributions are welcome!
