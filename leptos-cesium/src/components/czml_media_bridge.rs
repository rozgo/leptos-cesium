//! Internal helpers for parsing flattened CZML media metadata into overlay bindings.

use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

use crate::bindings::MediaSource;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{CzmlDataSource as CesiumCzmlDataSource, Entity};
#[cfg(target_arch = "wasm32")]
use crate::core::{RequestGate, ThreadSafeJsValue};
#[cfg(target_arch = "wasm32")]
use js_sys::{Array, Function, Reflect};
use url::Url;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Media kind for CZML media metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CzmlMediaKind {
    Video,
    Youtube,
}

/// Target metadata declared in CZML.
///
/// Overlay media is point-anchored in v1, but this field is retained for validation and
/// backwards-compatible parsing of existing `properties.media_target` packets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CzmlMediaTarget {
    Billboard,
    Rectangle,
    Polygon,
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
    pub controls: bool,
    pub width_px: u32,
    pub height_px: u32,
    pub poster: Option<String>,
    pub preload: Option<String>,
    pub start_seconds: Option<u32>,
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

/// Optional callback for custom URI -> media resolution.
pub type CzmlMediaResolver = Callback<CzmlMediaDescriptor, Option<MediaSource>>;

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum CzmlOverlayMedia {
    Video {
        src: String,
        width_px: u32,
        height_px: u32,
        autoplay: bool,
        loop_video: bool,
        muted: bool,
        plays_inline: bool,
        controls: bool,
        cross_origin: Option<String>,
        poster: Option<String>,
        preload: Option<String>,
    },
    Youtube {
        video_id: String,
        width_px: u32,
        height_px: u32,
        autoplay: bool,
        mute: bool,
        controls: bool,
        start_seconds: Option<u32>,
    },
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub(crate) struct CzmlOverlayBinding {
    pub entity_id: String,
    pub entity: ThreadSafeJsValue<Entity>,
    pub media: CzmlOverlayMedia,
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn reconcile_data_source_media(
    source_js: JsValue,
    request_id: u64,
    request_gate: RequestGate,
    resolve_media: Option<CzmlMediaResolver>,
    on_error: Option<Callback<CzmlMediaError>>,
    base_uri: Option<String>,
) -> Option<Vec<CzmlOverlayBinding>> {
    if request_gate.is_stale(request_id) {
        return None;
    }

    let data_source = match source_js.dyn_into::<CesiumCzmlDataSource>() {
        Ok(value) => value,
        Err(_) => {
            emit_media_error(
                on_error,
                None,
                "CzmlDataSource media handling expected a Cesium CzmlDataSource handle",
            );
            return Some(Vec::new());
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
            return Some(Vec::new());
        }
    };

    let mut bindings = Vec::new();

    for entity in entities_array.iter() {
        if request_gate.is_stale(request_id) {
            return None;
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

        if descriptor.target != CzmlMediaTarget::Billboard {
            emit_media_error(
                on_error,
                Some(descriptor.entity_id.clone()),
                "Overlay media only supports point anchors (`entity.position`) in v1",
            );
            continue;
        }

        if let Some(resolver) = resolve_media
            && matches!(descriptor.kind, CzmlMediaKind::Video)
            && let Some(resolved) = resolver.run(descriptor.clone())
        {
            match resolved_media_uri(&resolved) {
                Ok(uri) => descriptor.uri = uri,
                Err(error) => {
                    emit_media_error(on_error, Some(descriptor.entity_id.clone()), error);
                    continue;
                }
            }
        }

        if matches!(descriptor.kind, CzmlMediaKind::Video) {
            descriptor.uri = match normalize_media_uri(&descriptor.uri, base_uri.as_deref()) {
                Ok(value) => value,
                Err(error) => {
                    emit_media_error(on_error, Some(descriptor.entity_id.clone()), error);
                    continue;
                }
            };
        }

        let entity_handle = match entity.clone().dyn_into::<Entity>() {
            Ok(value) => value,
            Err(_) => {
                emit_media_error(
                    on_error,
                    Some(descriptor.entity_id.clone()),
                    "Failed to cast CZML entity handle to Cesium.Entity",
                );
                continue;
            }
        };

        if entity_handle.position().is_none() {
            emit_media_error(
                on_error,
                Some(descriptor.entity_id.clone()),
                "Overlay media requires `entity.position` in CZML",
            );
            continue;
        }

        let media = match overlay_media_from_descriptor(&descriptor) {
            Ok(value) => value,
            Err(error) => {
                emit_media_error(on_error, Some(descriptor.entity_id.clone()), error);
                continue;
            }
        };

        bindings.push(CzmlOverlayBinding {
            entity_id: descriptor.entity_id.clone(),
            entity: ThreadSafeJsValue::new(entity_handle),
            media,
        });
    }

    Some(bindings)
}

#[cfg(target_arch = "wasm32")]
fn overlay_media_from_descriptor(
    descriptor: &CzmlMediaDescriptor,
) -> Result<CzmlOverlayMedia, String> {
    match descriptor.kind {
        CzmlMediaKind::Video => Ok(CzmlOverlayMedia::Video {
            src: descriptor.uri.clone(),
            width_px: descriptor.width_px,
            height_px: descriptor.height_px,
            autoplay: descriptor.autoplay,
            loop_video: descriptor.loop_video,
            muted: descriptor.muted,
            plays_inline: descriptor.plays_inline,
            controls: descriptor.controls,
            cross_origin: descriptor.cross_origin.clone(),
            poster: descriptor.poster.clone(),
            preload: descriptor.preload.clone(),
        }),
        CzmlMediaKind::Youtube => {
            let Some(video_id) = extract_youtube_video_id(&descriptor.uri) else {
                return Err(format!(
                    "Unable to extract a YouTube video id from '{}'",
                    descriptor.uri
                ));
            };

            Ok(CzmlOverlayMedia::Youtube {
                video_id,
                width_px: descriptor.width_px,
                height_px: descriptor.height_px,
                autoplay: descriptor.autoplay,
                mute: descriptor.muted,
                controls: descriptor.controls,
                start_seconds: descriptor.start_seconds,
            })
        }
    }
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
    let explicit_youtube_id =
        read_property_string_field(&properties, &properties_value, "media_youtube_id");
    let uri = read_property_string_field(&properties, &properties_value, "media_uri")
        .or_else(|| read_property_string_field(&properties, &properties_value, "media_url"))
        .or(explicit_youtube_id.clone());
    let Some(uri) = uri else {
        return Ok(None);
    };

    let kind = match read_property_string_field(&properties, &properties_value, "media_kind")
        .unwrap_or_else(|| {
            if explicit_youtube_id.is_some() {
                "youtube".to_string()
            } else {
                infer_kind_from_uri(&uri).to_string()
            }
        })
        .to_lowercase()
        .as_str()
    {
        "video" => CzmlMediaKind::Video,
        "youtube" => CzmlMediaKind::Youtube,
        other => {
            return Err(format!(
                "Unsupported properties.media_kind '{}' (expected 'video' or 'youtube')",
                other
            ));
        }
    };

    let target = match read_property_string_field(&properties, &properties_value, "media_target") {
        Some(value) => parse_target(&value)?,
        None => infer_target(entity),
    };

    let is_video = matches!(kind, CzmlMediaKind::Video);
    let is_youtube = matches!(kind, CzmlMediaKind::Youtube);

    let autoplay = read_property_bool_field(&properties, &properties_value, "media_autoplay")
        .unwrap_or(is_video);
    let loop_video =
        read_property_bool_field(&properties, &properties_value, "media_loop").unwrap_or(is_video);
    let muted = read_property_bool_field(&properties, &properties_value, "media_muted")
        .unwrap_or(is_video || is_youtube);
    let plays_inline =
        read_property_bool_field(&properties, &properties_value, "media_playsinline")
            .or_else(|| {
                read_property_bool_field(&properties, &properties_value, "media_plays_inline")
            })
            .unwrap_or(is_video || is_youtube);
    let controls = read_property_bool_field(&properties, &properties_value, "media_controls")
        .unwrap_or(is_youtube);

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

    let width_px =
        read_property_u32_field(&properties, &properties_value, "media_width").unwrap_or(320);
    let height_px =
        read_property_u32_field(&properties, &properties_value, "media_height").unwrap_or(180);
    let poster = read_property_string_field(&properties, &properties_value, "media_poster");
    let preload = read_property_string_field(&properties, &properties_value, "media_preload");
    let start_seconds = read_property_u32_field(&properties, &properties_value, "media_start")
        .or_else(|| read_property_u32_field(&properties, &properties_value, "media_start_seconds"));

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
        controls,
        width_px,
        height_px,
        poster,
        preload,
        start_seconds,
    }))
}

#[cfg(target_arch = "wasm32")]
fn infer_kind_from_uri(uri: &str) -> &'static str {
    if extract_youtube_video_id(uri).is_some() && looks_like_youtube_source(uri) {
        return "youtube";
    }

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
        "video"
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
fn read_number_field(object: &JsValue, key: &str) -> Option<f64> {
    let value = Reflect::get(object, &JsValue::from_str(key)).ok()?;
    extract_number_value(&value)
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
fn read_property_u32_field(
    properties: &JsValue,
    properties_value: &JsValue,
    key: &str,
) -> Option<u32> {
    read_number_field(properties, key)
        .or_else(|| read_number_field(properties_value, key))
        .and_then(|value| {
            if value.is_finite() && value >= 0.0 {
                Some(value.round() as u32)
            } else {
                None
            }
        })
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
fn extract_number_value(value: &JsValue) -> Option<f64> {
    let value = resolve_property_value(value);
    if let Some(number) = value.as_f64() {
        return Some(number);
    }

    if !value.is_object() {
        return None;
    }

    for key in ["value", "number"] {
        let Ok(nested) = Reflect::get(&value, &JsValue::from_str(key)) else {
            continue;
        };
        let nested = resolve_property_value(&nested);
        if let Some(number) = nested.as_f64() {
            return Some(number);
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
fn resolved_media_uri(media_source: &MediaSource) -> Result<String, String> {
    match media_source {
        MediaSource::Url(value) | MediaSource::DataUrl(value) => Ok(value.clone()),
        _ => Err(
            "Custom CZML overlay media resolvers must return MediaSource::Url or MediaSource::DataUrl"
                .to_string(),
        ),
    }
}

#[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
fn extract_youtube_video_id(source: &str) -> Option<String> {
    let trimmed = source.trim();
    if trimmed.is_empty() {
        return None;
    }

    if !trimmed.contains('/') && !trimmed.contains('?') && !trimmed.contains('&') {
        return Some(trimmed.to_string());
    }

    let Ok(url) = Url::parse(trimmed) else {
        return None;
    };
    let host = url.host_str()?.to_ascii_lowercase();

    if host == "youtu.be" {
        return url
            .path_segments()
            .and_then(|mut segments| segments.next())
            .filter(|value| !value.is_empty())
            .map(str::to_string);
    }

    if !host.contains("youtube.com") {
        return None;
    }

    if let Some(video_id) = url
        .query_pairs()
        .find_map(|(key, value)| (key == "v" && !value.is_empty()).then(|| value.into_owned()))
    {
        return Some(video_id);
    }

    url.path_segments().and_then(|mut segments| {
        let first = segments.next()?;
        let second = segments.next()?;
        matches!(first, "embed" | "shorts" | "live")
            .then(|| second.to_string())
            .filter(|value| !value.is_empty())
    })
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn looks_like_youtube_source(source: &str) -> bool {
    let trimmed = source.trim();
    trimmed.contains("youtu.be")
        || trimmed.contains("youtube.com")
        || read_plain_youtube_id(trimmed).is_some()
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn read_plain_youtube_id(source: &str) -> Option<&str> {
    let trimmed = source.trim();
    (trimmed.len() >= 6
        && !trimmed.contains('/')
        && !trimmed.contains('?')
        && !trimmed.contains('&'))
    .then_some(trimmed)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_absolute_media_uri_unchanged() {
        let uri = "https://example.com/assets/video.mp4";
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
                "pin.svg",
                Some("https://example.com/assets/demo/scene.czml")
            )
            .unwrap(),
            "https://example.com/assets/demo/pin.svg"
        );
    }

    #[test]
    fn extracts_youtube_id_from_short_url() {
        assert_eq!(
            extract_youtube_video_id("https://youtu.be/M7lc1UVf-VE"),
            Some("M7lc1UVf-VE".to_string())
        );
    }

    #[test]
    fn extracts_youtube_id_from_watch_url() {
        assert_eq!(
            extract_youtube_video_id("https://www.youtube.com/watch?v=M7lc1UVf-VE"),
            Some("M7lc1UVf-VE".to_string())
        );
    }

    #[test]
    fn accepts_plain_youtube_ids() {
        assert_eq!(
            extract_youtube_video_id("M7lc1UVf-VE"),
            Some("M7lc1UVf-VE".to_string())
        );
    }
}
