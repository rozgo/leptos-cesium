//! Cesium SceneTransforms helpers.

use crate::bindings::{Cartesian2, Cartesian3, Scene};

/// Reflection-backed access to Cesium `SceneTransforms`.
pub struct SceneTransforms;

impl SceneTransforms {
    /// Convert a world-space position into viewer window coordinates.
    #[cfg(target_arch = "wasm32")]
    pub fn world_to_window_coordinates(scene: &Scene, position: &Cartesian3) -> Option<Cartesian2> {
        use js_sys::{Function, Reflect, global};
        use wasm_bindgen::{JsCast, JsValue};

        let cesium = Reflect::get(&global(), &JsValue::from_str("Cesium")).ok()?;
        let scene_transforms = Reflect::get(&cesium, &JsValue::from_str("SceneTransforms")).ok()?;
        let world_to_window = Reflect::get(
            &scene_transforms,
            &JsValue::from_str("worldToWindowCoordinates"),
        )
        .ok()?;
        let world_to_window: Function = world_to_window.dyn_into().ok()?;

        let result = world_to_window
            .call2(&scene_transforms, scene.as_ref(), position.as_ref())
            .ok()?;

        if result.is_null() || result.is_undefined() {
            None
        } else {
            result.dyn_into().ok()
        }
    }

    /// Stub implementation for non-WASM builds.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn world_to_window_coordinates(scene: &Scene, position: &Cartesian3) -> Option<Cartesian2> {
        let _ = (scene, position);
        None
    }
}
