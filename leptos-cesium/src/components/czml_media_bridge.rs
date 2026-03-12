//! Internal helpers for applying image/video media metadata from flattened CZML custom properties.

use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

use crate::bindings::MediaSource;

#[cfg(target_arch = "wasm32")]
use std::collections::{HashMap, HashSet};

#[cfg(target_arch = "wasm32")]
use crate::bindings::{CzmlDataSource as CesiumCzmlDataSource, ImageMaterialPropertyBuilder};
#[cfg(target_arch = "wasm32")]
use crate::core::{JsStoredValue, RequestGate};
#[cfg(target_arch = "wasm32")]
use js_sys::{Array, Function, Reflect};
use url::Url;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

/// Media kind for CZML media metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CzmlMediaKind {
    Image,
    Video,
}

/// Target graphic for CZML media metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CzmlMediaTarget {
    Billboard,
    Rectangle,
    Polygon,
}

impl CzmlMediaTarget {
    #[cfg(target_arch = "wasm32")]
    fn as_graphic_property(self) -> &'static str {
        match self {
            CzmlMediaTarget::Billboard => "billboard",
            CzmlMediaTarget::Rectangle => "rectangle",
            CzmlMediaTarget::Polygon => "polygon",
        }
    }
}

/// Parsed `properties.media_*` descriptor for one entity.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CzmlMediaDescriptor {
    pub entity_id: String,
    pub kind: CzmlMediaKind,
    pub target: CzmlMediaTarget,
    pub uri: String,
    pub autoplay: bool,
    pub loop_video: bool,
    pub muted: bool,
    pub plays_inline: bool,
    pub cross_origin: Option<String>,
}

/// Structured media error payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CzmlMediaError {
    pub entity_id: Option<String>,
    pub reason: String,
}

impl CzmlMediaError {
    #[cfg(target_arch = "wasm32")]
    fn new(entity_id: Option<String>, reason: impl Into<String>) -> Self {
        Self {
            entity_id,
            reason: reason.into(),
        }
    }
}

#[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct MediaCacheKey {
    entity_id: String,
    kind: CzmlMediaKind,
    target: CzmlMediaTarget,
    uri: String,
    autoplay: bool,
    loop_video: bool,
    muted: bool,
    plays_inline: bool,
    cross_origin: Option<String>,
}

impl MediaCacheKey {
    #[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
    fn from_descriptor(descriptor: &CzmlMediaDescriptor) -> Self {
        Self {
            entity_id: descriptor.entity_id.clone(),
            kind: descriptor.kind.clone(),
            target: descriptor.target,
            uri: descriptor.uri.clone(),
            autoplay: descriptor.autoplay,
            loop_video: descriptor.loop_video,
            muted: descriptor.muted,
            plays_inline: descriptor.plays_inline,
            cross_origin: descriptor.cross_origin.clone(),
        }
    }
}

/// Optional callback for custom URI -> media resolution.
pub type CzmlMediaResolver = Callback<CzmlMediaDescriptor, Option<MediaSource>>;

#[cfg(target_arch = "wasm32")]
pub(crate) type CzmlMediaCache = JsStoredValue<HashMap<MediaCacheKey, MediaSource>>;

#[cfg(target_arch = "wasm32")]
pub(crate) fn new_media_cache() -> CzmlMediaCache {
    JsStoredValue::new_local(HashMap::<MediaCacheKey, MediaSource>::new())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn reconcile_data_source_media(
    source_js: JsValue,
    request_id: u64,
    media_cache: CzmlMediaCache,
    request_gate: RequestGate,
    reapply_cached_bindings: bool,
    resolve_media: Option<CzmlMediaResolver>,
    on_error: Option<Callback<CzmlMediaError>>,
    base_uri: Option<String>,
) {
    if request_gate.is_stale(request_id) {
        return;
    }

    let data_source = match source_js.dyn_into::<CesiumCzmlDataSource>() {
        Ok(value) => value,
        Err(_) => {
            emit_media_error(
                on_error,
                None,
                "CzmlDataSource media handling expected a Cesium CzmlDataSource handle",
            );
            return;
        }
    };

    let entities_array = match entity_values_array(&data_source) {
        Some(values) => values,
        None => {
            emit_media_error(
                on_error,
                None,
                "Unable to access dataSource.entities.values",
            );
            return;
        }
    };

    let mut active_keys = HashSet::<MediaCacheKey>::new();

    for entity in entities_array.iter() {
        if request_gate.is_stale(request_id) {
            return;
        }

        let mut descriptor = match parse_media_descriptor(&entity) {
            Ok(Some(value)) => value,
            Ok(None) => continue,
            Err(error) => {
                let entity_id = entity_id(&entity);
                emit_media_error(on_error, entity_id, error);
                continue;
            }
        };

        descriptor.uri = match normalize_media_uri(&descriptor.uri, base_uri.as_deref()) {
            Ok(value) => value,
            Err(error) => {
                emit_media_error(on_error, Some(descriptor.entity_id.clone()), error);
                continue;
            }
        };

        let cache_key = MediaCacheKey::from_descriptor(&descriptor);
        active_keys.insert(cache_key.clone());

        let cached_media_source = media_cache.with_value(|cache| cache.get(&cache_key).cloned());
        let reused_cached_binding = cached_media_source.is_some();
        let media_source = match cached_media_source {
            Some(value) => Some(value),
            None => {
                let resolved = resolve_media.and_then(|resolver| resolver.run(descriptor.clone()));
                match resolved {
                    Some(value) => Some(value),
                    None => match default_media_source(
                        &descriptor,
                        request_id,
                        request_gate.clone(),
                        on_error,
                    ) {
                        Ok(value) => Some(value),
                        Err(error) => {
                            emit_media_error(on_error, Some(descriptor.entity_id.clone()), error);
                            None
                        }
                    },
                }
            }
        };

        let Some(media_source) = media_source else {
            continue;
        };

        media_cache.update_value(|cache| {
            cache.insert(cache_key, media_source.clone());
        });

        if (reapply_cached_bindings || !reused_cached_binding)
            && let Err(error) = apply_media_to_entity(&entity, &descriptor, &media_source)
        {
            emit_media_error(on_error, Some(descriptor.entity_id.clone()), error);
        }
    }

    // Release cached media for entities/descriptors no longer present in current data.
    media_cache.update_value(|cache| {
        cache.retain(|key, source| {
            if active_keys.contains(key) {
                true
            } else {
                release_media_source(source);
                false
            }
        });
    });
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn clear_media_cache(media_cache: CzmlMediaCache) {
    media_cache.update_value(|cache| {
        for source in cache.values() {
            release_media_source(source);
        }
        cache.clear();
    });
}

#[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
fn normalize_media_uri(uri: &str, base_uri: Option<&str>) -> Result<String, String> {
    if uri.is_empty() {
        return Err("properties.media_uri must not be empty".to_string());
    }

    if has_absolute_or_special_uri(uri) {
        return Ok(uri.to_string());
    }

    let Some(base_uri) = base_uri.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(uri.to_string());
    };

    if let Ok(base_url) = Url::parse(base_uri) {
        return base_url
            .join(uri)
            .map(|value| value.to_string())
            .map_err(|error| {
                format!(
                    "Failed to resolve media URI '{}' against '{}': {}",
                    uri, base_uri, error
                )
            });
    }

    let dummy_origin =
        Url::parse("https://leptos-cesium.invalid/").expect("dummy origin URL should always parse");
    let base_url = dummy_origin
        .join(base_uri)
        .map_err(|error| format!("Failed to resolve media base URI '{}': {}", base_uri, error))?;
    let joined = base_url.join(uri).map_err(|error| {
        format!(
            "Failed to resolve media URI '{}' against '{}': {}",
            uri, base_uri, error
        )
    })?;

    let preserve_root = base_uri.starts_with('/') || uri.starts_with('/');
    Ok(stringify_relative_join(joined, preserve_root))
}

#[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
fn has_absolute_or_special_uri(uri: &str) -> bool {
    uri.starts_with("//")
        || uri.starts_with("data:")
        || uri.starts_with("blob:")
        || Url::parse(uri).is_ok()
}

#[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
fn stringify_relative_join(joined: Url, preserve_root: bool) -> String {
    let mut result = joined.path().to_string();
    if !preserve_root {
        result = result.trim_start_matches('/').to_string();
    }

    if let Some(query) = joined.query() {
        result.push('?');
        result.push_str(query);
    }
    if let Some(fragment) = joined.fragment() {
        result.push('#');
        result.push_str(fragment);
    }

    result
}

#[cfg(target_arch = "wasm32")]
fn entity_values_array(data_source: &CesiumCzmlDataSource) -> Option<Array> {
    let entities = data_source.entities();
    let entities_js = JsValue::from(entities);
    let values = Reflect::get(&entities_js, &JsValue::from_str("values")).ok()?;
    values.dyn_into::<Array>().ok()
}

#[cfg(target_arch = "wasm32")]
fn parse_media_descriptor(entity: &JsValue) -> Result<Option<CzmlMediaDescriptor>, String> {
    let entity_id = entity_id(entity).unwrap_or_else(|| "<unknown>".to_string());

    let properties = match Reflect::get(entity, &JsValue::from_str("properties")) {
        Ok(value) if !value.is_null() && !value.is_undefined() => value,
        _ => return Ok(None),
    };

    let properties_value = resolve_property_value(&properties);
    let uri = read_property_string_field(&properties, &properties_value, "media_uri")
        .or_else(|| read_property_string_field(&properties, &properties_value, "media_url"));
    let Some(uri) = uri else {
        return Ok(None);
    };

    let kind = match read_property_string_field(&properties, &properties_value, "media_kind")
        .unwrap_or_else(|| infer_kind_from_uri(&uri).to_string())
        .to_lowercase()
        .as_str()
    {
        "image" => CzmlMediaKind::Image,
        "video" => CzmlMediaKind::Video,
        other => {
            return Err(format!(
                "Unsupported properties.media_kind '{}' (expected 'image' or 'video')",
                other
            ));
        }
    };

    let target = match read_property_string_field(&properties, &properties_value, "media_target") {
        Some(value) => parse_target(&value)?,
        None => infer_target(entity),
    };

    let is_video = matches!(kind, CzmlMediaKind::Video);

    let autoplay = read_property_bool_field(&properties, &properties_value, "media_autoplay")
        .unwrap_or(is_video);
    let loop_video =
        read_property_bool_field(&properties, &properties_value, "media_loop").unwrap_or(is_video);
    let muted =
        read_property_bool_field(&properties, &properties_value, "media_muted").unwrap_or(is_video);
    let plays_inline =
        read_property_bool_field(&properties, &properties_value, "media_playsinline")
            .or_else(|| {
                read_property_bool_field(&properties, &properties_value, "media_plays_inline")
            })
            .unwrap_or(is_video);

    let cross_origin =
        read_property_string_field(&properties, &properties_value, "media_cross_origin")
            .or_else(|| {
                read_property_string_field(&properties, &properties_value, "media_crossOrigin")
            })
            .or_else(|| {
                if is_video && !uri.starts_with("data:") {
                    Some("anonymous".to_string())
                } else {
                    None
                }
            });

    Ok(Some(CzmlMediaDescriptor {
        entity_id,
        kind,
        target,
        uri,
        autoplay,
        loop_video,
        muted,
        plays_inline,
        cross_origin,
    }))
}

#[cfg(target_arch = "wasm32")]
fn infer_kind_from_uri(uri: &str) -> &'static str {
    let normalized = uri
        .split(['?', '#'])
        .next()
        .unwrap_or(uri)
        .to_ascii_lowercase();
    if normalized.ends_with(".mp4")
        || normalized.ends_with(".webm")
        || normalized.ends_with(".mov")
        || normalized.ends_with(".m4v")
        || normalized.ends_with(".ogv")
        || normalized.ends_with(".m3u8")
    {
        "video"
    } else {
        "image"
    }
}

#[cfg(target_arch = "wasm32")]
fn parse_target(value: &str) -> Result<CzmlMediaTarget, String> {
    match value.to_lowercase().as_str() {
        "billboard" => Ok(CzmlMediaTarget::Billboard),
        "rectangle" => Ok(CzmlMediaTarget::Rectangle),
        "polygon" => Ok(CzmlMediaTarget::Polygon),
        other => Err(format!(
            "Unsupported properties.media_target '{}' (expected billboard|rectangle|polygon)",
            other
        )),
    }
}

#[cfg(target_arch = "wasm32")]
fn infer_target(entity: &JsValue) -> CzmlMediaTarget {
    if has_entity_graphic(entity, "billboard") {
        CzmlMediaTarget::Billboard
    } else if has_entity_graphic(entity, "rectangle") {
        CzmlMediaTarget::Rectangle
    } else if has_entity_graphic(entity, "polygon") {
        CzmlMediaTarget::Polygon
    } else {
        CzmlMediaTarget::Billboard
    }
}

#[cfg(target_arch = "wasm32")]
fn has_entity_graphic(entity: &JsValue, property: &str) -> bool {
    Reflect::get(entity, &JsValue::from_str(property))
        .map(|value| !value.is_null() && !value.is_undefined())
        .unwrap_or(false)
}

#[cfg(target_arch = "wasm32")]
fn read_string_field(object: &JsValue, key: &str) -> Option<String> {
    let value = Reflect::get(object, &JsValue::from_str(key)).ok()?;
    extract_string_value(&value)
}

#[cfg(target_arch = "wasm32")]
fn read_bool_field(object: &JsValue, key: &str) -> Option<bool> {
    let value = Reflect::get(object, &JsValue::from_str(key)).ok()?;
    extract_bool_value(&value)
}

#[cfg(target_arch = "wasm32")]
fn read_property_string_field(
    properties: &JsValue,
    properties_value: &JsValue,
    key: &str,
) -> Option<String> {
    read_string_field(properties, key).or_else(|| read_string_field(properties_value, key))
}

#[cfg(target_arch = "wasm32")]
fn read_property_bool_field(
    properties: &JsValue,
    properties_value: &JsValue,
    key: &str,
) -> Option<bool> {
    read_bool_field(properties, key).or_else(|| read_bool_field(properties_value, key))
}

#[cfg(target_arch = "wasm32")]
fn extract_string_value(value: &JsValue) -> Option<String> {
    let value = resolve_property_value(value);
    if let Some(text) = value.as_string() {
        return Some(text);
    }

    if !value.is_object() {
        return None;
    }

    for key in ["value", "string", "uri", "url"] {
        let Ok(nested) = Reflect::get(&value, &JsValue::from_str(key)) else {
            continue;
        };
        let nested = resolve_property_value(&nested);
        if let Some(text) = nested.as_string() {
            return Some(text);
        }
    }

    None
}

#[cfg(target_arch = "wasm32")]
fn extract_bool_value(value: &JsValue) -> Option<bool> {
    let value = resolve_property_value(value);
    if let Some(flag) = value.as_bool() {
        return Some(flag);
    }

    if !value.is_object() {
        return None;
    }

    for key in ["value", "boolean"] {
        let Ok(nested) = Reflect::get(&value, &JsValue::from_str(key)) else {
            continue;
        };
        let nested = resolve_property_value(&nested);
        if let Some(flag) = nested.as_bool() {
            return Some(flag);
        }
    }

    None
}

#[cfg(target_arch = "wasm32")]
fn resolve_property_value(value: &JsValue) -> JsValue {
    let mut current = value.clone();
    for _ in 0..4 {
        if current.is_null() || current.is_undefined() {
            break;
        }

        let Ok(get_value) = Reflect::get(&current, &JsValue::from_str("getValue")) else {
            break;
        };
        if !get_value.is_function() {
            break;
        }

        let get_value: Function = get_value.unchecked_into();
        let next = get_value
            .call0(&current)
            .or_else(|_| get_value.call1(&current, &JsValue::undefined()))
            .unwrap_or(current.clone());

        if next.is_null() || next.is_undefined() {
            return next;
        }

        current = next;
    }

    current
}

#[cfg(target_arch = "wasm32")]
fn apply_media_to_entity(
    entity: &JsValue,
    descriptor: &CzmlMediaDescriptor,
    media_source: &MediaSource,
) -> Result<(), String> {
    let target_property = descriptor.target.as_graphic_property();
    let target_graphic = ensure_entity_graphic(entity, target_property)?;

    match descriptor.target {
        CzmlMediaTarget::Billboard => {
            set_property_checked(
                &target_graphic,
                &JsValue::from_str("image"),
                &media_source.to_js_value(),
            )
            .map_err(|reason| format!("Failed to set {}.image: {}", target_property, reason))?;
        }
        CzmlMediaTarget::Rectangle | CzmlMediaTarget::Polygon => {
            let material = build_image_material_property_value(media_source);
            set_property_checked(&target_graphic, &JsValue::from_str("material"), &material)
                .map_err(|reason| {
                    format!("Failed to set {}.material: {}", target_property, reason)
                })?;
        }
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn build_image_material_property_value(media_source: &MediaSource) -> JsValue {
    let material = ImageMaterialPropertyBuilder::new()
        .image(media_source.clone())
        .build();
    JsValue::from(material)
}

#[cfg(target_arch = "wasm32")]
fn set_property_checked(target: &JsValue, key: &JsValue, value: &JsValue) -> Result<(), String> {
    let wrote = Reflect::set(target, key, value).map_err(js_error_to_string)?;
    if wrote {
        Ok(())
    } else {
        Err("Reflect.set returned false".to_string())
    }
}

#[cfg(target_arch = "wasm32")]
fn ensure_entity_graphic(entity: &JsValue, property: &str) -> Result<JsValue, String> {
    let existing = Reflect::get(entity, &JsValue::from_str(property))
        .map_err(|_| format!("Failed to access entity.{}", property))?;

    if !existing.is_null() && !existing.is_undefined() {
        return Ok(existing);
    }

    Err(format!("entity.{} is not available yet", property))
}

#[cfg(target_arch = "wasm32")]
fn default_media_source(
    descriptor: &CzmlMediaDescriptor,
    request_id: u64,
    request_gate: RequestGate,
    on_error: Option<Callback<CzmlMediaError>>,
) -> Result<MediaSource, String> {
    match descriptor.kind {
        CzmlMediaKind::Image => {
            if descriptor.uri.starts_with("data:") {
                Ok(MediaSource::DataUrl(descriptor.uri.clone()))
            } else {
                Ok(MediaSource::Url(descriptor.uri.clone()))
            }
        }
        CzmlMediaKind::Video => {
            create_video_media_source(descriptor, request_id, request_gate, on_error)
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn create_video_media_source(
    descriptor: &CzmlMediaDescriptor,
    request_id: u64,
    request_gate: RequestGate,
    on_error: Option<Callback<CzmlMediaError>>,
) -> Result<MediaSource, String> {
    let window = web_sys::window().ok_or_else(|| "Window is not available".to_string())?;
    let document = window
        .document()
        .ok_or_else(|| "Document is not available".to_string())?;
    let video_element = document
        .create_element("video")
        .map_err(js_error_to_string)?
        .dyn_into::<web_sys::HtmlVideoElement>()
        .map_err(|_| "Failed to create HTMLVideoElement".to_string())?;

    video_element.set_autoplay(descriptor.autoplay);
    video_element.set_loop(descriptor.loop_video);
    video_element.set_muted(descriptor.muted);

    if descriptor.plays_inline {
        let _ = video_element.set_attribute("playsinline", "");
    }

    if let Some(cross_origin) = descriptor.cross_origin.as_deref() {
        video_element.set_cross_origin(Some(cross_origin));
        let _ = video_element.set_attribute("crossorigin", cross_origin);
    }

    video_element.set_src(&descriptor.uri);
    video_element.load();

    if descriptor.autoplay {
        let play_result = video_element.play();
        match play_result {
            Ok(promise) => {
                let request_gate = request_gate.clone();
                let entity_id = descriptor.entity_id.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if JsFuture::from(promise).await.is_err() && !request_gate.is_stale(request_id)
                    {
                        emit_media_error(
                            on_error,
                            Some(entity_id),
                            "Video autoplay was blocked; user gesture may be required",
                        );
                    }
                });
            }
            Err(error) => {
                return Err(format!(
                    "Video autoplay failed to start: {}",
                    js_error_to_string(error)
                ));
            }
        }
    }

    Ok(MediaSource::HtmlVideo(video_element))
}

#[cfg(target_arch = "wasm32")]
fn release_media_source(media_source: &MediaSource) {
    if let MediaSource::HtmlVideo(video) = media_source {
        let _ = video.pause();
        video.set_src("");
        video.load();
        video.remove();
    }
}

#[cfg(target_arch = "wasm32")]
fn emit_media_error(
    on_error: Option<Callback<CzmlMediaError>>,
    entity_id: Option<String>,
    reason: impl Into<String>,
) {
    let error = CzmlMediaError::new(entity_id, reason);

    if let Some(callback) = on_error {
        callback.run(error);
    } else {
        let entity_label = error
            .entity_id
            .as_deref()
            .map(|id| format!("[{}] ", id))
            .unwrap_or_default();
        web_sys::console::warn_1(&JsValue::from_str(&format!(
            "CzmlMedia {}{}",
            entity_label, error.reason
        )));
    }
}

#[cfg(target_arch = "wasm32")]
fn entity_id(entity: &JsValue) -> Option<String> {
    Reflect::get(entity, &JsValue::from_str("id"))
        .ok()
        .and_then(|value| value.as_string())
}

#[cfg(target_arch = "wasm32")]
fn js_error_to_string(error: JsValue) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_absolute_media_uri_unchanged() {
        let uri = "https://example.com/video.mp4";
        assert_eq!(
            normalize_media_uri(uri, Some("SampleData/demo.czml")).unwrap(),
            uri
        );
    }

    #[test]
    fn resolves_relative_media_uri_against_relative_czml_path() {
        assert_eq!(
            normalize_media_uri("pin.svg", Some("SampleData/demo.czml")).unwrap(),
            "SampleData/pin.svg"
        );
        assert_eq!(
            normalize_media_uri("../pin.svg", Some("SampleData/demo.czml")).unwrap(),
            "pin.svg"
        );
    }

    #[test]
    fn resolves_relative_media_uri_against_absolute_czml_url() {
        assert_eq!(
            normalize_media_uri(
                "video.mp4",
                Some("https://example.com/assets/routes/demo.czml"),
            )
            .unwrap(),
            "https://example.com/assets/routes/video.mp4"
        );
    }

    #[test]
    fn cache_key_includes_video_flags() {
        let base = CzmlMediaDescriptor {
            entity_id: "video".to_string(),
            kind: CzmlMediaKind::Video,
            target: CzmlMediaTarget::Rectangle,
            uri: "video.mp4".to_string(),
            autoplay: true,
            loop_video: true,
            muted: true,
            plays_inline: true,
            cross_origin: Some("anonymous".to_string()),
        };

        let mut changed = base.clone();
        changed.muted = false;

        assert_ne!(
            MediaCacheKey::from_descriptor(&base),
            MediaCacheKey::from_descriptor(&changed)
        );
    }

    #[test]
    fn cache_key_includes_target_and_uri() {
        let base = CzmlMediaDescriptor {
            entity_id: "video".to_string(),
            kind: CzmlMediaKind::Video,
            target: CzmlMediaTarget::Rectangle,
            uri: "video.mp4".to_string(),
            autoplay: true,
            loop_video: true,
            muted: true,
            plays_inline: true,
            cross_origin: Some("anonymous".to_string()),
        };

        let mut changed_target = base.clone();
        changed_target.target = CzmlMediaTarget::Polygon;

        let mut changed_uri = base.clone();
        changed_uri.uri = "video-2.mp4".to_string();

        assert_ne!(
            MediaCacheKey::from_descriptor(&base),
            MediaCacheKey::from_descriptor(&changed_target)
        );
        assert_ne!(
            MediaCacheKey::from_descriptor(&base),
            MediaCacheKey::from_descriptor(&changed_uri)
        );
    }
}
