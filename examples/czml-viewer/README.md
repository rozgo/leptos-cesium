# CZML Viewer Example

This example demonstrates loading and displaying CZML (Cesium Language) data sources with camera controls.

## Features

- Load CZML data files dynamically
- Control camera position and orientation
- Timeline and animation controls enabled
- Remove/reset data sources

## Setup

1. Place your CZML files in the `public/SampleData` directory:
   - `SampleData/simple.czml` - Satellite data example
   - `SampleData/Vehicle.czml` - Vehicle tracking example

2. Ensure you have a Cesium Ion token in `.env.local` at the project root:
   ```
   CESIUM_ION_TOKEN=your_token_here
   ```

## Running

From this directory:

```bash
trunk serve
```

Then open http://localhost:8080

## Usage

- **Satellites** button: Loads `SampleData/simple.czml` and flies camera to home position
- **Vehicle** button: Loads `SampleData/Vehicle.czml` and sets a specific camera view
- **Reset** button: Removes all loaded data sources

## CZML Data Files

You can download sample CZML files from:
- [Cesium Sample Data](https://github.com/CesiumGS/cesium/tree/main/Apps/SampleData)

Or create your own CZML following the [CZML specification](https://github.com/AnalyticalGraphicsInc/czml-writer/wiki/CZML-Guide).

## Code Structure

The example demonstrates **declarative CZML loading and camera control**:

### Declarative Components Used:

- **`<CzmlDataSource url=... />`** - Declaratively loads CZML from a URL
  - When the URL signal changes, the old data source is cleared and the new one is loaded
  - Automatically cleans up data sources when the component unmounts

- **`<CameraFlyHome trigger=... />`** - Flies camera to home position when trigger signal updates
  - Uses reactive signals to trigger camera movements declaratively

- **`<CameraSetView destination=... heading=... />`** - Sets camera view position and orientation
  - Declaratively controls camera position via props
  - Updates when any prop changes

### Declarative Pattern:

```rust
// Signals control state
let (czml_url, set_czml_url) = signal("".to_string());
let (show_vehicle_camera, set_show_vehicle_camera) = signal(false);

view! {
    <ViewerContainer>
        // Conditionally render components based on state
        {move || (!czml_url.get().is_empty()).then(|| view! {
            <CzmlDataSource url=czml_url.get() />
        })}

        {move || show_vehicle_camera.get().then(|| view! {
            <CameraSetView destination=... heading=... />
        })}
    </ViewerContainer>
}
```

This follows the same declarative patterns as other leptos-cesium components like `<Entity>` and `<PointGraphics>`.
