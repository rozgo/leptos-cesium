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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CzmlMediaKind {
    Image,
    Video,
    Youtube,
    Rerun,
}

/// Parsed `properties.media_*` descriptor for one entity.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CzmlMediaDescriptor {
    pub entity_id: String,
    pub kind: CzmlMediaKind,
    pub media_uri: Option<String>,
    pub youtube_id: Option<String>,
    pub resizable: bool,
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
    Image {
        src: String,
        width_px: u32,
        height_px: u32,
        resizable: bool,
        cross_origin: Option<String>,
    },
    Video {
        src: String,
        width_px: u32,
        height_px: u32,
        resizable: bool,
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
        resizable: bool,
        autoplay: bool,
        mute: bool,
        controls: bool,
        start_seconds: Option<u32>,
    },
    #[cfg(feature = "rerun")]
    Rerun {
        src: String,
        width_px: u32,
        height_px: u32,
        resizable: bool,
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
pub(crate) fn reconcile_data_source_overlay_media(
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

        if let Some(resolver) = resolve_media
            && matches!(descriptor.kind, CzmlMediaKind::Video)
            && let Some(resolved) = resolver.run(descriptor.clone())
        {
            match resolved_media_uri(&resolved) {
                Ok(uri) => descriptor.media_uri = Some(uri),
                Err(error) => {
                    emit_media_error(on_error, Some(descriptor.entity_id.clone()), error);
                    continue;
                }
            }
        }

        if matches!(
            descriptor.kind,
            CzmlMediaKind::Image | CzmlMediaKind::Video | CzmlMediaKind::Rerun
        ) {
            let Some(media_uri) = descriptor.media_uri.as_deref() else {
                emit_media_error(
                    on_error,
                    Some(descriptor.entity_id.clone()),
                    media_uri_required_error(descriptor.kind),
                );
                continue;
            };

            descriptor.media_uri = match normalize_media_uri(media_uri, base_uri.as_deref()) {
                Ok(value) => Some(value),
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
        CzmlMediaKind::Image => Ok(CzmlOverlayMedia::Image {
            src: descriptor
                .media_uri
                .clone()
                .ok_or_else(|| media_uri_required_error(CzmlMediaKind::Image).to_string())?,
            width_px: descriptor.width_px,
            height_px: descriptor.height_px,
            resizable: descriptor.resizable,
            cross_origin: descriptor.cross_origin.clone(),
        }),
        CzmlMediaKind::Video => Ok(CzmlOverlayMedia::Video {
            src: descriptor
                .media_uri
                .clone()
                .ok_or_else(|| media_uri_required_error(CzmlMediaKind::Video).to_string())?,
            width_px: descriptor.width_px,
            height_px: descriptor.height_px,
            resizable: descriptor.resizable,
            autoplay: descriptor.autoplay,
            loop_video: descriptor.loop_video,
            muted: descriptor.muted,
            plays_inline: descriptor.plays_inline,
            controls: descriptor.controls,
            cross_origin: descriptor.cross_origin.clone(),
            poster: descriptor.poster.clone(),
            preload: descriptor.preload.clone(),
        }),
        CzmlMediaKind::Youtube => Ok(CzmlOverlayMedia::Youtube {
            video_id: descriptor.youtube_id.clone().ok_or_else(|| {
                "YouTube overlay media requires `properties.media_youtube_id`".to_string()
            })?,
            width_px: descriptor.width_px,
            height_px: descriptor.height_px,
            resizable: descriptor.resizable,
            autoplay: descriptor.autoplay,
            mute: descriptor.muted,
            controls: descriptor.controls,
            start_seconds: descriptor.start_seconds,
        }),
        CzmlMediaKind::Rerun => rerun_overlay_media_from_descriptor(descriptor),
    }
}

#[cfg(target_arch = "wasm32")]
fn media_uri_required_error(kind: CzmlMediaKind) -> &'static str {
    match kind {
        CzmlMediaKind::Image => "Image overlay media requires `properties.media_uri`",
        CzmlMediaKind::Video => "Video overlay media requires `properties.media_uri`",
        CzmlMediaKind::Youtube => "YouTube overlay media does not use `properties.media_uri`",
        CzmlMediaKind::Rerun => "Rerun overlay media requires `properties.media_uri`",
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
    let media_kind = read_property_string_field(&properties, &properties_value, "media_kind");
    let media_uri = read_property_string_field(&properties, &properties_value, "media_uri");
    let youtube_id = read_property_string_field(&properties, &properties_value, "media_youtube_id");
    let legacy_media_target =
        read_property_string_field(&properties, &properties_value, "media_target");
    let legacy_media_url = read_property_string_field(&properties, &properties_value, "media_url");
    let legacy_media_start = read_property_u32_field(&properties, &properties_value, "media_start");

    if media_kind.is_none()
        && media_uri.is_none()
        && youtube_id.is_none()
        && legacy_media_target.is_none()
        && legacy_media_url.is_none()
        && legacy_media_start.is_none()
    {
        return Ok(None);
    }

    if legacy_media_target.is_some() {
        return Err(
            "Legacy `properties.media_target` is no longer supported; overlay media now uses `entity.position` only".to_string(),
        );
    }

    if legacy_media_url.is_some() {
        return Err(
            "Legacy `properties.media_url` is no longer supported; use `properties.media_uri`"
                .to_string(),
        );
    }

    if legacy_media_start.is_some() {
        return Err(
            "Legacy `properties.media_start` is no longer supported; use `properties.media_start_seconds`".to_string(),
        );
    }

    let Some(kind_value) = media_kind else {
        return Err("Overlay media requires `properties.media_kind`".to_string());
    };

    let kind = match kind_value.to_lowercase().as_str() {
        "image" => CzmlMediaKind::Image,
        "video" => CzmlMediaKind::Video,
        "youtube" => CzmlMediaKind::Youtube,
        "rerun" => CzmlMediaKind::Rerun,
        other => {
            return Err(format!(
                "Unsupported properties.media_kind '{}' (expected 'image', 'video', 'youtube' or 'rerun')",
                other
            ));
        }
    };

    let is_video = matches!(kind, CzmlMediaKind::Video);
    let is_youtube = matches!(kind, CzmlMediaKind::Youtube);

    let autoplay = read_property_bool_field(&properties, &properties_value, "media_autoplay")
        .unwrap_or(is_video);
    let resizable = read_property_bool_field(&properties, &properties_value, "media_resizable")
        .unwrap_or(false);
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
        read_property_string_field(&properties, &properties_value, "media_cross_origin").or_else(
            || read_property_string_field(&properties, &properties_value, "media_crossOrigin"),
        );

    let width_px =
        read_property_u32_field(&properties, &properties_value, "media_width").unwrap_or(320);
    let height_px =
        read_property_u32_field(&properties, &properties_value, "media_height").unwrap_or(180);
    let poster = read_property_string_field(&properties, &properties_value, "media_poster");
    let preload = read_property_string_field(&properties, &properties_value, "media_preload");
    let start_seconds =
        read_property_u32_field(&properties, &properties_value, "media_start_seconds");

    let (media_uri, youtube_id) = match kind {
        CzmlMediaKind::Image => {
            let Some(image_uri) = media_uri else {
                return Err(media_uri_required_error(CzmlMediaKind::Image).to_string());
            };
            (Some(image_uri), None)
        }
        CzmlMediaKind::Video => {
            let Some(video_uri) = media_uri else {
                return Err(media_uri_required_error(CzmlMediaKind::Video).to_string());
            };
            (Some(video_uri), None)
        }
        CzmlMediaKind::Youtube => {
            if media_uri.is_some() {
                return Err(
                    "YouTube overlay media requires `properties.media_youtube_id`; `properties.media_uri` is not supported".to_string(),
                );
            }
            let Some(youtube_id) = youtube_id else {
                return Err(
                    "YouTube overlay media requires `properties.media_youtube_id`".to_string(),
                );
            };
            (None, Some(youtube_id))
        }
        CzmlMediaKind::Rerun => {
            let Some(rerun_uri) = media_uri else {
                return Err(media_uri_required_error(CzmlMediaKind::Rerun).to_string());
            };
            (Some(rerun_uri), None)
        }
    };

    Ok(Some(CzmlMediaDescriptor {
        entity_id,
        kind,
        media_uri,
        youtube_id,
        resizable,
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

#[cfg(all(target_arch = "wasm32", feature = "rerun"))]
fn rerun_overlay_media_from_descriptor(
    descriptor: &CzmlMediaDescriptor,
) -> Result<CzmlOverlayMedia, String> {
    let src = descriptor
        .media_uri
        .clone()
        .ok_or_else(|| media_uri_required_error(CzmlMediaKind::Rerun).to_string())?;

    Ok(CzmlOverlayMedia::Rerun {
        src: absolutize_browser_url(&src),
        width_px: descriptor.width_px,
        height_px: descriptor.height_px,
        resizable: descriptor.resizable,
    })
}

#[cfg(all(target_arch = "wasm32", not(feature = "rerun")))]
fn rerun_overlay_media_from_descriptor(
    _descriptor: &CzmlMediaDescriptor,
) -> Result<CzmlOverlayMedia, String> {
    Err("Rerun overlay media requires the `rerun` feature on `leptos-cesium`".to_string())
}

#[cfg(target_arch = "wasm32")]
fn absolutize_browser_url(url: &str) -> String {
    if url.contains("://") || url.starts_with("//") {
        return url.to_string();
    }

    let Some(origin) = web_sys::window().and_then(|window| window.location().origin().ok()) else {
        return url.to_string();
    };

    if url.starts_with('/') {
        format!("{origin}{url}")
    } else {
        format!("{origin}/{url}")
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
}
