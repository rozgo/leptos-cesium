//! Convenience re-exports for crate users.

// Re-export core utilities
pub use crate::core::*;

// Re-export components (these are what users primarily interact with)
pub use crate::components::*;

// Re-export common bindings (selective to avoid conflicts)
pub use crate::bindings::{
    cartesian3_from_degrees, cartesian3_from_degrees_array, cartesian3_from_degrees_array_heights,
    Cartesian2, Cartesian3, Color, Material, PolygonHierarchy, Rectangle, StripeMaterialProperty,
    StripeOptions, Viewer,
};

// Re-export math utilities
pub use crate::bindings::math::{to_degrees, to_radians};

// Re-export cesium namespace
pub use crate::cesium;
