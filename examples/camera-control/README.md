# Camera Control Example (Scaffold)

This directory is reserved for a dedicated camera-controls example.  
At the moment it is a scaffold (no `Cargo.toml` or `src/main.rs` yet), so it is not directly runnable.

## Intended Coverage

When implemented, this example should showcase:

- `CameraSetView`
- `CameraFlyTo`
- `CameraFlyHome`
- `CameraFlyToBoundingSphere`
- optional camera orientation and timing options

## Current Alternatives

You can already see camera control components in active examples:

- `examples/czml-viewer` (camera moves tied to CZML workflow)
- `examples/google-3d-tiles` (initial fly-to on load)

## Target Pattern

The dedicated example should look roughly like:

```rust
view! {
    <ViewerContainer ion_token=ion_token>
        <CameraSetView destination=DVec3::new(-75.0, 40.0, 1000.0) />
        <CameraFlyTo destination=DVec3::new(-122.4, 37.8, 5000.0) duration=3.0 />
        <CameraFlyHome duration=2.0 />
    </ViewerContainer>
}
```

## Notes

- Cesium assets in this repository are served from CDN via each example's `index.html`.
- No local `public/Cesium` sync workflow is used.
