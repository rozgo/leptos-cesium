# Cloudflare Workers (Leptos) Reference

Based on the template at:
`/Users/rozgo/vertex/workers-rs/templates/leptos`

## Table of Contents
- [When to Use](#when-to-use)
- [Architecture](#architecture)
- [Cargo Setup](#cargo-setup)
- [Worker Entrypoint](#worker-entrypoint)
- [Wrangler Build Pipeline](#wrangler-build-pipeline)
- [Commands](#commands)
- [Server Functions and Worker Env](#server-functions-and-worker-env)
- [Checklist](#checklist)

## When to Use

Use this pattern when you want:
- Leptos SSR and server functions on Cloudflare Workers
- Static assets served by Workers Assets
- Unmatched routes rendered by Leptos on the Worker runtime

## Architecture

The template uses a two-build flow:
1. Build browser/hydration assets with `cargo leptos` (`hydrate` feature).
2. Build Worker backend with `worker-build` (`ssr` feature).

The Worker serves files from `target/site` as assets; non-asset requests hit the Worker fetch handler.

## Cargo Setup

Core shape from template:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
axum = { version = "0.8", default-features = false, optional = true }
leptos = { version = "0.8" }
leptos_axum = { version = "0.8", default-features = false, features = ["wasm"], optional = true }
leptos_meta = { version = "0.8" }
leptos_router = { version = "0.8" }
worker = { version = "0.7", features = ["http", "axum", "d1"], optional = true }
getrandom = { version = "0.3", features = ["wasm_js"] }
wasm-bindgen = "0.2.105"
tower-service = "0.3"

[features]
hydrate = ["leptos/hydrate"]
ssr = [
  "dep:axum",
  "dep:leptos_axum",
  "dep:worker",
  "leptos/ssr",
  "leptos_router/ssr",
]

[package.metadata.leptos]
wasm-validation = false
output-name = "my_app"
site-root = "target/site"
site-pkg-dir = "pkg"
site-addr = "127.0.0.1:8787"
bin-features = ["ssr"]
bin-default-features = false
lib-features = ["hydrate"]
lib-default-features = false
```

Also in `.cargo/config.toml`:

```toml
[target.wasm32-unknown-unknown]
rustflags = ["--cfg", "getrandom_backend=\"wasm_js\""]
```

## Worker Entrypoint

Use a Worker fetch event under `#[cfg(feature = "ssr")]`:

```rust
#[cfg(feature = "ssr")]
#[worker::event(fetch)]
async fn fetch(
    req: worker::HttpRequest,
    env: worker::Env,
    _ctx: worker::Context,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    use std::sync::Arc;
    use axum::{Extension, Router};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_service::Service;

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(app::App);

    let mut router = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || app::shell(leptos_options.clone())
        })
        .with_state(leptos_options)
        .layer(Extension(Arc::new(env)));

    Ok(router.call(req).await?)
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    leptos::mount::hydrate_body(app::App);
}
```

## Wrangler Build Pipeline

Template `wrangler.toml` pattern:

```toml
name = "my_app"
main = "build/index.js"

[build]
command = "cargo leptos build --release && cargo install -q worker-build@^0.7 && LEPTOS_OUTPUT_NAME=my_app worker-build --release --features ssr"

[assets]
directory = "./target/site"
```

`LEPTOS_OUTPUT_NAME` must match `package.metadata.leptos.output-name`.

## Commands

```bash
# one-time
cargo install --locked cargo-leptos

# local dev
npx wrangler dev

# deploy
npx wrangler deploy
```

## Server Functions and Worker Env

The template injects Worker `Env` into Axum via:

```rust
.layer(Extension(Arc::new(env)))
```

Inference from this pattern: in server functions, you can typically extract it via Axum extractors (for example using `leptos_axum::extract` + `axum::Extension`), then access bindings like D1/KV/R2 through `worker::Env`.

## Checklist

1. Keep `hydrate` and `ssr` features separate and mutually exclusive per build.
2. Keep Worker/Axum deps `optional` and only enabled in `ssr`.
3. Include `leptos_router/ssr` in `ssr` feature.
4. Use `#[worker::event(fetch)]` in `src/lib.rs`; avoid a native server `main` for Worker runtime.
5. Ensure `wrangler.toml` assets directory is `target/site`.
6. Ensure `LEPTOS_OUTPUT_NAME` matches Leptos output-name.
7. Keep `getrandom` wasm setup (`features = ["wasm_js"]` plus rustflag cfg).
