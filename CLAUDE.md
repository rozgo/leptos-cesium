# CLAUDE.md

## Context

This repository houses `leptos-cesium`, an experimental Leptos component library targeting CesiumJS. The goal is to match the ergonomics of `leptos-leaflet` while leveraging auto-generated bindings where possible.

## Local Tooling

- Run `cargo fmt` and `cargo clippy --all-targets --all-features` before submitting patches.
- Use `cargo test --target wasm32-unknown-unknown` to run the `wasm-bindgen-test` smoke suite (once populated).
- **Examples use Trunk**: Run `trunk serve` from example directories (e.g., `examples/simple-viewer`).
- Trunk configuration is in `Trunk.toml` - **NEVER set `inject_scripts = false`** as this prevents WASM loading.

## Environment Configuration

**Single `.env.local` at project root:**
- Copy `.env.example` to `.env.local` and add your Cesium Ion token
- Token is loaded at build time via `option_env!("CESIUM_ION_TOKEN")`
- Pass to component: `<ViewerContainer ion_token=token />`
- Never commit `.env.local` (gitignored)

## Cesium Ion Token Setup

The `ViewerContainer` component accepts an `ion_token` prop to set the Cesium Ion access token:

```rust
use leptos::prelude::*;
use leptos_cesium::components::ViewerContainer;

#[component]
pub fn App() -> impl IntoView {
    // Load token from environment at build time
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());

    view! {
        <ViewerContainer ion_token=ion_token>
            // your entities, data sources, etc.
        </ViewerContainer>
    }
}
```

**For workspace development:**
1. Create `.env.local` at project root with `CESIUM_ION_TOKEN=your_token`
2. Token is automatically picked up by `cargo` build

**For external projects using leptos-cesium:**
- Set `CESIUM_ION_TOKEN` environment variable before building
- Or pass token directly as a prop from any source (server, config file, etc.)

## Troubleshooting

### Black screen / viewer not rendering

1. **Check `Trunk.toml`**: Ensure it does NOT have `inject_scripts = false` (this prevents WASM from loading)
2. **Verify HTML structure**:
   - `index.html` must have `<link data-trunk rel="rust" data-bindgen-target="web">` in `<body>`
   - `<body>` should be empty (no `<div id="root">`) - Leptos `mount_to_body()` creates its own mount point
   - Cesium.js must load in `<head>` before WASM: `<script src="Cesium/Cesium.js"></script>`
3. **Check token setup**:
   - Verify `.env.local` exists at project root with valid `CESIUM_ION_TOKEN`
   - Confirm token is passed to `<ViewerContainer ion_token=... />`
4. **Verify load order** (check Network tab):
   - `Cesium.js` → WASM module → app initialization

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

## Cesium Vendor Installation

**Single vendor path:** `vendor/Cesium/<version>/Build/Cesium`

To install Cesium assets:
1. Download the official Cesium release from https://cesium.com/downloads/ (e.g., `Cesium-1.135.zip`)
2. Extract the zip file
3. Move `Cesium-<version>/Build` to `vendor/Cesium/<version>/`
4. Run `./scripts/sync_cesium_assets.sh` to symlink assets into all example directories

The sync script validates the Cesium build and creates symlinks (or copies if symlinks fail) from `vendor/Cesium/<version>/Build/Cesium` to each example's `public/Cesium` directory.

## Example Structure

Each example follows this structure:

```
examples/my-example/
├── Cargo.toml          # Binary crate with leptos-cesium dependency
├── Trunk.toml          # Build config (watch paths, static_dir)
├── index.html          # Load Cesium.js, widgets.css, trunk directives
├── src/
│   └── main.rs         # App component with ViewerContainer
└── public/
    └── Cesium/         # Symlink to vendor/Cesium/<version>/Build/Cesium
```

**Required HTML structure:**
```html
<head>
  <link rel="stylesheet" href="Cesium/Widgets/widgets.css" />
  <link data-trunk rel="copy-dir" href="public/Cesium" data-target-path="Cesium" />
  <script src="Cesium/Cesium.js"></script>
</head>
<body>
  <link data-trunk rel="rust" data-bindgen-target="web" />
</body>
```

**Required Trunk.toml:**
```toml
[build]
target = "index.html"
dist = "dist"
static_dir = "public"
filehash = false
watch = ["../../leptos-cesium"]  # Watch library for changes
```

**Minimal App:**
```rust
use leptos::prelude::*;
use leptos_cesium::components::ViewerContainer;

#[component]
pub fn App() -> impl IntoView {
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());

    view! {
        <ViewerContainer
            ion_token=ion_token
            style="width: 100%; height: 100%;".to_string()
        >
            // Add entities, data sources, etc. here
        </ViewerContainer>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> });
}
```

## Adding leptos-cesium to Your Leptos Project

1. **Add dependency to Cargo.toml:**
```toml
[dependencies]
leptos-cesium = "0.1"  # or git = "https://github.com/..."
```

2. **Install Cesium assets:**
   - Download Cesium release and extract to your project's `public/Cesium/`
   - Or symlink to a shared Cesium installation

3. **Configure environment:**
   - Create `.env.local` with `CESIUM_ION_TOKEN=your_token`
   - Or set environment variable before building

4. **Use ViewerContainer in your app:**
```rust
let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());
view! {
    <ViewerContainer ion_token=ion_token>
        // your content
    </ViewerContainer>
}
```

5. **Update HTML to load Cesium.js:**
```html
<head>
  <link rel="stylesheet" href="Cesium/Widgets/widgets.css" />
  <script src="Cesium/Cesium.js"></script>
</head>
```

## Implementation Guidance

1. Port core patterns from `leptos-leaflet`:
   - Thread-aware JsValue wrappers for SSR safety.
   - Context providers for the viewer, entities, and data sources.
   - `cesium_events!` macro mirroring the Leaflet events builder.
2. Generate bindings with `ts-bindgen`; manual shims should live beside the generated modules.
3. Expand documentation as features land (README, example guides, API docs).

When in doubt, compare behavior and architecture decisions with `leptos-leaflet` and the upstream Leptos workspace for idiomatic practices.
