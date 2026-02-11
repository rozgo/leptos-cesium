# Google Photorealistic 3D Tiles Example

This example demonstrates loading Google's photorealistic 3D Tiles using `leptos-cesium`.

## Features Demonstrated

- `GooglePhotorealistic3DTiles` component usage
- Cesium Ion token-based access
- Initial camera fly-to over San Francisco
- Viewer UI trimming (`animation=false`, `timeline=false`) for a focused 3D tiles scene

## Prerequisites

1. Configure a Cesium Ion token in `.env.local` at the repository root:

```bash
cp .env.example .env.local
# Edit .env.local and set CESIUM_ION_TOKEN=your_token_here
```

2. Ensure your token has access to Google Photorealistic 3D Tiles in Cesium Ion.

## Run

```bash
cd examples/google-3d-tiles
trunk serve --open
```

## Build Check

```bash
cd examples/google-3d-tiles
trunk build
```

## Notes

- The example uses Cesium CDN assets from `index.html` (no local `public/Cesium` sync step).
- By default, `GooglePhotorealistic3DTiles` uses the Cesium Ion-backed dataset.
- You can pass `google_api_key` to the component for direct Google key usage when needed.
