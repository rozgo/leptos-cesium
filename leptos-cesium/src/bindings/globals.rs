//! Bindings for Cesium global helpers.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
pub fn set_base_url(base_url: &str) {
    use js_sys::{global, Function, Reflect};
    use web_sys::console;

    let global = global();
    let Some(cesium) = Reflect::get(&global, &JsValue::from_str("Cesium")).ok() else {
        console::warn_1(&JsValue::from_str(
            "Cesium global unavailable; cannot set base URL",
        ));
        return;
    };
    let Some(build_module_url) =
        Reflect::get(&cesium, &JsValue::from_str("buildModuleUrl")).ok()
    else {
        console::warn_1(&JsValue::from_str(
            "Cesium.buildModuleUrl missing; cannot set base URL",
        ));
        return;
    };
    let Some(set_base) =
        Reflect::get(&build_module_url, &JsValue::from_str("setBaseUrl")).ok()
    else {
        console::warn_1(&JsValue::from_str(
            "Cesium.buildModuleUrl.setBaseUrl missing; cannot set base URL",
        ));
        return;
    };
    let Ok(set_base_fn) = set_base.dyn_into::<Function>() else {
        console::warn_1(&JsValue::from_str(
            "Cesium.buildModuleUrl.setBaseUrl is not a function",
        ));
        return;
    };
    if let Err(err) = set_base_fn.call1(&build_module_url, &JsValue::from_str(base_url)) {
        console::warn_1(&JsValue::from(format!(
            "Failed calling setBaseUrl: {:?}",
            err
        )));
    }
}

#[cfg(target_arch = "wasm32")]
pub fn set_ion_default_access_token(token: &str) {
    use js_sys::{global, Reflect};

    let global = global();
    if let Ok(cesium) = Reflect::get(&global, &JsValue::from_str("Cesium")) {
        if let Ok(ion) = Reflect::get(&cesium, &JsValue::from_str("Ion")) {
            let _ = Reflect::set(&ion, &JsValue::from_str("defaultAccessToken"), &JsValue::from_str(token));
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub fn set_base_url(_base_url: &str) {}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub fn set_ion_default_access_token(_token: &str) {}
