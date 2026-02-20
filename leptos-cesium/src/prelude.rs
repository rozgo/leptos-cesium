//! Convenience re-exports for crate users.

// Re-export standard Rust types for component props
pub use geo_types::{Coord, LineString, Point, Polygon, Rect};
pub use glam::DVec3;
pub use palette::Srgba;

// Re-export core utilities
pub use crate::core::*;

// Re-export components (these are what users primarily interact with)
pub use crate::components::*;

// Re-export common bindings (selective to avoid conflicts)
pub use crate::bindings::{
    BoundingSphere, Camera, Cartesian2, Cartesian3, CheckerboardMaterialProperty,
    CheckerboardOptions, ClockRangeMode, ClockStepMode, Color, CzmlDataSource, DataSource,
    DataSourceClock, DataSourceCollection, Ellipsoid, EntityCluster, HeadingPitchRange,
    HeadingPitchRoll, Material, Matrix4, PolygonHierarchy, PolylineGlowMaterialProperty,
    PolylineGlowOptions, Ray, Rectangle, ScreenSpaceCameraController, StripeMaterialProperty,
    StripeOptions, Viewer, ViewerFlyToOptions,
};

// Re-export math utilities
pub use crate::bindings::math::{to_degrees, to_radians};

// Re-export cesium namespace
pub use crate::cesium;
