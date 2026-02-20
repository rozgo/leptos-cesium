# Camera Control Example

Interactive camera parity demo for `leptos-cesium`.

## Features Demonstrated

- `CameraSetView` with multiple destination types:
  - `CameraDestination::Degrees`
  - `CameraDestination::Rectangle`
- `CameraFlyTo` with optional orientation/duration
- `CameraFlyHome` trigger-based reset
- `CameraMove` and `CameraZoom` trigger actions
- `CameraController` toggles (`enable_inputs`, `enable_collision_detection`)

## Run

```bash
cd examples/camera-control
trunk clean
NO_COLOR=false trunk serve --release --open
```

## Build Check

```bash
cd examples/camera-control
trunk build
```

## Notes

- Startup is intentionally a baseline: Cesium default home view with no auto camera commands.
- `CameraController` is mounted only when you enable it via the "Mount CameraController" toggle.
- This example intentionally uses optional props only where Cesium options are optional.
- Cesium assets are loaded from CDN via `index.html`.
