# CZML Implementation Roadmap

**Last Updated:** 2025-11-11

**Purpose:** Detailed analysis of CZML support in leptos-cesium, gaps compared to full Cesium CZML API, and prioritized recommendations for enhancement.

---

## 📊 Current Implementation Summary

### What Works Today

**Component:** `CzmlDataSource`
- **Location:** `leptos-cesium/src/components/czml_data_source.rs`
- **Bindings:** `leptos-cesium/src/bindings/data_source.rs`

**Supported Features:**
- ✅ Load CZML from URL (`url: Signal<String>`)
- ✅ Clear existing data sources (`clear_existing: Signal<bool>`)
- ✅ Automatic clock synchronization (syncs viewer clock to CZML clock)
- ✅ Automatic animation enablement (`shouldAnimate = true`)
- ✅ Reactive URL changes (removes old, loads new)
- ✅ Cleanup on unmount (removes all data sources)
- ✅ Error logging to console
- ✅ Promise-based async loading with `wasm_bindgen_futures`

**Current API:**
```rust
view! {
    <ViewerContainer ion_token=token animation=true timeline=true>
        <CzmlDataSource
            url="satellites.czml"
            clear_existing=true
        />
    </ViewerContainer>
}
```

**Bindings Layer:**
```rust
// Types bound
pub type DataSourceCollection;
pub type CzmlDataSource;
pub type DataSourceClock;

// Methods bound
DataSourceCollection.add()
DataSourceCollection.remove_all()
CzmlDataSource.clock()

// Helper functions
czml_data_source_load(url: &str) -> Promise
czml_data_source_load_with_options(url: &str, options: &JsValue) -> Promise
```

**Architecture Strengths:**
- Clean reactive API using Leptos signals
- SSR-safe with conditional compilation
- Proper resource cleanup preventing memory leaks
- Works well with example CZML files (satellites, vehicle tracking)

**Working Examples:**
- `examples/czml-viewer` - Demonstrates CZML loading with camera controls

---

## 🔍 Full Cesium CZML API Reference

### CzmlDataSource Static Methods

| Method | Parameters | Returns | Description |
|--------|-----------|---------|-------------|
| `load()` | `czml`, `options?` | `Promise<CzmlDataSource>` | Creates new instance loaded with CZML |
| `processPacketData()` | 7 parameters | - | Helper for custom CZML updaters |
| `processPositionPacketData()` | 6 parameters | - | Helper for PositionProperty creation |
| `processMaterialPacketData()` | 6 parameters | - | Helper for MaterialProperty creation |
| `updaters` | - | Array | Static array of CZML processing functions |

### CzmlDataSource Instance Properties

| Property | Type | Access | Description |
|----------|------|--------|-------------|
| `name` | string | Read/Write | Human-readable identifier for data source |
| `entities` | EntityCollection | Read-only | Collection of entities loaded from CZML |
| `clock` | DataSourceClock | Read-only | Clock settings from CZML (undefined if static) |
| `show` | boolean | Read/Write | Controls visibility of all entities |
| `clustering` | EntityCluster | Read/Write | Clustering options (shareable across data sources) |
| `credit` | Credit | Read/Write | Attribution credit displayed on canvas |
| `isLoading` | boolean | Read-only | Current loading state |

### CzmlDataSource Instance Methods

| Method | Parameters | Returns | Description |
|--------|-----------|---------|-------------|
| `load()` | `czml`, `options?` | `Promise<this>` | Loads data, replacing existing entities |
| `process()` | `czml`, `options?` | `Promise<this>` | Loads data, appending to existing entities |
| `update()` | `time` | boolean | Updates data source to simulation time |

### LoadOptions Object

```typescript
{
  sourceUri?: Resource | string,  // Base URI for resolving relative links
  credit?: Credit | string         // Attribution credit
}
```

### Events Available

| Event | Type | Description |
|-------|------|-------------|
| `changedEvent` | Event | Raised when underlying data changes |
| `errorEvent` | Event | Raised when error occurs during processing |
| `loadingEvent` | Event | Raised when `isLoading` property changes |

### CZML Input Formats

Cesium supports multiple input formats:
1. **URL string** - Load from remote file
2. **Raw CZML string** - Inline JSON string
3. **Parsed JSON object** - JavaScript object
4. **Resource object** - Cesium Resource with headers, retry logic, etc.

---

## ❌ Detailed Gap Analysis

### Coverage Summary

| Feature Category | Cesium API | leptos-cesium | Coverage |
|-----------------|-----------|---------------|----------|
| Loading Methods | 4 formats | 1 format (URL) | 25% |
| Instance Properties | 7 properties | 1 property (clock) | 14% |
| Instance Methods | 3 methods | 1 method (load) | 33% |
| Load Options | 2 options | 0 options | 0% |
| Events | 3 events | 0 events | 0% |
| **Overall** | - | - | **~30%** |

---

### Gap 1: Inline CZML Data Support

**Status:** ❌ Not implemented
**Priority:** 🔴 HIGH
**Impact:** Blocks Rust-first workflows

**What's Missing:**
- Can't pass CZML as string or JSON object
- Must serve CZML files (even for simple/dynamic data)
- Can't generate CZML from Rust structs

**Want This:**
```rust
use serde_json::json;

let czml = json!([
    {"id": "document", "version": "1.0"},
    {
        "id": "point",
        "position": {"cartographicDegrees": [-75.0, 40.0, 0.0]},
        "point": {"pixelSize": 10, "color": {"rgba": [255, 0, 0, 255]}}
    }
]);

view! {
    <CzmlDataSource data=czml.to_string() />  // ❌ NOT SUPPORTED
}
```

**Use Cases Blocked:**
- Dynamic CZML generation from Rust data structures
- Testing without fixture files
- Serverless/edge deployments without file storage
- Real-time data transformation (Rust → CZML → Cesium)

**Implementation Notes:**
- Need `data` prop as alternative to `url`
- Pass string to `CzmlDataSource.load()`
- Minimal binding changes needed

---

### Gap 2: Data Source Visibility Control

**Status:** ❌ Not implemented
**Priority:** 🔴 HIGH
**Impact:** Layer management broken

**What's Missing:**
- Can't hide/show data source without unmounting
- Must recreate component to toggle visibility
- Expensive for large datasets

**Want This:**
```rust
let (show_satellites, set_show_satellites) = signal(true);
let (show_ground_stations, set_show_ground_stations) = signal(true);

view! {
    <div class="layer-controls">
        <label>
            <input type="checkbox" checked=show_satellites />
            "Satellites"
        </label>
        <label>
            <input type="checkbox" checked=show_ground_stations />
            "Ground Stations"
        </label>
    </div>

    <ViewerContainer>
        <CzmlDataSource url="satellites.czml" show=show_satellites />
        <CzmlDataSource url="stations.czml" show=show_ground_stations />
    </ViewerContainer>
}
```

**Use Cases Blocked:**
- GIS-style layer controls
- Multi-scenario comparison (toggle between datasets)
- Progressive disclosure of information
- Performance optimization (hide instead of destroy)

**Implementation Notes:**
- Add `show` prop (Signal<bool>)
- Bind `show` getter/setter on CzmlDataSource
- Reactive effect to update on signal changes

---

### Gap 3: Loading State & Error Callbacks

**Status:** ⚠️ Partial (internal only)
**Priority:** 🟡 MEDIUM-HIGH
**Impact:** Poor UX, no error recovery

**What's Missing:**
- No loading state exposed to user
- Errors only logged to console
- Can't show loading spinners or progress
- Can't implement retry logic

**Want This:**
```rust
let (is_loading, set_is_loading) = signal(false);
let (error_msg, set_error_msg) = signal(None::<String>);

view! {
    <div>
        {move || is_loading.get().then(|| view! {
            <div class="loading-overlay">
                <div class="spinner" />
                "Loading satellite data..."
            </div>
        })}

        {move || error_msg.get().map(|err| view! {
            <div class="error-banner">
                "Error: " {err}
                <button on:click=retry_load>"Retry"</button>
            </div>
        })}

        <ViewerContainer>
            <CzmlDataSource
                url="data.czml"
                on_loading=Callback::new(move |loading| set_is_loading.set(loading))
                on_error=Callback::new(move |err| set_error_msg.set(Some(err)))
                on_load=Callback::new(move |_| set_error_msg.set(None))
            />
        </ViewerContainer>
    </div>
}
```

**Use Cases Blocked:**
- Loading spinners/progress bars
- User-facing error messages
- Retry mechanisms
- Analytics tracking (success/failure rates)

**Implementation Notes:**
- Add `on_load`, `on_error`, `on_loading` callback props
- Track state through promise lifecycle
- Call callbacks at appropriate points

---

### Gap 4: Process Mode (Incremental Loading)

**Status:** ❌ Not implemented
**Priority:** 🟡 MEDIUM
**Impact:** Can't stream updates

**What's Missing:**
- Only "replace" mode (via `load()`) supported
- No "append" mode (via `process()`)
- Can't incrementally add entities
- Must reload entire dataset for updates

**Want This:**
```rust
pub enum CzmlLoadMode {
    Replace,  // Current: load() - replaces all entities
    Append,   // New: process() - adds to existing entities
}

view! {
    <ViewerContainer>
        // Initial load
        <CzmlDataSource
            url="initial-positions.czml"
            mode=CzmlLoadMode::Replace
        />

        // Incremental updates (append new data)
        <CzmlDataSource
            data=update_czml
            mode=CzmlLoadMode::Append
        />
    </ViewerContainer>
}
```

**Use Cases Blocked:**
- Real-time telemetry streaming
- WebSocket data updates
- Progressive loading of large datasets
- Append-only time-series data

**Implementation Notes:**
- Add `mode` prop (enum: Replace | Append)
- Create helper for `CzmlDataSource.process()`
- Conditional logic based on mode

---

### Gap 5: Clustering Support

**Status:** ❌ Not implemented
**Priority:** 🟡 MEDIUM
**Impact:** Performance issues with large datasets

**What's Missing:**
- No clustering configuration
- Thousands of entities cause performance issues
- No way to group nearby entities
- No cluster styling options

**Want This:**
```rust
let cluster_config = {
    let cluster = EntityCluster::new();
    cluster.set_enabled(true);
    cluster.set_pixel_range(50);
    cluster.set_minimum_cluster_size(2);
    cluster
};

view! {
    <ViewerContainer>
        <CzmlDataSource
            url="10000-sensors.czml"
            clustering=Some(cluster_config)
        />
    </ViewerContainer>
}
```

**Use Cases Blocked:**
- Large sensor networks (thousands of points)
- City-scale data visualization
- Global datasets (all airports, weather stations, etc.)
- Any scenario with 1000+ entities

**Implementation Notes:**
- Bind `EntityCluster` type
- Add `clustering` prop (JsSignal<Option<EntityCluster>>)
- Bind `clustering` getter/setter on CzmlDataSource
- Create cluster configuration builder

---

### Gap 6: Data Source Naming

**Status:** ❌ Not implemented
**Priority:** 🟢 LOW
**Impact:** Can't identify data sources

**What's Missing:**
- No `name` property
- Can't distinguish multiple data sources
- Debugging is harder

**Want This:**
```rust
view! {
    <ViewerContainer>
        <CzmlDataSource name="Satellites" url="satellites.czml" />
        <CzmlDataSource name="Ground Stations" url="stations.czml" />
        <CzmlDataSource name="Coverage Circles" url="coverage.czml" />
    </ViewerContainer>
}
```

**Use Cases:**
- Multi-layer management
- UI layer lists/legends
- Debugging and logging
- Data source identification

**Implementation Notes:**
- Add `name` prop (Signal<String>)
- Use constructor instead of static load
- Bind instance `load()` method

---

### Gap 7: Credit/Attribution

**Status:** ❌ Not implemented
**Priority:** 🟢 LOW
**Impact:** Legal requirements for some data

**What's Missing:**
- Can't set data credit/attribution
- Legal compliance issues for commercial data

**Want This:**
```rust
view! {
    <CzmlDataSource
        url="commercial-data.czml"
        credit="Data © Provider 2025"
    />
}
```

**Use Cases:**
- License compliance
- Commercial data attribution
- Legal requirements

---

### Gap 8: Source URI Override

**Status:** ❌ Not implemented
**Priority:** 🟢 LOW
**Impact:** Edge case for relative URLs

**What's Missing:**
- Can't override base URI for relative links
- CZML with relative image/model URLs won't resolve correctly

**Want This:**
```rust
view! {
    <CzmlDataSource
        url="data.czml"
        source_uri="https://cdn.example.com/"
    />
}
```

**Use Cases:**
- CZML with relative asset URLs
- CDN/proxy scenarios
- Cross-origin resources

---

### Gap 9: Entity Collection Access

**Status:** ❌ Not implemented
**Priority:** 🟢 LOW
**Impact:** Can't access loaded entities

**What's Missing:**
- Can't get EntityCollection from data source
- Can't modify entities after load
- Can't filter/query loaded entities

**Want This:**
```rust
let entities_ref = create_signal(None);

view! {
    <CzmlDataSource
        url="data.czml"
        bind:entities=entities_ref
    />
}
// Later: inspect or modify entities
```

**Use Cases:**
- Post-load entity modification
- Entity filtering
- Custom styling after load

---

## 🎯 Prioritized Recommendations

### Tier 1: Must-Have Features (Next Sprint)

#### 1. Inline CZML Data Support ⭐⭐⭐⭐⭐

**Effort:** LOW (1-2 days)
**Impact:** HIGH
**Priority:** 🔴 CRITICAL

**Changes Required:**

Add `data` prop:
```rust
#[component(transparent)]
pub fn CzmlDataSource(
    #[prop(optional, into)]
    url: Option<Signal<String>>,

    #[prop(optional, into)]
    data: Option<Signal<String>>,  // NEW

    #[prop(optional, into, default = true.into())]
    clear_existing: Signal<bool>,
) -> impl IntoView
```

Implementation:
```rust
Effect::new(move |_| {
    let czml_source = if let Some(url_signal) = url {
        url_signal.get()
    } else if let Some(data_signal) = data {
        data_signal.get()
    } else {
        return; // No source provided
    };

    let promise = czml_data_source_load(&czml_source);
    // ... existing promise handling
});
```

**Benefits:**
- Enables Rust → CZML → Cesium workflow
- Use `serde_json` to generate CZML
- No file I/O for simple data
- Testing without fixtures

---

#### 2. Data Source Visibility Control ⭐⭐⭐⭐⭐

**Effort:** LOW (1 day)
**Impact:** HIGH
**Priority:** 🔴 CRITICAL

**Changes Required:**

Add `show` prop:
```rust
#[prop(optional, into, default = true.into())]
show: Signal<bool>,
```

New bindings needed:
```rust
#[wasm_bindgen(method, getter, js_name = show)]
pub fn show(this: &CzmlDataSource) -> bool;

#[wasm_bindgen(method, setter, js_name = show)]
pub fn set_show(this: &CzmlDataSource, value: bool);
```

Implementation:
```rust
// Track data source reference
let current_ds = StoredValue::new(None::<CzmlDataSource>);

// Separate effect for show property
Effect::new(move |_| {
    let should_show = show.get();
    if let Some(ds) = current_ds.get_value() {
        ds.set_show(should_show);
    }
});
```

**Benefits:**
- Layer controls without unmounting
- Better performance (hide vs destroy)
- Multi-layer visualization
- Progressive disclosure

---

#### 3. Loading State & Error Callbacks ⭐⭐⭐⭐

**Effort:** MEDIUM (2-3 days)
**Impact:** HIGH
**Priority:** 🟡 HIGH

**Changes Required:**

Add callback props:
```rust
#[prop(optional)]
on_load: Option<Callback<()>>,

#[prop(optional)]
on_error: Option<Callback<String>>,

#[prop(optional)]
on_loading: Option<Callback<bool>>,
```

Implementation:
```rust
// Before load
if let Some(cb) = on_loading {
    cb.call(true);
}

// Promise handling
match JsFuture::from(promise).await {
    Ok(ds) => {
        if let Some(cb) = on_load {
            cb.call(());
        }
        if let Some(cb) = on_loading {
            cb.call(false);
        }
    }
    Err(e) => {
        let error_msg = format!("{:?}", e);
        if let Some(cb) = on_error {
            cb.call(error_msg);
        } else {
            console::error_1(&JsValue::from_str(&error_msg));
        }
        if let Some(cb) = on_loading {
            cb.call(false);
        }
    }
}
```

**Benefits:**
- Loading spinners/progress
- User-facing error messages
- Retry logic
- Better UX

---

### Tier 2: Should-Have Features (Next 2-3 Sprints)

#### 4. Data Source Naming ⭐⭐⭐

**Effort:** LOW (1 day)
**Impact:** MEDIUM
**Priority:** 🟢 MEDIUM

**Changes Required:**

Add `name` prop:
```rust
#[prop(optional, into)]
name: Option<Signal<String>>,
```

New bindings:
```rust
#[wasm_bindgen(constructor, js_namespace = Cesium)]
pub fn new(name: &str) -> CzmlDataSource;

#[wasm_bindgen(method, js_name = load)]
pub fn load(this: &CzmlDataSource, czml: &JsValue) -> js_sys::Promise;
```

Implementation:
```rust
let ds = if let Some(name) = name {
    CzmlDataSource::new(&name.get())
} else {
    CzmlDataSource::new("")
};
let promise = ds.load(&czml_js_value);
```

**Benefits:**
- Data source identification
- Layer management
- Debugging

---

#### 5. Process Mode (Incremental Loading) ⭐⭐⭐⭐

**Effort:** MEDIUM (2-3 days)
**Impact:** MEDIUM-HIGH
**Priority:** 🟡 MEDIUM

**Changes Required:**

Add mode enum:
```rust
#[derive(Clone, Copy, PartialEq)]
pub enum CzmlLoadMode {
    Replace,  // Current: load()
    Append,   // New: process()
}

#[prop(optional, into, default = CzmlLoadMode::Replace.into())]
mode: Signal<CzmlLoadMode>,
```

New bindings:
```rust
pub fn czml_data_source_process(czml: &str) -> js_sys::Promise {
    // Similar to load, but calls .process() on existing instance
}
```

Implementation:
```rust
let promise = match mode.get() {
    CzmlLoadMode::Replace => czml_data_source_load(&czml),
    CzmlLoadMode::Append => czml_data_source_process(&czml),
};
```

**Benefits:**
- Streaming data updates
- Real-time telemetry
- Incremental loading
- WebSocket integration

---

#### 6. Clustering Support ⭐⭐⭐⭐

**Effort:** MEDIUM (3-4 days)
**Impact:** HIGH (for large datasets)
**Priority:** 🟡 MEDIUM

**Changes Required:**

New type bindings:
```rust
#[wasm_bindgen(js_namespace = Cesium)]
pub type EntityCluster;

#[wasm_bindgen(constructor, js_namespace = Cesium)]
pub fn new() -> EntityCluster;

#[wasm_bindgen(method, getter, js_name = enabled)]
pub fn enabled(this: &EntityCluster) -> bool;

#[wasm_bindgen(method, setter, js_name = enabled)]
pub fn set_enabled(this: &EntityCluster, value: bool);

#[wasm_bindgen(method, getter, js_name = pixelRange)]
pub fn pixel_range(this: &EntityCluster) -> f64;

#[wasm_bindgen(method, setter, js_name = pixelRange)]
pub fn set_pixel_range(this: &EntityCluster, value: f64);

#[wasm_bindgen(method, getter, js_name = minimumClusterSize)]
pub fn minimum_cluster_size(this: &EntityCluster) -> i32;

#[wasm_bindgen(method, setter, js_name = minimumClusterSize)]
pub fn set_minimum_cluster_size(this: &EntityCluster, value: i32);
```

Add prop:
```rust
#[prop(optional, into)]
clustering: JsSignal<Option<EntityCluster>>,
```

Implementation:
```rust
Effect::new(move |_| {
    if let Some(cluster) = clustering.get_untracked() {
        if let Some(ds) = current_ds.get_value() {
            ds.set_clustering(&cluster);
        }
    }
});
```

**Benefits:**
- Handle thousands of entities
- Better performance
- Better UX for dense data

---

### Tier 3: Nice-to-Have Features (Future)

#### 7. Credit/Attribution ⭐⭐

**Effort:** LOW (1 day)
**Impact:** LOW
**Priority:** 🟢 LOW

Add `credit` prop with string-to-Credit conversion.

---

#### 8. Source URI Override ⭐

**Effort:** LOW (1 day)
**Impact:** LOW
**Priority:** 🟢 LOW

Add `source_uri` prop, pass in options to load.

---

#### 9. Entity Collection Access ⭐

**Effort:** MEDIUM (2 days)
**Impact:** LOW
**Priority:** 🟢 LOW

Expose `entities` via signal or ref.

---

## 💼 Real-World Use Cases

### Use Case 1: Satellite Tracking

**Status:** ✅ Works Well (with current implementation)

**Requirements:**
- Load satellite orbit CZML
- Timeline/clock synchronization
- Time-based animation

**Missing:**
- Layer controls (show/hide different constellations)
- Real-time updates (ISS position updates)

**Priority:** Medium enhancements

---

### Use Case 2: Vehicle Tracking

**Status:** ⚠️ Partial (needs enhancements)

**Requirements:**
- Load vehicle positions from CZML
- Stream position updates (WebSocket → CZML)
- Multiple vehicle layers
- Error handling for connection loss

**Missing (Critical):**
- Process mode for streaming
- Show/hide per layer
- Loading/error callbacks

**Priority:** HIGH

---

### Use Case 3: Point Clouds / Large Datasets

**Status:** ❌ Not Well Supported

**Requirements:**
- Visualize 10,000+ points (sensors, assets)
- Cluster for performance
- Toggle entity groups

**Missing (Critical):**
- Clustering support
- Layer management
- Loading progress

**Priority:** HIGH

---

### Use Case 4: Dynamic Visualization

**Status:** ❌ Not Supported

**Requirements:**
- Generate CZML from Rust structs
- Update visualization without files
- Transform data in Rust → CZML

**Missing (Blocking):**
- Inline data support

**Priority:** HIGH

---

### Use Case 5: Multi-Layer Comparison

**Status:** ⚠️ Partial

**Requirements:**
- Load multiple CZML files
- Toggle layers individually
- Identify layers in UI

**Missing:**
- Show/hide without unmounting
- Layer naming

**Priority:** MEDIUM

---

### Use Case 6: Streaming Updates

**Status:** ❌ Not Supported

**Requirements:**
- WebSocket → CZML → Cesium pipeline
- Incremental updates
- Connection state management

**Missing (Blocking):**
- Process mode
- Error callbacks
- Inline data

**Priority:** HIGH

---

## 📦 Implementation Effort Summary

### Tier 1 (Must-Have)
| Feature | Effort | Bindings | Props | Total |
|---------|--------|----------|-------|-------|
| Inline Data | 1-2 days | 0 new | 1 | ~2 days |
| Show/Hide | 1 day | 2 new | 1 | ~1 day |
| Callbacks | 2-3 days | 0 new | 3 | ~3 days |
| **Total** | **~1 week** | **2** | **5** | **~6 days** |

### Tier 2 (Should-Have)
| Feature | Effort | Bindings | Props | Total |
|---------|--------|----------|-------|-------|
| Naming | 1 day | 2 new | 1 | ~1 day |
| Process Mode | 2-3 days | 1 new | 1 | ~2 days |
| Clustering | 3-4 days | ~8 new | 1 | ~4 days |
| **Total** | **~1.5 weeks** | **11** | **3** | **~7 days** |

### Tier 3 (Nice-to-Have)
| Feature | Effort | Total |
|---------|--------|-------|
| Credit | 1 day | ~1 day |
| Source URI | 1 day | ~1 day |
| Entities | 2 days | ~2 days |
| **Total** | **~4 days** | **~4 days** |

### Grand Total
- **All Tiers:** ~3.5 weeks
- **Tier 1 Only:** ~1 week (recommended minimum)
- **Tier 1 + Tier 2:** ~2.5 weeks (recommended for production)

---

## 🎯 Recommended Implementation Order

### Phase 1: Essential UX (Week 1)
1. Inline CZML data (2 days)
2. Show/hide control (1 day)
3. Loading/error callbacks (3 days)

**Result:** 70% of use cases enabled

### Phase 2: Real-Time Support (Week 2-3)
4. Data source naming (1 day)
5. Process mode (2 days)
6. Clustering (4 days)

**Result:** Real-time apps, large datasets supported

### Phase 3: Polish (Week 4)
7. Credit/attribution
8. Source URI
9. Entity access

**Result:** Feature-complete CZML support

---

## 📝 Code Examples

### Example 1: Inline CZML Generation

```rust
use serde_json::json;
use leptos::prelude::*;
use leptos_cesium::prelude::*;

#[component]
fn DynamicCzmlDemo() -> impl IntoView {
    let (czml_data, set_czml_data) = signal(String::new());

    // Generate CZML from Rust data
    Effect::new(move |_| {
        let czml = json!([
            {
                "id": "document",
                "name": "Generated from Rust",
                "version": "1.0"
            },
            {
                "id": "point1",
                "name": "San Francisco",
                "position": {
                    "cartographicDegrees": [-122.4194, 37.7749, 0]
                },
                "point": {
                    "pixelSize": 10,
                    "color": {"rgba": [255, 0, 0, 255]}
                }
            },
            {
                "id": "point2",
                "name": "New York",
                "position": {
                    "cartographicDegrees": [-74.0060, 40.7128, 0]
                },
                "point": {
                    "pixelSize": 10,
                    "color": {"rgba": [0, 0, 255, 255]}
                }
            }
        ]);

        set_czml_data.set(czml.to_string());
    });

    view! {
        <ViewerContainer ion_token=token>
            <CzmlDataSource data=czml_data />
        </ViewerContainer>
    }
}
```

---

### Example 2: Layer Control with Visibility

```rust
#[component]
fn LayerControlDemo() -> impl IntoView {
    let (show_satellites, set_show_satellites) = signal(true);
    let (show_ground_stations, set_show_ground_stations) = signal(true);
    let (show_coverage, set_show_coverage) = signal(false);

    view! {
        <div class="app">
            <div class="sidebar">
                <h3>"Layers"</h3>
                <label>
                    <input
                        type="checkbox"
                        checked=show_satellites
                        on:change=move |_| set_show_satellites.update(|v| *v = !*v)
                    />
                    " Satellites"
                </label>
                <label>
                    <input
                        type="checkbox"
                        checked=show_ground_stations
                        on:change=move |_| set_show_ground_stations.update(|v| *v = !*v)
                    />
                    " Ground Stations"
                </label>
                <label>
                    <input
                        type="checkbox"
                        checked=show_coverage
                        on:change=move |_| set_show_coverage.update(|v| *v = !*v)
                    />
                    " Coverage Areas"
                </label>
            </div>

            <ViewerContainer ion_token=token>
                <CzmlDataSource
                    name="Satellites"
                    url="satellites.czml"
                    show=show_satellites
                />
                <CzmlDataSource
                    name="Ground Stations"
                    url="ground-stations.czml"
                    show=show_ground_stations
                />
                <CzmlDataSource
                    name="Coverage"
                    url="coverage-circles.czml"
                    show=show_coverage
                />
            </ViewerContainer>
        </div>
    }
}
```

---

### Example 3: Loading States & Error Handling

```rust
#[component]
fn LoadingErrorDemo() -> impl IntoView {
    let (is_loading, set_is_loading) = signal(false);
    let (error_msg, set_error_msg) = signal(None::<String>);
    let (success, set_success) = signal(false);
    let (czml_url, set_czml_url) = signal("satellites.czml".to_string());

    let on_loading = Callback::new(move |loading: bool| {
        set_is_loading.set(loading);
        if loading {
            set_success.set(false);
            set_error_msg.set(None);
        }
    });

    let on_error = Callback::new(move |err: String| {
        set_error_msg.set(Some(err));
        set_success.set(false);
    });

    let on_load = Callback::new(move |_| {
        set_success.set(true);
        set_error_msg.set(None);
    });

    let retry = move |_| {
        // Force reload by updating signal
        set_czml_url.update(|url| *url = url.clone());
    };

    view! {
        <div class="app">
            {move || is_loading.get().then(|| view! {
                <div class="loading-overlay">
                    <div class="spinner"></div>
                    <p>"Loading CZML data..."</p>
                </div>
            })}

            {move || error_msg.get().map(|err| view! {
                <div class="error-banner">
                    <strong>"Error:"</strong> " " {err}
                    <button on:click=retry>"Retry"</button>
                </div>
            })}

            {move || success.get().then(|| view! {
                <div class="success-banner">
                    "✓ Data loaded successfully!"
                </div>
            })}

            <ViewerContainer ion_token=token>
                <CzmlDataSource
                    url=czml_url
                    on_loading=on_loading
                    on_error=on_error
                    on_load=on_load
                />
            </ViewerContainer>
        </div>
    }
}
```

---

### Example 4: Streaming Updates (Process Mode)

```rust
#[component]
fn StreamingDemo() -> impl IntoView {
    let (base_loaded, set_base_loaded) = signal(false);
    let (update_czml, set_update_czml) = signal(String::new());

    // Simulate WebSocket updates
    Effect::new(move |_| {
        if base_loaded.get() {
            // Set up interval for position updates
            let interval_handle = set_interval(
                move || {
                    spawn_local(async move {
                        // Fetch new positions from server/WebSocket
                        let positions = fetch_vehicle_updates().await;

                        // Generate incremental CZML
                        let czml = json!([
                            {
                                "id": "document",
                                "version": "1.0"
                            },
                            // ... updated vehicle positions
                        ]);

                        set_update_czml.set(czml.to_string());
                    });
                },
                Duration::from_secs(5)
            );

            on_cleanup(move || {
                interval_handle.clear();
            });
        }
    });

    view! {
        <ViewerContainer ion_token=token animation=true timeline=true>
            // Initial load - replace mode
            <CzmlDataSource
                url="initial-vehicle-positions.czml"
                mode=CzmlLoadMode::Replace
                on_load=Callback::new(move |_| set_base_loaded.set(true))
            />

            // Streaming updates - append mode
            {move || (base_loaded.get() && !update_czml.get().is_empty()).then(|| view! {
                <CzmlDataSource
                    data=update_czml
                    mode=CzmlLoadMode::Append
                />
            })}
        </ViewerContainer>
    }
}
```

---

### Example 5: Clustering for Large Datasets

```rust
#[component]
fn ClusteringDemo() -> impl IntoView {
    // Create cluster configuration
    let cluster_config = {
        let cluster = EntityCluster::new();
        cluster.set_enabled(true);
        cluster.set_pixel_range(50);
        cluster.set_minimum_cluster_size(2);

        // TODO: Add custom cluster styling when event system is implemented
        // cluster.cluster_event().add_listener(...)

        cluster
    };

    view! {
        <ViewerContainer ion_token=token>
            <CzmlDataSource
                name="Sensor Network"
                url="10000-sensors.czml"
                clustering=Some(cluster_config)
            />
        </ViewerContainer>
    }
}
```

---

## 🏁 Conclusion

### Current State
- **Coverage:** ~30% of Cesium CZML API
- **Status:** Solid foundation, basic use cases work well
- **Limitations:** Missing critical features for production apps

### Critical Gaps
1. ❌ No inline data (blocks Rust workflows)
2. ❌ No show/hide (blocks layer management)
3. ⚠️ Limited error handling (poor UX)
4. ❌ No streaming (blocks real-time apps)
5. ❌ No clustering (performance issues)

### Recommendations
1. **Immediate:** Implement Tier 1 features (~1 week)
   - Enables 70% of use cases
   - Unlocks Rust-first workflows
   - Provides essential UX features

2. **Short-term:** Implement Tier 2 features (~1.5 weeks)
   - Enables real-time applications
   - Handles large datasets
   - Production-ready

3. **Future:** Tier 3 features (~4 days)
   - Edge cases
   - Advanced features
   - 100% API coverage

### Next Steps
- Review and prioritize based on project needs
- Start with Tier 1 implementation
- Create examples demonstrating new features
- Update documentation as features land

---

**Total Effort Estimate:**
- Minimum viable (Tier 1): ~1 week
- Production-ready (Tier 1 + 2): ~2.5 weeks
- Feature-complete (All tiers): ~3.5 weeks
