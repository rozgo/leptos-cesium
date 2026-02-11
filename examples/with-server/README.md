# With Server (SSR + Hydration) Example

This example demonstrates `leptos-cesium` in a full Leptos server setup with:

- server-side rendering (`ssr` feature)
- client hydration (`hydrate` feature)
- routing via `leptos_router`
- Cesium viewer rendering through `ViewerContainer`

## Features Demonstrated

- Axum + Leptos integration (`main.rs`)
- Shared app shell and route rendering (`app.rs`)
- Hydration entrypoint for browser startup (`lib.rs`)
- Cesium CDN script/style integration in SSR HTML shell

## Prerequisites

1. Configure a Cesium Ion token in `.env.local` at the repository root:

```bash
cp .env.example .env.local
# Edit .env.local and set CESIUM_ION_TOKEN=your_token_here
```

2. Install `cargo-leptos`:

```bash
cargo install cargo-leptos
```

## Run (Recommended)

```bash
cd examples/with-server
cargo leptos watch
```

Then open: `http://127.0.0.1:3000`

## Build Checks

```bash
cargo check --manifest-path examples/with-server/Cargo.toml --features ssr
cargo check --manifest-path examples/with-server/Cargo.toml --features hydrate
```

## Architecture Notes

- `src/main.rs`
  - SSR server entrypoint (Axum + Leptos routes) behind `feature = "ssr"`
- `src/lib.rs`
  - hydration entrypoint (`hydrate()`) behind `feature = "hydrate"`
- `src/app.rs`
  - shared app UI, shell HTML, Cesium asset tags, and route definitions

## What You Should See

- A server-rendered page that hydrates in-browser
- A Cesium viewer filling the page with animation/timeline disabled
- Stable SSR-to-hydration handoff using the same component tree
