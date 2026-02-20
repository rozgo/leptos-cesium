# CZML Viewer Example

This example demonstrates loading and displaying CZML (Cesium Language) data sources with viewer-level focus and clock tracking controls.

## Features

- Load CZML data files dynamically
- Focus the loaded CZML data source with `Viewer.flyTo`
- Track CZML timeline with `viewer.clockTrackedDataSource`
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

- **Satellites** button: Loads `SampleData/simple.czml` and focuses that loaded data source
- **Vehicle** button: Loads `SampleData/Vehicle.czml` and focuses that loaded data source
- **Reset** button: Removes loaded data sources, clears clock tracking, and flies home

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

- **`<ViewerFlyToTarget trigger=... target=... />`** - Triggers `Viewer.flyTo(target)` for loaded CZML data source objects
  - Accepts target objects (data source/entity/JsValue), not only coordinate destinations

- **`<ViewerSetClockTrackedDataSource trigger=... data_source=... />`** - Triggers `viewer.clockTrackedDataSource = ...`
  - Keeps timeline UI aligned with the currently loaded CZML data source clock

- **`<CameraFlyHome trigger=... />`** - Flies camera to home position on reset

### Declarative Pattern:

```rust
// Signals control state
let (czml_url, set_czml_url) = signal("".to_string());
let loaded_target = JsRwSignal::new_local(None::<ViewerTarget>);
let loaded_data_source = JsRwSignal::new_local(None::<DataSource>);

view! {
    <ViewerContainer>
        // Conditionally load CZML
        {move || (!czml_url.get().is_empty()).then(|| view! {
            <CzmlDataSource
                url=czml_url.get()
                on_loaded=on_loaded
            />
        })}

        <ViewerFlyToTarget trigger=focus_loaded_trigger target=loaded_target />
        <ViewerSetClockTrackedDataSource trigger=track_clock_trigger data_source=loaded_data_source />
    </ViewerContainer>
}
```

This follows the same declarative patterns as other leptos-cesium components like `<Entity>` and `<PointGraphics>`.
