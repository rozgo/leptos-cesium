//! Cesium Ion API bindings for token management.

/// Sets the Cesium Ion default access token.
///
/// This must be called before creating a Cesium Viewer instance.
#[cfg(target_arch = "wasm32")]
pub use crate::bindings::globals::set_ion_default_access_token as set_default_access_token;
