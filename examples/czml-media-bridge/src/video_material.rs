use js_sys::{Function, Reflect};
use leptos::wasm_bindgen::{JsCast, JsValue};
use leptos::web_sys;
use leptos_cesium::prelude::{ImageMaterialPropertyBuilder, MediaSource};

use crate::scenario::{MEDIA_VIDEO_RECT_ENTITY_ID, video_uri};

pub fn apply_video_material_from_czml(data_source_js: &JsValue) -> Result<(), String> {
    let data_source = data_source_js
        .clone()
        .dyn_into::<leptos_cesium::bindings::CzmlDataSource>()
        .map_err(|_| "Expected Cesium CzmlDataSource handle".to_string())?;

    let entities_js = JsValue::from(data_source.entities());
    let get_by_id = Reflect::get(&entities_js, &JsValue::from_str("getById"))
        .map_err(js_error_to_string)?
        .dyn_into::<Function>()
        .map_err(|_| "EntityCollection.getById is not callable".to_string())?;

    let entity = get_by_id
        .call1(&entities_js, &JsValue::from_str(MEDIA_VIDEO_RECT_ENTITY_ID))
        .map_err(js_error_to_string)?;

    if entity.is_null() || entity.is_undefined() {
        return Err(format!(
            "CZML entity '{}' not found",
            MEDIA_VIDEO_RECT_ENTITY_ID
        ));
    }

    let rectangle =
        Reflect::get(&entity, &JsValue::from_str("rectangle")).map_err(js_error_to_string)?;
    if rectangle.is_null() || rectangle.is_undefined() {
        return Err(format!(
            "Entity '{}' has no rectangle graphics",
            MEDIA_VIDEO_RECT_ENTITY_ID
        ));
    }

    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or_else(|| "Window/document not available".to_string())?;

    let video = document
        .create_element("video")
        .map_err(js_error_to_string)?
        .dyn_into::<web_sys::HtmlVideoElement>()
        .map_err(|_| "Failed to create HTMLVideoElement".to_string())?;

    video.set_autoplay(true);
    video.set_loop(true);
    video.set_muted(true);
    video.set_cross_origin(Some("anonymous"));
    let _ = video.set_attribute("playsinline", "");
    let _ = video.set_attribute("crossorigin", "anonymous");
    video.set_src(video_uri());
    video.load();
    let _ = video.play();

    let image_material = ImageMaterialPropertyBuilder::new()
        .image(MediaSource::HtmlVideo(video))
        .build();

    Reflect::set(
        &rectangle,
        &JsValue::from_str("material"),
        &JsValue::from(image_material),
    )
    .map_err(js_error_to_string)?;

    Ok(())
}

pub fn js_error_to_string(error: JsValue) -> String {
    if let Some(message) = error.as_string() {
        return message;
    }
    if let Ok(text) = js_sys::JSON::stringify(&error)
        && let Some(value) = text.as_string()
        && !value.is_empty()
    {
        return value;
    }
    format!("{:?}", error)
}
