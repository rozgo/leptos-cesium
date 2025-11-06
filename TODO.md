# leptos-cesium Implementation Plan

**Goal:** Create a comprehensive Leptos component library for CesiumJS with auto-generated bindings, matching leptos-leaflet's coverage and patterns.

## Progress Summary (2025-11-05)
- Workspace scaffolding mirrors leptos-leaflet: shared Leptos 0.8 dependencies, crate features (`csr`, `hydrate`, `ssr`), and example workspace members.
- Core utilities ported: thread-safe `JsValue` wrapper with tests plus local-storage JS signal aliases under `src/core`.
- Viewer/entity context scaffolding in place with SSR-aware thread checks and helper provision functions.
- Asset workflow operational via `scripts/sync_cesium_assets.sh`, syncing a vendorized `Cesium-1.135` build into each example's `public` directory.
- Minimal manual bindings added for `Cesium.Viewer` (constructor/destroy) plus `buildModuleUrl.setBaseUrl`; `ViewerContainer` now instantiates a real viewer on wasm targets.
- Hand-rolled entity/Cartesian bindings allow `examples/simple-viewer` to add a point entity once the viewer mounts (CSR smoke test still passes, now with initial scene setup).

## Near-Term Next Steps
1. Flesh out viewer creation options (basic `ViewerOptions` pass-through, animation/timeline toggles) and make base URL overrides ergonomic.
2. Extend entity helpers (context add/remove APIs, typed graphics options) building on the basic bindings.
3. Update `examples/simple-viewer` to include widget CSS guidance and verify assets/Base URL paths work with `cargo leptos`.
4. Kick tooling/docs back into gear after rendering is verified end-to-end (defer cargo-make/hooks until functionality is proven).

## Phase 1: Project Setup & Infrastructure

### 1. Create Cargo workspace structure
- [x] Initialize single `leptos-cesium` crate at `/Users/rozgo/vertex/leptos-cesium/`
- [ ] Copy workspace setup from leptos-leaflet (remaining: shared cargo-make targets, lint config)
- [x] Add dependencies: leptos 0.8, wasm-bindgen, web-sys, js-sys, paste
- [x] Set up CSR/hydrate/SSR features matching leptos-leaflet

### 2. Set up build configuration
- [ ] Configure asset serving for `Cesium-1.135/Build/Cesium/` directory
- [ ] Document Workers and Assets deployment requirements
- [ ] Integrate assets with `cargo leptos` metadata (`assets-dir`) so examples/server builds can mount the vendor directory without bespoke scripts; document when the helper sync script is still useful (plain `cargo` workflows, publishing, etc.)
- [ ] Set up proper CESIUM_BASE_URL configuration
- [x] Provide helper script `scripts/sync_cesium_assets.sh` to fetch/update the Cesium bundle (download official zip/tar, extract to `vendor/Cesium/<version>/Build/Cesium`, sync runtime folders into example `public/Cesium`)

### 3. Document local tooling workflow
- [x] Capture recommended commands (`cargo fmt`, `cargo clippy --all-targets --all-features`, `cargo test`, `cargo leptos build/watch` for examples) in README and CLAUDE guidance
- [ ] Add a minimal `Makefile.toml` (cargo-make) mirroring the Leptos repo conventions (`fmt`, `check`, `test`, `ci` bundles)
- [ ] Provide an optional sample hook under `.githooks/pre-commit` that runs the formatting/lint command bundle for developers who opt in

### 4. Create initial project structure
```
leptos-cesium/
├── Cargo.toml
├── README.md
├── CLAUDE.md
├── leptos-cesium/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── bindings/      # Generated/manual Cesium bindings
│       │   ├── mod.rs
│       │   ├── viewer.rs
│       │   ├── entity.rs
│       │   └── ...
│       ├── components/    # Leptos components
│       │   ├── mod.rs
│       │   ├── context.rs
│       │   ├── viewer_container.rs
│       │   ├── entity.rs
│       │   └── events/
│       ├── core/         # Context system, signals
│       │   ├── mod.rs
│       │   ├── js_signals.rs
│       │   └── thread_safe_jsvalue.rs
│       └── prelude.rs
└── examples/         # Example apps
    ├── simple-viewer/
    ├── with-entities/
    ├── geojson-data/
    ├── camera-control/
    └── with-server/
- [x] Stubbed module tree and placeholder components/contexts to keep the crate compiling
- [x] Added initial `examples/simple-viewer` crate wired into the workspace as a smoke test
- `src/lib.rs` re-exports generated Cesium bindings (e.g. `pub use crate::bindings::generated as cesium;`) alongside `components`, `core`, and `prelude`
```

## Phase 2: Manual Cesium Bindings (initial coverage)

### 5. Bootstrap viewer bindings
- [x] wasm-bindgen `Cesium.Viewer` constructor/destroy glue
- [x] Expose `Cesium.buildModuleUrl.setBaseUrl`
- [ ] Model a minimal `ViewerOptions` struct + pass-through serialization
- [ ] Surface common viewer flags (timeline, animation, infoBox) via component props
- [ ] Add a wasm smoke test that mounts/destroys the viewer

### 6. Entities and scene primitives
- [x] Bind `Cesium.Entity` creation/removal with a lightweight wrapper
- [ ] Wire entity collection helpers on the viewer context
- [x] Provide a temporary Cartesian3 constructor for positioning primitives
- [ ] Exercise entity add/remove in wasm tests

## Phase 3: Core Infrastructure (leptos-leaflet patterns)

### 7. Create core module (`src/core/`)
- [x] Copy thread-safe JsValue wrappers from leptos-leaflet
- [x] Adapt JsRwSignal, JsReadSignal, JsWriteSignal
- [ ] Create coordinate conversion traits:
  - [ ] IntoCartesian3
  - [ ] IntoCartographic
  - [ ] IntoCartesian2
- [ ] Create Position types (similar to leptos-leaflet)
- [ ] Centralize Cesium resource lifecycle utilities (disposal helpers, `request_render` management)

### 8. Implement context system (`src/components/context.rs`)
- [ ] `CesiumViewerContext` (similar to LeafletMapContext)
  - [x] viewer: JsRwSignal<Option<Viewer>>
  - [x] Thread ID validation for SSR
  - [x] Methods: set_viewer, viewer, viewer_untracked, viewer_signal
  - [ ] add_entity, remove_entity helpers
- [ ] `CesiumEntityContext` (similar to LeafletOverlayContainerContext)
  - [x] entity: JsRwSignal<Option<Entity>>
  - [x] Thread-safe accessors
- [ ] `DataSourceContext` for data source management
  - [ ] Track loaded data sources
  - [ ] Provide add/remove methods
- [ ] Helper functions:
  - [x] provide_cesium_context, use_cesium_context
  - [x] extend_context_with_entity, use_entity_context
  - [ ] create_viewer_signal

### 9. Create event system (`src/components/events/`)
- [ ] Define `cesium_events!` macro (adapted from leaflet_events!)
- [ ] Create event modules:
  - [ ] mouse_events.rs (click, double_click, move, wheel)
  - [ ] camera_events.rs (move_start, move_end, changed)
  - [ ] scene_events.rs (render, pre_render, post_render)
  - [ ] entity_events.rs (picked, mouse_over, mouse_out)
- [ ] Integrate with ScreenSpaceEventHandler

## Phase 4: Core Components

### 10. ViewerContainer component (equivalent to MapContainer)
- [ ] Component props:
  - [x] Basic: class, style, node_ref
  - [ ] Viewer options: animation, timeline, base_layer_picker, etc.
  - [ ] Scene options: scene_mode (3D/2D/Columbus)
  - [ ] Camera: home_button, navigation_help_button
  - [ ] Terrain: terrain_provider
  - [ ] Events: viewer_events, camera_events, scene_events
- [ ] Initialize Cesium.Viewer with proper asset paths
- [ ] Set default `CESIUM_BASE_URL` to the synced `public/Cesium` directory (e.g. `/pkg/Cesium`) and allow overriding via props/env
- [x] Provide CesiumViewerContext
- [x] Handle cleanup on unmount
- [x] Support children (entities, data sources, etc.)

### 11. Entity component
- [ ] Props:
  - [ ] position: Signal<Cartesian3> (reactive)
  - [ ] name, description, id
  - [ ] availability (time-based visibility)
  - [ ] show: Signal<bool>
- [ ] Sub-component props:
  - [ ] billboard: BillboardGraphics options
  - [ ] label: LabelGraphics options
  - [ ] point: PointGraphics options
  - [ ] model: ModelGraphics options
  - [ ] path, polygon, polyline, etc.
- [ ] Add/remove from EntityCollection via context
- [x] Support children (for entity-specific overlays)
- [ ] Reactive updates for position and other props

### 12. Camera controls
- [ ] CameraView component for programmatic control
  - [ ] destination: Signal<Cartesian3>
  - [ ] orientation: heading, pitch, roll
  - [ ] duration (for animations)
- [ ] Camera helper functions via context:
  - [ ] fly_to, set_view, look_at
  - [ ] zoom_in, zoom_out, reset
- [ ] Reactive camera position tracking

## Phase 5: Data Sources & Layers

### 13. DataSource components
- [ ] GeoJsonDataSource
  - [ ] url or data prop
  - [ ] styling options (stroke, fill, marker_symbol, etc.)
  - [ ] clamp_to_ground option
- [ ] CzmlDataSource
  - [ ] url or czml data
  - [ ] clock animation control
- [ ] KmlDataSource
  - [ ] url or kml data
  - [ ] camera behavior options
- [ ] CustomDataSource wrapper
  - [ ] Manual entity management

### 14. Imagery components
- [ ] ImageryLayer
  - [ ] provider prop
  - [ ] alpha, brightness, contrast, hue, saturation
  - [ ] show, split_direction
- [ ] OpenStreetMapImageryProvider
- [ ] BingMapsImageryProvider
  - [ ] api_key, map_style
- [ ] UrlTemplateImageryProvider
  - [ ] url template pattern
- [ ] ArcGisMapServerImageryProvider
- [ ] WebMapServiceImageryProvider

### 15. Terrain components
- [ ] TerrainProvider wrapper
- [ ] CesiumTerrainProvider
  - [ ] url, request_vertex_normals, request_water_mask
- [ ] EllipsoidTerrainProvider (flat terrain)
- [ ] createWorldTerrain helper

## Phase 6: Primitives & Graphics

### 16. Primitive components
- [ ] PointPrimitive, PointPrimitiveCollection
  - [ ] position, color, pixel_size, outline
- [ ] Polyline, PolylineCollection
  - [ ] positions, width, material, clamp_to_ground
- [ ] Billboard, BillboardCollection
  - [ ] image, position, scale, rotation
- [ ] Label, LabelCollection
  - [ ] text, position, font, fill_color, outline

### 17. Entity graphics components
- [ ] BoxGraphics
- [ ] CylinderGraphics
- [ ] EllipseGraphics
- [ ] EllipsoidGraphics
- [ ] PathGraphics
- [ ] PolygonGraphics
- [ ] PolylineGraphics (entity version)
- [ ] WallGraphics
- [ ] RectangleGraphics
- [ ] CorridorGraphics

## Phase 7: Widgets & UI

### 18. Widget components (optional toggles via props)
- [ ] Timeline
  - [ ] Control time-based animations
- [ ] Animation
  - [ ] Play/pause, speed control
- [ ] BaseLayerPicker
  - [ ] Switch between imagery providers
- [ ] Geocoder
  - [ ] Search locations
- [ ] HomeButton
  - [ ] Reset camera to home view
- [ ] SceneModePicker
  - [ ] Switch 3D/2D/Columbus
- [ ] NavigationHelpButton
  - [ ] Show mouse/touch controls help
- [ ] FullscreenButton
  - [ ] Toggle fullscreen mode

## Phase 8: Documentation & Examples

### 19. Create comprehensive examples
- [ ] **simple-viewer**: Basic 3D globe
  - [ ] Minimal ViewerContainer setup
  - [ ] OSM imagery layer
  - [ ] Basic camera controls
- [ ] **with-entities**: Entity markers and labels
  - [ ] Multiple Entity components
  - [ ] Billboards and labels
  - [ ] Reactive position updates
- [ ] **geojson-data**: Load GeoJSON
  - [ ] GeoJsonDataSource component
  - [ ] Styling options
  - [ ] Click handling
- [ ] **camera-control**: Reactive camera
  - [ ] Programmatic camera movement
  - [ ] Camera event handling
  - [ ] Smooth animations
- [ ] **with-server**: Full SSR example with Axum
  - [ ] Server setup
  - [ ] Asset serving configuration
  - [ ] Hydration example
- [ ] Ensure each example includes the metadata needed for `cargo leptos build/watch` and document the commands as the primary workflow
- [ ] Sync Cesium assets into each example’s `public/Cesium` directory via `scripts/sync_cesium_assets.sh`

### 20. Write CLAUDE.md
- [ ] Follow leptos-leaflet pattern
- [ ] Document build commands
- [ ] Explain architecture (contexts, events, components)
- [ ] Note Cesium-specific patterns (asset serving, coordinate systems)
- [ ] Version compatibility table

### 21. Write README.md
- [ ] Project description and goals
- [ ] Component list (comprehensive)
- [ ] Installation instructions
- [ ] Asset serving setup (critical for Cesium)
  - [ ] Workers directory
  - [ ] Assets directory
  - [ ] CSS requirements
- [ ] Basic usage example
- [ ] Version compatibility table
- [ ] Feature flags (csr, hydrate, ssr)
- [ ] Link to examples
- [ ] Document Cesium asset acquisition/update workflow and helper script usage

## Key Implementation Notes

### Patterns to Follow from leptos-leaflet:
- All components use Leptos contexts for parent-child communication
- Use `Effect::new` for Cesium object initialization
- Thread-safe signals with thread ID validation for SSR compatibility
- All props accept signals for reactivity
- Event handlers via builder pattern (like MapEvents)
- Cleanup on component unmount (remove from scene/collections)

### Cesium-Specific Considerations:
- **Asset Path Configuration**: Must serve Cesium assets (Workers, Assets, Widgets)
- **CESIUM_BASE_URL**: Set via script tag or configuration
- **WebGL Requirement**: Check for WebGL support, show error if unavailable
- **Clock/Time**: Many features depend on Clock for time-based animations
- **Coordinate Systems**: Handle conversions between Cartesian3, Cartographic, screen coords
- **Async Loading**: Many operations are async (terrain, imagery, data sources)
- **Performance**: Consider requestRender mode instead of continuous rendering
- **Memory Management**: Proper disposal of Cesium objects to prevent leaks

### Development Order Priority:
1. Core bindings + ViewerContainer (get something on screen)
2. Entity component (basic markers/labels)
3. Context system + events
4. Data sources (GeoJSON most useful)
5. Camera controls
6. Additional graphics/primitives
7. Widgets (optional features)

## Testing Strategy
- [ ] Create test utilities for SSR testing
- [ ] Test thread safety of all contexts
- [ ] Test reactive prop updates
- [ ] Test cleanup/disposal
- [ ] Test with different render modes (CSR, hydrate, SSR)
- [ ] Browser compatibility testing (WebGL support)

## Future Enhancements (Post-MVP)
- [ ] 3D Tiles support (large datasets)
- [ ] Particle systems
- [ ] Post-processing effects
- [ ] Custom shaders
- [ ] VR support
- [ ] Web workers for data processing
- [ ] Integration with Cesium ion services
