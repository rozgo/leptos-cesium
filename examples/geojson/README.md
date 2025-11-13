# GeoJSON Viewer Example

This example demonstrates loading and styling GeoJSON data with `leptos-cesium`.

## Features Demonstrated

- **Loading GeoJSON from URLs** - Declarative data loading with the `GeoJsonDataSource` component
- **Dynamic Styling** - Custom colors and styles for polygons, lines, and points
- **Reactive Layer Switching** - Toggle between different GeoJSON datasets
- **Styling Options**:
  - Polygon fill and stroke customization
  - Point marker styling (color and size)
  - Polyline stroke styling
  - Clamp to ground option
- **Camera Control** - Initial view positioning

## Sample Data

The example includes three GeoJSON datasets:

- **Countries** (`countries.geojson`) - Simplified country polygons with custom fill and stroke
- **Cities** (`cities.geojson`) - Major world cities as point features with custom markers
- **Rivers** (`rivers.geojson`) - Major rivers as polyline features with custom stroke

## Running the Example

### Prerequisites

1. Ensure Cesium assets are synced:
   ```bash
   # From repository root
   ./scripts/sync_cesium_assets.sh
   ```

2. Set up your Cesium Ion token:
   ```bash
   # Copy .env.example to .env.local and add your token
   cp .env.example .env.local
   # Edit .env.local and add: CESIUM_ION_TOKEN=your_token_here
   ```

### Run with Trunk

```bash
cd examples/geojson
trunk serve --open
```

The example will open in your browser at http://localhost:8080

## UI Controls

- **Countries** - Load simplified country boundaries with blue stroke and cyan fill
- **Cities** - Load major city points with red markers
- **Rivers** - Load major rivers with blue polylines
- **Clear** - Remove all GeoJSON data from the viewer
- **Custom Styling** - Toggle between custom styling and Cesium defaults
- **Clamp to Ground** - Enable/disable terrain clamping for features

## Code Highlights

### Loading GeoJSON with Styling

```rust
<GeoJsonDataSource
    url="SampleData/countries.geojson"
    stroke=Color::blue()
    stroke_width=2.0
    fill=Color::cyan().with_alpha(0.3)
    clamp_to_ground=Some(false)
/>
```

### Dynamic Styling Based on Data Type

```rust
{move || {
    let url = geojson_url.get();
    let use_style = use_custom_style.get();

    (!url.is_empty()).then(|| {
        if use_style {
            if url.contains("countries") {
                view! {
                    <GeoJsonDataSource
                        url=url
                        stroke=Color::blue()
                        fill=Color::cyan().with_alpha(0.3)
                    />
                }
            } else if url.contains("cities") {
                view! {
                    <GeoJsonDataSource
                        url=url
                        marker_color=Color::red()
                        marker_size=24.0
                    />
                }
            } else {
                view! {
                    <GeoJsonDataSource
                        url=url
                        stroke=Color::deepskyblue()
                        stroke_width=3.0
                    />
                }
            }
        } else {
            // Default styling
            view! { <GeoJsonDataSource url=url /> }
        }
    })
}}
```

## Component API

The `GeoJsonDataSource` component supports these props:

- `url` - URL to the GeoJSON file (required)
- `stroke` - Stroke color for polylines and polygon outlines (default: BLACK)
- `stroke_width` - Stroke width in pixels (default: 2.0)
- `fill` - Fill color for polygons (default: YELLOW)
- `marker_color` - Color for point markers (default: ROYALBLUE)
- `marker_size` - Marker size in pixels (default: 48)
- `marker_symbol` - Maki identifier or single character for markers
- `clamp_to_ground` - Whether to clamp features to terrain (default: false)
- `clear_existing` - Clear other data sources before loading (default: true)

## Creating Custom GeoJSON

The sample data files follow the GeoJSON specification. You can create your own:

### Point Features (Cities)
```json
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "properties": { "name": "City Name" },
      "geometry": {
        "type": "Point",
        "coordinates": [longitude, latitude]
      }
    }
  ]
}
```

### Polygon Features (Countries)
```json
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "properties": { "name": "Country Name" },
      "geometry": {
        "type": "Polygon",
        "coordinates": [[
          [lon1, lat1], [lon2, lat2], ..., [lon1, lat1]
        ]]
      }
    }
  ]
}
```

### LineString Features (Rivers)
```json
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "properties": { "name": "River Name" },
      "geometry": {
        "type": "LineString",
        "coordinates": [
          [lon1, lat1], [lon2, lat2], [lon3, lat3]
        ]
      }
    }
  ]
}
```

## Learn More

- [GeoJSON Specification](https://geojson.org/)
- [Cesium GeoJSON Documentation](https://cesium.com/learn/cesiumjs/ref-doc/GeoJsonDataSource.html)
- [leptos-cesium Documentation](../../README.md)
