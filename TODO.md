# leptos-cesium Implementation Status & Roadmap

**Last Updated:** 2025-11-11

**Goal:** Create a comprehensive Leptos component library for CesiumJS with idiomatic patterns, matching leptos-leaflet's ergonomics while exposing Cesium's unique 3D capabilities.

---

## ✅ Completed Features

### Core Infrastructure
- ✅ Thread-safe JsValue wrappers (`ThreadSafeJsValue`)
- ✅ Specialized signal types (`JsSignal`, `JsReadSignal`, `JsWriteSignal`, `JsStoredValue`)
- ✅ Context system (`CesiumViewerContext`, `CesiumEntityContext`)
- ✅ Helper functions (`provide_cesium_context`, `use_cesium_context`, `extend_context_with_entity`, `use_entity_context`)
- ✅ SSR-safe architecture with thread validation
- ✅ Proper cleanup handlers (`on_cleanup`) throughout

### ViewerContainer Component
- ✅ Basic props: `class`, `style`, `ion_token`
- ✅ All UI widget toggles: `animation`, `timeline`, `base_layer_picker`, `home_button`, `scene_mode_picker`, `navigation_help_button`, `fullscreen_button`
- ✅ `should_animate` prop for CZML clock synchronization
- ✅ Context provision for child components
- ✅ Children support
- ✅ Cleanup on unmount (viewer destruction)

### Entity Component
- ✅ `name` prop
- ✅ Add/remove from EntityCollection
- ✅ Children support for graphics components
- ✅ Entity context provision
- ✅ Cleanup on unmount (entity removal)

### Graphics Components (11 types)

**2D Shapes:**
- ✅ **RectangleGraphics** - Rectangles with coordinates, materials, outlines
- ✅ **PolygonGraphics** - Polygons with holes support (PolygonHierarchy)
- ✅ **EllipseGraphics** - Ellipses with rotation, semi-axes

**3D Primitives:**
- ✅ **BoxGraphics** - Cuboid shapes with dimensions
- ✅ **EllipsoidGraphics** - Spheres and ellipsoids with radii
- ✅ **CylinderGraphics** - Cylinders and cones with adjustable top/bottom radii

**Paths & Volumes:**
- ✅ **PolylineGraphics** - Lines with width and materials
- ✅ **WallGraphics** - Vertical walls with positions and heights
- ✅ **CorridorGraphics** - Corridor paths with width
- ✅ **PolylineVolumeGraphics** - Custom 2D shapes extruded along paths

**Points:**
- ✅ **PointGraphics** - Point markers with pixel size, colors, outlines

### Materials System (4 types with Builder APIs)
- ✅ **Color Material** - Solid colors with alpha support
- ✅ **Stripe Material** - Striped patterns (`StripeOptions` builder)
- ✅ **Checkerboard Material** - Checkerboard patterns (`CheckerboardOptions` builder)
- ✅ **Polyline Glow Material** - Glowing polylines (`PolylineGlowOptions` builder)

### Camera Controls (5 components)
- ✅ **CameraFlyTo** - Animated flight to destination with orientation, duration, offset
- ✅ **CameraSetView** - Instant camera positioning with destination, orientation
- ✅ **CameraFlyHome** - Animated return to home view with duration
- ✅ **CameraFlyToBoundingSphere** - Zoom to fit entity/target with offset
- ✅ **ClockReset** - Reset viewer clock to current time

### Data Sources
- ✅ **CzmlDataSource** - Load CZML data from URLs with automatic clock synchronization

### 3D Tiles
- ✅ **Google Photorealistic 3D Tiles** - Cesium3DTileset component support

### Coordinate & Math Utilities
- ✅ **Cartesian2** - 2D coordinates
- ✅ **Cartesian3** - 3D coordinates with helper functions:
  - `cartesian3_from_degrees(lon, lat, height)`
  - `cartesian3_from_degrees_array(coords)`
  - `cartesian3_from_degrees_array_heights(coords, heights)`
- ✅ **Rectangle** - Geographic rectangles with `from_degrees(west, south, east, north)`
- ✅ **PolygonHierarchy** - Polygons with holes support
- ✅ **HeadingPitchRoll** - Camera orientation
- ✅ **HeadingPitchRange** - Camera offset
- ✅ **BoundingSphere** - Bounding volumes
- ✅ **Math utilities** - `to_radians()`, `to_degrees()`

### Color System
- ✅ Predefined colors: `red`, `green`, `blue`, `yellow`, `cyan`, `magenta`, `white`, `black`, `purple`, `deepskyblue`
- ✅ Builder methods: `with_alpha()` for transparency
- ✅ Thread-safe wrappers for SSR compatibility

### Examples (4 working)
- ✅ **simple-viewer** - Basic 3D globe with Bing Maps imagery
- ✅ **with-entities** - Showcase of all graphics types and materials
- ✅ **czml-viewer** - CZML data loading with camera controls and time-based animations
- ✅ **with-server** - Full SSR example with Axum server

### Build System & Assets
- ✅ Trunk-based build system for examples
- ✅ Cesium asset sync script (`scripts/sync_cesium_assets.sh`)
- ✅ Environment token loading (`.env.local`)
- ✅ HTML structure documentation
- ✅ SSR support with conditional compilation

---

## 🔥 High Priority - Critical Next Features

### Event System (CRITICAL)
- [ ] Implement `cesium_events!` macro (mirror leptos-leaflet pattern)
- [ ] Mouse events: `click`, `double_click`, `move`, `wheel`
- [ ] Camera events: `move_start`, `move_end`, `changed`
- [ ] Entity picking and selection events
- [ ] Scene events: `render`, `pre_render`, `post_render`
- [ ] ScreenSpaceEventHandler integration

### Essential Graphics Components
- [ ] **BillboardGraphics** - Images/icons on entities
  - Props: `image`, `scale`, `rotation`, `pixel_offset`, `horizontal_origin`, `vertical_origin`
  - Support for dynamic images
- [ ] **LabelGraphics** - Text labels on entities
  - Props: `text`, `font`, `fill_color`, `outline_color`, `outline_width`, `style`, `pixel_offset`
  - Reactive text updates
- [ ] **ModelGraphics** - 3D models (GLTF/GLB)
  - Props: `uri`, `scale`, `minimum_pixel_size`, `maximum_scale`
  - Animation support
- [ ] **PathGraphics** - Entity trajectories over time
  - Props: `material`, `width`, `resolution`, `lead_time`, `trail_time`

### Entity Reactivity Enhancements
- [ ] Reactive `position` prop on Entity (Signal<Cartesian3>)
- [ ] Reactive `show` prop on Entity (Signal<bool>)
- [ ] Reactive `description` prop
- [ ] `id` prop for entity identification
- [ ] `availability` prop (TimeIntervalCollection for time-based visibility)

### GeoJSON Data Source
- [ ] **GeoJsonDataSource** component
  - Props: `url` or `data` (inline)
  - Styling options: `stroke`, `fill`, `marker_symbol`, `marker_color`, `marker_size`
  - `clamp_to_ground` option
  - `show` prop for visibility toggle
  - Loading state handling

---

## 📋 Medium Priority - Important Enhancements

### Additional Data Sources
- [ ] **KmlDataSource** component
  - Props: `url` or `kml` data
  - Camera behavior options
  - `show` prop
- [ ] **CustomDataSource** wrapper
  - Manual entity management
  - Add/remove entity methods

### Camera Enhancements
- [ ] `lookAt` method support
- [ ] Viewer tracking mode for entities
- [ ] Camera event handlers (expose camera change events)
- [ ] Reactive camera position tracking
- [ ] Home view customization

### Entity Property Expansion
- [ ] `orientation` prop (quaternion or heading/pitch/roll)
- [ ] `view_from` prop (default camera offset)
- [ ] Properties bag for time-dynamic properties

### Imagery Components
- [ ] **ImageryLayer** component
  - Props: `provider`, `alpha`, `brightness`, `contrast`, `hue`, `saturation`
  - `show`, `split_direction`
- [ ] **OpenStreetMapImageryProvider**
- [ ] **BingMapsImageryProvider**
  - Props: `api_key`, `map_style`
- [ ] **UrlTemplateImageryProvider**
  - Props: `url` template pattern
- [ ] **ArcGisMapServerImageryProvider**
- [ ] **WebMapServiceImageryProvider**

### Terrain Components
- [ ] **TerrainProvider** wrapper
- [ ] **CesiumTerrainProvider**
  - Props: `url`, `request_vertex_normals`, `request_water_mask`
- [ ] **EllipsoidTerrainProvider** (flat terrain)
- [ ] `createWorldTerrain` helper integration

---

## 🔮 Low Priority / Future Features

### Primitive Components
- [ ] **PointPrimitive**, **PointPrimitiveCollection**
  - Lower-level alternative to PointGraphics
  - Better performance for large point datasets
- [ ] **Polyline**, **PolylineCollection** (primitive versions)
  - Props: `positions`, `width`, `material`, `clamp_to_ground`
- [ ] **Billboard**, **BillboardCollection** (primitive versions)
  - Lower-level billboard rendering
- [ ] **Label**, **LabelCollection** (primitive versions)
  - Lower-level text rendering

### Widget Components
Currently, all widgets are controlled via ViewerContainer props. Consider extracting as standalone components:
- [ ] Timeline component (standalone time control)
- [ ] Animation component (play/pause, speed control)
- [ ] BaseLayerPicker component (imagery provider switching)
- [ ] Geocoder component (location search)
- [ ] NavigationCompass component

### Advanced 3D Features
- [ ] Particle systems (smoke, fire, rain effects)
- [ ] Post-processing effects (bloom, ambient occlusion, depth of field)
- [ ] Custom shaders and materials
- [ ] Scene rendering modes (wireframe, depth buffer visualization)

### Performance & Optimization
- [ ] `requestRenderMode` support (render on demand)
- [ ] Level-of-detail (LOD) controls
- [ ] Frustum culling configuration
- [ ] Memory management utilities

### Integrations
- [ ] Cesium Ion asset browser integration
- [ ] Terrain/imagery asset management
- [ ] Analytics integration
- [ ] VR/AR support (experimental)
- [ ] Web workers for data processing

### Developer Experience
- [ ] Error boundary components for Cesium errors
- [ ] Loading state components
- [ ] Debug overlays (FPS, draw calls, entity count)
- [ ] Performance monitoring hooks

---

## 📚 Documentation & Testing

### Documentation Needed
- [ ] Add example showcasing Google Photorealistic 3D Tiles
- [ ] Camera control tutorial
- [ ] Entity animation tutorial
- [ ] Time-based data visualization tutorial
- [ ] Terrain visualization example
- [ ] Custom imagery provider example
- [ ] API documentation for all components
- [ ] Migration guide from direct Cesium usage to leptos-cesium
- [ ] Performance best practices guide

### Testing Infrastructure
- [ ] Set up wasm-bindgen-test infrastructure
- [ ] SSR thread safety tests for all contexts
- [ ] Reactive property update tests
- [ ] Component lifecycle tests (mount/unmount)
- [ ] Cleanup/disposal tests (memory leak prevention)
- [ ] Browser compatibility tests (WebGL support)
- [ ] CI pipeline for examples

### Current Documentation Status
- ✅ README.md with feature list and getting started
- ✅ CLAUDE.md with implementation patterns and troubleshooting
- ✅ 4 working example apps
- ✅ Inline code documentation for core utilities
- [ ] Comprehensive API docs (rustdoc)
- [ ] Tutorial series
- [ ] Video walkthroughs

---

## 🔧 Infrastructure & Tooling

### Build System Improvements
- [ ] Integrate assets with `cargo leptos` metadata (`assets-dir`)
- [ ] Reduce reliance on sync script for production builds
- [ ] Make `CESIUM_BASE_URL` configuration more ergonomic
- [ ] Document asset serving for various deployment targets (Vercel, Netlify, etc.)

### Developer Tooling
- [ ] Add `Makefile.toml` (cargo-make) with standard targets:
  - `fmt` - Format code
  - `check` - Run clippy on all targets
  - `test` - Run test suite
  - `ci` - Full CI bundle (fmt + check + test)
- [ ] Optional pre-commit hook (`.githooks/pre-commit`)
- [ ] Improved error messages for common setup issues
- [ ] Development mode with hot reload

### Project Structure
- [ ] Consider extracting bindings to separate crate
- [ ] Evaluate code generation for repetitive bindings
- [ ] Optimize compile times (investigate feature flags)

---

## Key Implementation Notes

### Patterns Currently Used (Modern Leptos 0.8)
- ✅ `signal()` for signal creation (not `create_signal`)
- ✅ `Effect::new()` for effects (not `create_effect`)
- ✅ `provide_context()` / `use_context()` for contexts
- ✅ `#[prop(into)]` extensively for ergonomic APIs
- ✅ `Children` type for child components
- ✅ `on_cleanup()` for resource cleanup
- ✅ Thread-safe signals with `LocalStorage` for JS types
- ✅ SSR-safe conditional compilation (`#[cfg(target_arch = "wasm32")]`)
- ✅ Builder pattern for complex options (FlyToOptions, StripeOptions, etc.)

### Cesium-Specific Considerations
- **Asset Path Configuration**: Cesium requires Workers, Assets, Widgets directories
- **CESIUM_BASE_URL**: Must point to the Cesium bundle location
- **WebGL Requirement**: CesiumJS requires WebGL support
- **Clock/Time**: Many features depend on Clock for time-based animations
- **Coordinate Systems**: Multiple systems (Cartesian3, Cartographic, degrees)
- **Async Loading**: Terrain, imagery, data sources load asynchronously
- **Performance**: Consider requestRender mode for better performance
- **Memory Management**: Proper disposal of Cesium objects is critical

### Architecture Principles
1. **Declarative Component API** - Users compose scenes with components
2. **Reactive by Default** - Props accept signals for reactivity
3. **Context-Based Hierarchy** - Parent-child communication via contexts
4. **SSR-Safe** - All components compile and work in SSR environments
5. **Type-Safe** - Strong typing throughout, minimize JsValue exposure
6. **Builder APIs** - Complex options use builder pattern
7. **Cleanup Guaranteed** - All resources cleaned up on unmount

---

## Project Status Summary

**Overall Completion:** ~50% of core functionality

**Strengths:**
- ✅ Solid foundation with modern Leptos patterns
- ✅ Comprehensive graphics coverage (11 types)
- ✅ Excellent material system with builder APIs
- ✅ Complete camera control system
- ✅ SSR support is production-ready
- ✅ 3D Tiles support (Google Photorealistic)
- ✅ Working examples demonstrating capabilities

**Critical Gaps:**
- ❌ Event system (only placeholder exists)
- ❌ Billboard/Label/Model graphics
- ❌ Entity reactivity (position, show props)
- ❌ GeoJSON data source

**Next Milestone Goals:**
1. Implement event system for interactivity
2. Add Billboard, Label, Model graphics
3. Make Entity properties reactive
4. Add GeoJSON support
5. Create comprehensive testing suite

---

## Contributing

Contributions are welcome! Priority areas:
1. Event system implementation
2. Billboard/Label/Model graphics
3. GeoJSON data source
4. Testing infrastructure
5. Documentation and examples

See individual sections above for specific tasks and implementation details.
