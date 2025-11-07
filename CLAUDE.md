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

## Code Gotchas & Patterns

### wasm_bindgen Cesium Bindings

**Static properties and namespace methods require reflection:**

Cesium uses namespace objects (not ES6 classes), so `static_method_of` and `getter` attributes don't work for static properties or methods. Use manual reflection instead:

```rust
// ❌ WRONG - This won't work for Cesium namespace objects
#[wasm_bindgen(static_method_of = Color, getter, js_name = RED)]
pub fn red() -> Color;

// ✅ CORRECT - Use reflection to access static properties
fn get_color_property(name: &str) -> Color {
    use js_sys::{global, Reflect};
    use wasm_bindgen::JsCast;

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium"))
        .expect("Cesium global to be available");
    let color_class = Reflect::get(&cesium, &JsValue::from_str("Color"))
        .expect("Cesium.Color to exist");
    Reflect::get(&color_class, &JsValue::from_str(name))
        .expect(&format!("Cesium.Color.{} to exist", name))
        .unchecked_into::<Color>()
}

impl Color {
    pub fn red() -> Color {
        get_color_property("RED")
    }
}
```

**Same pattern for static methods like `Rectangle.fromDegrees()`:**

```rust
pub fn from_degrees(west: f64, south: f64, east: f64, north: f64) -> Rectangle {
    use js_sys::{global, Function, Reflect};
    use wasm_bindgen::{JsCast, JsValue};

    let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium"))
        .expect("Cesium global to be available");
    let rectangle = Reflect::get(&cesium, &JsValue::from_str("Rectangle"))
        .expect("Cesium.Rectangle to exist");
    let from_degrees_fn = Reflect::get(&rectangle, &JsValue::from_str("fromDegrees"))
        .expect("Cesium.Rectangle.fromDegrees to exist");
    let from_degrees_fn: Function = from_degrees_fn
        .dyn_into()
        .expect("Cesium.Rectangle.fromDegrees to be callable");
    from_degrees_fn
        .call4(&rectangle, &JsValue::from_f64(west), &JsValue::from_f64(south),
               &JsValue::from_f64(east), &JsValue::from_f64(north))
        .expect("Cesium.Rectangle.fromDegrees to succeed")
        .unchecked_into::<Rectangle>()
}
```

### Clone Trait for wasm_bindgen Types

**Use `#[derive(Clone)]` directly on extern type definitions:**

```rust
#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]  // ✅ Works with wasm_bindgen extern types
    #[wasm_bindgen(js_namespace = Cesium)]
    pub type Color;
}
```

This generates proper clone implementations for JsValue-based types (ref-counting under the hood).

### Signal Types for JsValue Props

**Use `JsSignal<T>` for JS types, regular `Signal<T>` for primitives:**

```rust
#[component(transparent)]
pub fn RectangleGraphics(
    // JS types use JsSignal (LocalStorage)
    #[prop(into)]
    coordinates: JsSignal<Rectangle>,

    #[prop(optional, into)]
    material: JsSignal<Option<Material>>,

    // Primitives use regular Signal (SyncStorage)
    #[prop(into)]
    semi_minor_axis: Signal<f64>,

    #[prop(optional, into)]
    outline: Signal<Option<bool>>,
) -> impl IntoView
```

**Access JsSignal values with `get_untracked()`:**

```rust
Effect::new(move |_| {
    // Use get_untracked() for JsSignal (LocalStorage doesn't support get())
    let coords = coordinates.get_untracked();
    let mat = material.get_untracked();

    // Use get() for regular Signal
    let radius = semi_minor_axis.get();
    let show_outline = outline.get();
});
```

### Ergonomic Component Props

**Always use `#[prop(into)]` to allow passing raw values:**

```rust
// ✅ Users can pass values directly, Leptos converts to signals
view! {
    <RectangleGraphics
        coordinates=Rectangle::from_degrees(-110.0, 20.0, -80.0, 25.0)
        material=Some(Material::color(Color::red()))
        outline=Some(true)
    />
}

// No need for:
// coordinates=create_signal(Rectangle::from_degrees(...))
// The #[prop(into)] attribute handles the conversion
```

### Builder Pattern for Complex Options

**Use builder pattern for options with multiple fields:**

```rust
// ✅ Clean, self-documenting API
StripeOptions::new()
    .even_color(Color::white())
    .odd_color(Color::blue())
    .repeat(5.0)
    .build()

// ❌ Avoid exposing raw JsValue manipulation to users
let options = Object::new();
Reflect::set(&options, &JsValue::from_str("evenColor"), &JsValue::from(Color::white()));
// ... more boilerplate
```

### Strongly-Typed Enums Over JsValue

**Create typed enums for polymorphic parameters:**

```rust
// ✅ Type-safe material enum
pub enum Material {
    Color(ThreadSafeJsValue<Color>),
    Stripe(ThreadSafeJsValue<StripeMaterialProperty>),
}

impl Material {
    pub fn color(color: Color) -> Self {
        Material::Color(ThreadSafeJsValue::new(color))
    }

    #[cfg(target_arch = "wasm32")]
    pub fn to_js_value(&self) -> JsValue {
        match self {
            Material::Color(c) => JsValue::from(c.value().clone()),
            Material::Stripe(s) => JsValue::from(s.value().clone()),
        }
    }
}

// ❌ Avoid using raw JsValue in public APIs
#[prop(optional)]
material: Option<Signal<JsValue>>  // Hard to use, no type safety
```

### Entity Context Generic Type Specification

**Specify concrete types when calling generic context methods:**

```rust
// Entity context methods are generic over JsCast types
if entity_context.entity_untracked::<CesiumEntity>().is_some() {
    // ... work with entity
}

// Not just entity_untracked() - compiler needs the type
```

### SSR/WASM Conditional Compilation Pattern

**Always consume component props in both SSR and WASM builds to avoid warnings:**

Since Cesium bindings are only available in WASM (`target_arch = "wasm32"`), but Leptos components run in both SSR and browser contexts, you must ensure props are consumed in all build configurations.

**❌ WRONG - Causes "unused variable" warnings in SSR builds:**
```rust
#[component]
pub fn ViewerContainer(
    #[prop(into)] ion_token: Signal<Option<String>>,
    #[prop(optional)] animation: bool,
) -> impl IntoView {
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            // Props only used here - unused in SSR!
            if let Some(token) = ion_token.get() {
                set_default_access_token(&token);
            }
            // use animation...
        }
    });
}
```

**✅ CORRECT - Consume props in both contexts:**

**Pattern 1: For components with Effects (ViewerContainer, Entity):**
```rust
#[component]
pub fn ViewerContainer(
    #[prop(into)] ion_token: Signal<Option<String>>,
    #[prop(optional)] animation: bool,
) -> impl IntoView {
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(token) = ion_token.get() {
                set_default_access_token(&token);
            }
            // use animation...
        }

        // Consume props in SSR builds to avoid warnings
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (ion_token, animation);
        }
    });
}
```

**Pattern 2: For transparent graphics components:**
```rust
#[component(transparent)]
pub fn PointGraphics(
    #[prop(into)] pixel_size: Signal<f64>,
    #[prop(optional, into)] color: JsSignal<Option<Color>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let entity_context = use_entity_context().expect("PointGraphics must be a child of Entity");
        Effect::new(move |_| {
            // use pixel_size, color...
        });
    }

    // Consume props in SSR builds
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (pixel_size, color);
    }
}
```

**Import organization:**
- Move imports used only in WASM code to `#[cfg(target_arch = "wasm32")]` blocks
- Keep common imports (leptos, component props) unconditional
- Example:
```rust
use leptos::prelude::*;  // Used in all contexts
use crate::core::JsSignal;  // Used in all contexts

#[cfg(target_arch = "wasm32")]
use crate::components::use_cesium_context;  // Only used in WASM
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;  // Only used in WASM
```

**Why this matters:**
- `cargo leptos build` compiles for both SSR (native) and browser (WASM)
- Props captured by Effect closures but not used in SSR trigger warnings
- Zero-cost: `let _ = (...)` is optimized away at compile time
- Ensures clean builds: `cargo leptos build` should produce zero warnings
