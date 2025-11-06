# CLAUDE.md

## Context

This repository houses `leptos-cesium`, an experimental Leptos component library targeting CesiumJS. The goal is to match the ergonomics of `leptos-leaflet` while leveraging auto-generated bindings where possible.

## Local Tooling

- Run `cargo fmt` and `cargo clippy --all-targets --all-features` before submitting patches.
- Use `cargo test --target wasm32-unknown-unknown` to run the `wasm-bindgen-test` smoke suite (once populated).
- Prefer `cargo leptos watch`/`build` inside example directories. Each example's `Cargo.toml` will expose `package.metadata.leptos.assets-dir = "public"` so the Cesium runtime is served automatically.

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
