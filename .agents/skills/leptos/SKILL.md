---
name: leptos
description: "Guidance for editing code in this Leptos repository (0.8.15): idiomatic components/signals/router/server functions, SSR/hydrate/CSR feature hygiene, and common pitfalls. Use when changing Leptos crates or examples in this repo."
---

# Leptos Skill

Use when working in a Rust project that depends on Leptos (`leptos` in `Cargo.toml`). Produce idiomatic, reactive Leptos code.

## Repo-First Sources

When guidance conflicts, follow this order:

1. `CLAUDE.md` for build/test/format workflow in this repo
2. `docs/COMMON_BUGS.md` for known reactivity/template pitfalls
3. Matching patterns in `examples/` for real, current API usage

Prefer adapting an existing example over inventing a new pattern.

## Triage

Before writing code, identify:

1. **Rendering mode**
   - **CSR**: Client-only WASM app. Features: `["csr"]`
   - **SSR**: Server renders HTML. Features: `["ssr"]`
   - **Hydrate**: Server renders + client hydrates. Split build: `ssr` (server) + `hydrate` (client)

2. **App shape**
   - Single page vs router-based (`leptos_router`)
   - Server functions / RPC (`#[server]`)
   - State management: signals, context, or stores
   - Deployment target: native server (Axum/Actix) vs edge worker (Cloudflare Workers)

If rendering mode isn't explicit, infer from `Cargo.toml` features and `package.metadata.leptos` first; ask only if still ambiguous.

## Quick Reference (0.8.x API)

```rust
use leptos::prelude::*;

// Signals (reactive state) - both are Copy + 'static
let count = RwSignal::new(0);           // Unified read/write
let (read, write) = signal(0);          // Split read/write

// Reading and writing
count.get();                            // Read (tracks dependency)
count.get_untracked();                  // Read without tracking
count.set(5);                           // Replace value
count.update(|n| *n += 1);              // Mutate in place

// Derived values
let doubled = move || count.get() * 2;  // Derived (cheap, no cache)
let doubled = Memo::new(move |_| count.get() * 2);  // Memo (cached)

// Effects (side effects only)
Effect::new(move |_| {
    log!("Count: {}", count.get());
});

// Context
provide_context(count);
let count = expect_context::<RwSignal<i32>>();
```

## Core Patterns

### Components

```rust
use leptos::prelude::*;

#[component]
pub fn Counter(
    /// Starting value
    initial: i32,
    /// Prop with Into conversion
    #[prop(into)] label: String,
    /// Optional prop (defaults to None)
    #[prop(optional)] class: Option<String>,
    /// Prop with default value
    #[prop(default = 1)] step: i32,
) -> impl IntoView {
    let count = RwSignal::new(initial);

    view! {
        <div class=class>
            <span>{label} ": " {count}</span>
            <button on:click=move |_| count.update(|n| *n += step)>
                "+"{step}
            </button>
        </div>
    }
}
```

Components run **once** (setup, not render). Reactivity is via signals.

### Children

```rust
#[component]
fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="card">{children()}</div>
    }
}

// Usage
view! { <Card><p>"Content"</p></Card> }
```

### Event Handlers

```rust
view! {
    <button on:click=move |_| count.update(|n| *n += 1)>
        "Click"
    </button>

    // With event data
    <input on:input:target=move |ev| {
        let value = ev.target().value();
        set_text.set(value);
    } />
}
```

### Input Binding (Critical)

Use `prop:value`, not `value`:

```rust
// WRONG - only sets default
<input value=text />

// CORRECT - reactive binding
<input
    prop:value=text
    on:input:target=move |ev| set_text.set(ev.target().value())
/>

// Checkbox
<input type="checkbox" prop:checked=checked on:change=... />
```

### Control Flow

```rust
// Conditional
<Show when=move || count.get() > 0 fallback=|| "Empty">
    <p>"Count: " {count}</p>
</Show>
```

### Reactive Lists (Critical)

**Problem**: Naive `<For>` redraws entire list when any item changes.

**Solution 1: Signals inside items**

```rust
#[derive(Clone)]
struct Todo {
    id: Uuid,                       // Stable ID for key
    title: RwSignal<String>,        // Reactive!
    completed: RwSignal<bool>,      // Reactive!
}

let todos = RwSignal::new(Vec::<Todo>::new());

<For
    each=move || todos.get()
    key=|todo| todo.id
    let:todo
>
    <TodoRow todo />
</For>

#[component]
fn TodoRow(todo: Todo) -> impl IntoView {
    view! {
        <li class:done=move || todo.completed.get()>
            {move || todo.title.get()}
        </li>
    }
}
```

**Solution 2: Stores** (see [references/stores.md](references/stores.md))

```rust
#[derive(Clone, Store)]
struct State {
    #[store(key: usize = |todo| todo.id)]
    todos: Vec<Todo>,
}

<For each=move || store.todos() key=|t| t.id().get() let:todo>
    <TodoRow todo />
</For>
```

**WRONG patterns:**

```rust
// BAD: Plain struct, no signals inside
struct Todo { id: usize, title: String }
let todos = RwSignal::new(Vec::<Todo>::new());  // Full redraw on any change!

// BAD: Snapshot values
<For each=move || todos.get() key=|t| t.id let:todo>
    <li>{todo.title}</li>  // Static! Won't update
</For>
```

### Async and Errors

```rust
// Async with Suspense
<Suspense fallback=|| "Loading...">
    {move || Suspend::new(async move {
        data.await.map(|d| view! { <p>{d}</p> })
    })}
</Suspense>

// Error boundary
<ErrorBoundary fallback=|errors| view! { <p>"Error"</p> }>
    {result_signal}
</ErrorBoundary>
```

## SSR/CSR Setup

**Full guide: [references/ssr-csr-setup.md](references/ssr-csr-setup.md)**

### Rules to Avoid Warnings

1. **Features are mutually exclusive** - never enable `ssr` and `hydrate` together
2. **Server deps must be `optional = true`**
3. **Server imports inside `#[cfg(feature = "ssr")]` module or inside server fn**
4. **Workspaces that target WASM should set `resolver = "2"`**

```toml
[dependencies]
leptos = { version = "0.8" }
sqlx = { version = "0.8", optional = true }  # Server-only!

[features]
ssr = ["leptos/ssr", "dep:sqlx"]
hydrate = ["leptos/hydrate"]
```

### Server-Only Code Pattern

```rust
#[cfg(feature = "ssr")]
pub mod ssr {
    use sqlx::*;
    pub async fn db() -> Result<SqliteConnection, ServerFnError> { ... }
}

#[server]
pub async fn get_data() -> Result<Data, ServerFnError> {
    use self::ssr::*;  // Import HERE, not at file top
    let conn = db().await?;
    // ...
}
```

### Build Commands

```bash
# App-level iteration (cargo-leptos templates/examples)
cargo leptos watch
cargo check --features ssr
cargo check --features hydrate --lib --target wasm32-unknown-unknown

# This repository's contributor workflow
cargo +nightly fmt
cargo +nightly make check
cargo +nightly make test

# If editing an example directory:
# cargo +nightly fmt -- --config-path ../..
# cargo +nightly make --profile=github-actions verify-flow
```

## Cloudflare Workers (Edge SSR)

Use this setup when deploying Leptos SSR/server functions on Cloudflare Workers.

- Keep split features: `hydrate` for browser bundle, `ssr` for worker runtime.
- Worker entrypoint is a `#[worker::event(fetch)]` handler in `src/lib.rs` (not a server `main`).
- `wrangler.toml` build should run both `cargo leptos build` and `worker-build --features ssr`.
- Worker assets come from `target/site` and unmatched routes should flow through Leptos SSR routes.

Full guide: [references/cloudflare-workers.md](references/cloudflare-workers.md)

## Common Pitfalls

1. **Plain structs in `<For/>`**: Use signals inside items or stores (see Reactive Lists above)
2. **`value=` vs `prop:value=`**: Use `prop:value` for reactive inputs
3. **Writing to signals in effects**: Derive what can be derived
4. **Nested signal updates**: Wrap in `batch(|| { ... })`
5. **Unstable keys in `<For/>`**: Use unique, stable IDs (not indices!)
6. **Server imports at file top**: Put inside `#[cfg(feature = "ssr")]` module

See [references/common-bugs.md](references/common-bugs.md) for detailed solutions.

## Advanced Topics

| Topic | Reference |
|-------|-----------|
| SSR/CSR Setup | [references/ssr-csr-setup.md](references/ssr-csr-setup.md) |
| Routing | [references/routing.md](references/routing.md) |
| Server Functions | [references/server-functions.md](references/server-functions.md) |
| Cloudflare Workers | [references/cloudflare-workers.md](references/cloudflare-workers.md) |
| Stores | [references/stores.md](references/stores.md) |
| Islands, Directives, Slots | [references/advanced-patterns.md](references/advanced-patterns.md) |

### Quick Routing Example

```rust
use leptos_router::{components::*, hooks::*, path};

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav><A href="/">"Home"</A></nav>
            <Routes fallback=|| "Not found">
                <Route path=path!("/") view=Home />
                <Route path=path!("/users/:id") view=UserDetail />
            </Routes>
        </Router>
    }
}

#[derive(Params, PartialEq, Clone)]
struct UserParams { id: Option<usize> }

#[component]
fn UserDetail() -> impl IntoView {
    let params = use_params::<UserParams>();
    view! { <p>"User: " {move || params.get().ok().and_then(|p| p.id)}</p> }
}
```

### Quick Server Function Example

```rust
use leptos::prelude::*;
use server_fn::ServerFnError;

#[server]
pub async fn get_user(id: u64) -> Result<User, ServerFnError> {
    // Server-only code
    Ok(db::fetch_user(id).await?)
}

#[component]
fn UserLoader() -> impl IntoView {
    let user = Resource::new(|| (), |_| get_user(1));

    view! {
        <Suspense fallback=|| "Loading...">
            {move || Suspend::new(async move {
                user.await.map(|u| view! { <p>{u.name}</p> })
            })}
        </Suspense>
    }
}
```

## Debugging Checklist

1. **Nothing updates**: Check signal `.get()` is in reactive context (closure)
2. **Input not reflecting state**: Use `prop:value` not `value`
3. **Hydration mismatch**: Ensure same code path for SSR and client
4. **Effect runs repeatedly**: Check for accidental signal writes
5. **Panic on nested update**: Use `batch()` for multiple signal updates

## When to Ask

- Rendering mode (CSR/SSR/hydrate) can't be inferred from features/metadata
- Server functions needed but none exist
- Router/integration stack can't be inferred from dependencies (axum/actix)
- Idiomatic Leptos vs matching existing patterns
