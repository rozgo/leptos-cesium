//! Conversions from standard Rust types to Cesium JS types.
//!
//! These conversions are only available in WASM builds and should be called
//! inside Effects (which only run on the client).

#[cfg(target_arch = "wasm32")]
use crate::bindings::{Cartesian3, Color, PolygonHierarchy, Rectangle};

// ============================================================================
// geo-types conversions
// ============================================================================

/// Convert a 2D geographic point to Cesium Cartesian3 (height = 0).
#[cfg(target_arch = "wasm32")]
impl From<geo_types::Point<f64>> for Cartesian3 {
    fn from(point: geo_types::Point<f64>) -> Self {
        Cartesian3::from_degrees(point.x(), point.y(), 0.0)
    }
}

/// Convert a geographic rectangle (bounding box) to Cesium Rectangle.
#[cfg(target_arch = "wasm32")]
impl From<geo_types::Rect<f64>> for Rectangle {
    fn from(rect: geo_types::Rect<f64>) -> Self {
        let min = rect.min();
        let max = rect.max();
        Rectangle::from_degrees(min.x, min.y, max.x, max.y)
    }
}

/// Convert a LineString to an array of Cartesian3 positions.
#[cfg(target_arch = "wasm32")]
pub fn linestring_to_cartesian_array(linestring: &geo_types::LineString<f64>) -> js_sys::Array {
    let coords: Vec<f64> = linestring.coords().flat_map(|c| [c.x, c.y]).collect();
    Cartesian3::from_degrees_array(&coords)
}

/// Convert a LineString with heights to an array of Cartesian3 positions.
#[cfg(target_arch = "wasm32")]
pub fn linestring_to_cartesian_array_with_heights(
    linestring: &geo_types::LineString<f64>,
    heights: &[f64],
) -> js_sys::Array {
    let coords: Vec<f64> = linestring
        .coords()
        .zip(heights.iter().chain(std::iter::repeat(&0.0)))
        .flat_map(|(c, h)| [c.x, c.y, *h])
        .collect();
    Cartesian3::from_degrees_array_heights(&coords)
}

/// Convert a geo_types::Polygon to Cesium PolygonHierarchy.
#[cfg(target_arch = "wasm32")]
pub fn polygon_to_hierarchy(polygon: &geo_types::Polygon<f64>) -> PolygonHierarchy {
    use wasm_bindgen::JsValue;

    // Convert exterior ring
    let exterior_positions = linestring_to_cartesian_array(polygon.exterior());

    // Convert holes
    let holes = polygon.interiors();
    if holes.is_empty() {
        PolygonHierarchy::new_simple(&JsValue::from(exterior_positions))
    } else {
        let holes_array = js_sys::Array::new();
        for hole in holes {
            let hole_positions = linestring_to_cartesian_array(hole);
            let hole_hierarchy = PolygonHierarchy::new_simple(&JsValue::from(hole_positions));
            holes_array.push(&JsValue::from(hole_hierarchy));
        }
        PolygonHierarchy::new(
            &JsValue::from(exterior_positions),
            &JsValue::from(holes_array),
        )
    }
}

// ============================================================================
// glam conversions
// ============================================================================

/// Convert a DVec3 to Cesium Cartesian3.
/// Interpretation: x = longitude, y = latitude, z = height (in degrees/meters).
#[cfg(target_arch = "wasm32")]
impl From<glam::DVec3> for Cartesian3 {
    fn from(v: glam::DVec3) -> Self {
        Cartesian3::from_degrees(v.x, v.y, v.z)
    }
}

// ============================================================================
// palette conversions
// ============================================================================

/// Convert an SRGBA color to Cesium Color.
#[cfg(target_arch = "wasm32")]
impl From<palette::Srgba<f32>> for Color {
    fn from(c: palette::Srgba<f32>) -> Self {
        Color::new(c.red as f64, c.green as f64, c.blue as f64, c.alpha as f64)
    }
}

/// Convert an SRGBA color (f64) to Cesium Color.
#[cfg(target_arch = "wasm32")]
impl From<palette::Srgba<f64>> for Color {
    fn from(c: palette::Srgba<f64>) -> Self {
        Color::new(c.red, c.green, c.blue, c.alpha)
    }
}

/// Convert an SRGB color (no alpha) to Cesium Color with alpha = 1.0.
#[cfg(target_arch = "wasm32")]
impl From<palette::Srgb<f32>> for Color {
    fn from(c: palette::Srgb<f32>) -> Self {
        Color::new(c.red as f64, c.green as f64, c.blue as f64, 1.0)
    }
}

// ============================================================================
// Dimensional conversions (x, y, z in meters, NOT geographic)
// ============================================================================

/// Convert DVec3 to Cartesian3 for dimensional values (x, y, z in meters).
/// Use this for radii, dimensions, shape sizes - NOT for geographic positions.
#[cfg(target_arch = "wasm32")]
pub fn dvec3_to_cartesian_dimensions(v: glam::DVec3) -> Cartesian3 {
    Cartesian3::new(v.x, v.y, v.z)
}
