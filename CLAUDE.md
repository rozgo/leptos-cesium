# CLAUDE.md

## Context

This repository houses `leptos-cesium`, an experimental Leptos component library targeting CesiumJS. The goal is to match the ergonomics of `leptos-leaflet` while leveraging auto-generated bindings where possible.

## Local Tooling

- Run `cargo fmt` and `cargo clippy --all-targets --all-features` before submitting patches.
- Use `cargo test --target wasm32-unknown-unknown` to run the `wasm-bindgen-test` smoke suite (once populated).
- **Examples use Trunk, not cargo-leptos**: Run `trunk serve` from example directories (e.g., `examples/simple-viewer`).
- Trunk configuration is in `Trunk.toml` - **NEVER set `inject_scripts = false`** as this prevents WASM loading.

## Troubleshooting

### Black screen / viewer not rendering

1. **Check `Trunk.toml`**: Ensure it does NOT have `inject_scripts = false` (this prevents WASM from loading)
2. **Verify HTML structure**:
   - `index.html` must have `<link data-trunk rel="rust" data-bindgen-target="web">` in `<body>`
   - `<body>` should be empty (no `<div id="root">`) - Leptos `mount_to_body()` creates its own mount point
   - Cesium.js must load in `<head>` before WASM: `<script src="Cesium/Cesium.js"></script>`
3. **Check token setup**:
   - Verify `cesium-env.js` exists in dist and contains your token
   - Confirm inline script sets `Cesium.Ion.defaultAccessToken` after Cesium.js loads
4. **Verify load order** (check Network tab):
   - `cesium-env.js` → `Cesium.js` → WASM module → app initialization

### "No such file or directory" errors

- Run `./scripts/sync_cesium_assets.sh` to set up the Cesium assets
- Verify symlink at `examples/simple-viewer/public/Cesium` points to valid vendor directory
- Check that `vendor/Cesium/1.135/Build/Cesium` contains the Cesium.js file

### App container empty in DOM

1. **Check browser console** for Rust panics or JavaScript errors
2. **Verify WASM injection**: View page source, look for `<script type="module">import init`
3. **Ensure proper crate structure**:
   - Binary crate: `src/main.rs` with `main()` function
   - Library crate: `src/lib.rs` with `[lib]` section and `#[wasm_bindgen(start)]`
   - **Do NOT mix both** - remove `[lib]` from Cargo.toml if using binary crate

### Trunk not injecting WASM scripts

- Verify `<link data-trunk rel="rust">` is in `<body>`, not `<head>`
- Check `data-bindgen-target="web"` is set (defaults to `no-modules` which doesn't work with modern patterns)
- Ensure Cargo.toml doesn't have conflicting `crate-type` settings when using binary crate
- Check that the working directory is the example directory, not the workspace root

## Cesium Assets

Execute `./scripts/sync_cesium_assets.sh` after downloading the official Cesium release. The script keeps a single vendor copy under `vendor/Cesium/<version>/Build/Cesium` and symlinks (or copies) it into every example's `public/Cesium` directory.

## Implementation Guidance

1. Port core patterns from `leptos-leaflet`:
   - Thread-aware JsValue wrappers for SSR safety.
   - Context providers for the viewer, entities, and data sources.
   - `cesium_events!` macro mirroring the Leaflet events builder.
2. Generate bindings with `ts-bindgen`; manual shims should live beside the generated modules.
3. Default the viewer to a `CESIUM_BASE_URL` pointing at `/pkg/Cesium`, with overrides available via props or environment configuration.
4. Expand documentation as features land (README, example guides, API docs).

When in doubt, compare behavior and architecture decisions with `leptos-leaflet` and the upstream Leptos workspace for idiomatic practices.
