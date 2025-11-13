//! Cesium3DTileset bindings for loading 3D tiles.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Cesium3DTileset for loading 3D tile data
    #[wasm_bindgen(js_namespace = Cesium, js_name = Cesium3DTileset)]
    pub type Cesium3DTileset;

    #[wasm_bindgen(method, js_name = destroy)]
    pub fn destroy(this: &Cesium3DTileset);

    /// Create Google Photorealistic 3D Tiles tileset
    ///
    /// # Parameters
    /// - `api_options`: Optional configuration with `onlyUsingWithGoogleGeocoder` and `key`
    /// - `tileset_options`: Optional Cesium3DTileset constructor options
    ///
    /// # JavaScript Signature
    /// ```js
    /// async function createGooglePhotorealistic3DTileset(apiOptions, tilesetOptions)
    /// ```
    #[wasm_bindgen(js_namespace = Cesium, js_name = createGooglePhotorealistic3DTileset)]
    pub fn create_google_photorealistic_3d_tileset(
        api_options: &JsValue,
        tileset_options: &JsValue,
    ) -> js_sys::Promise;
}

/// Options for createGooglePhotorealistic3DTileset API options parameter
#[cfg(target_arch = "wasm32")]
pub struct GooglePhotorealistic3DTilesApiOptions {
    /// Your Google Maps API key. If not provided, uses Cesium Ion asset 2275207
    pub key: Option<String>,
    /// Confirmation that tileset will only be used with Google geocoder
    pub only_using_with_google_geocoder: bool,
}

#[cfg(target_arch = "wasm32")]
impl Default for GooglePhotorealistic3DTilesApiOptions {
    fn default() -> Self {
        Self {
            key: None,
            only_using_with_google_geocoder: true,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl GooglePhotorealistic3DTilesApiOptions {
    pub fn to_js_value(&self) -> JsValue {
        let obj = js_sys::Object::new();

        if let Some(key) = &self.key {
            let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("key"), &JsValue::from_str(key));
        }

        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("onlyUsingWithGoogleGeocoder"),
            &JsValue::from_bool(self.only_using_with_google_geocoder),
        );

        obj.into()
    }
}

/// Options for Cesium3DTileset constructor (subset of most commonly used options)
#[cfg(target_arch = "wasm32")]
pub struct Cesium3DTilesetOptions {
    /// Size in bytes for tile cache. Default in Google tiles: 1536 * 1024 * 1024
    pub cache_bytes: Option<u32>,
    /// Maximum cache overflow in bytes. Default in Google tiles: 1024 * 1024 * 1024
    pub maximum_cache_overflow_bytes: Option<u32>,
    /// Enable collision detection. Default in Google tiles: true
    pub enable_collision: Option<bool>,
    /// Maximum screen space error for LOD. Default: 16
    pub maximum_screen_space_error: Option<f64>,
}

#[cfg(target_arch = "wasm32")]
impl Default for Cesium3DTilesetOptions {
    fn default() -> Self {
        Self {
            cache_bytes: Some(1536 * 1024 * 1024),
            maximum_cache_overflow_bytes: Some(1024 * 1024 * 1024),
            enable_collision: Some(true),
            maximum_screen_space_error: None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Cesium3DTilesetOptions {
    pub fn to_js_value(&self) -> JsValue {
        let obj = js_sys::Object::new();

        if let Some(cache_bytes) = self.cache_bytes {
            let _ = js_sys::Reflect::set(
                &obj,
                &JsValue::from_str("cacheBytes"),
                &JsValue::from_f64(cache_bytes as f64),
            );
        }

        if let Some(overflow) = self.maximum_cache_overflow_bytes {
            let _ = js_sys::Reflect::set(
                &obj,
                &JsValue::from_str("maximumCacheOverflowBytes"),
                &JsValue::from_f64(overflow as f64),
            );
        }

        if let Some(collision) = self.enable_collision {
            let _ = js_sys::Reflect::set(
                &obj,
                &JsValue::from_str("enableCollision"),
                &JsValue::from_bool(collision),
            );
        }

        if let Some(error) = self.maximum_screen_space_error {
            let _ = js_sys::Reflect::set(
                &obj,
                &JsValue::from_str("maximumScreenSpaceError"),
                &JsValue::from_f64(error),
            );
        }

        obj.into()
    }
}
