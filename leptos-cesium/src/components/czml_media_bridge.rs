//! Bridge component for applying image/video media metadata from CZML entity properties.

use leptos::prelude::*;
use wasm_bindgen::JsValue;

use crate::bindings::MediaSource;
use crate::core::JsSignal;

#[cfg(target_arch = "wasm32")]
use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use crate::bindings::{CzmlDataSource as CesiumCzmlDataSource, ImageMaterialPropertyBuilder};
#[cfg(target_arch = "wasm32")]
use crate::core::{JsStoredValue, RequestGate};
#[cfg(target_arch = "wasm32")]
use js_sys::{Array, Function, Reflect};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

/// Media kind for CZML bridge metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CzmlMediaKind {
    Image,
    Video,
}

/// Target graphic for CZML bridge metadata.
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

/// Parsed `properties.media` descriptor for one entity.
#[derive(Clone, Debug)]
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

/// Structured bridge error payload.
#[derive(Clone, Debug)]
pub struct CzmlMediaBridgeError {
    pub entity_id: Option<String>,
    pub reason: String,
}

impl CzmlMediaBridgeError {
    #[cfg(target_arch = "wasm32")]
    fn new(entity_id: Option<String>, reason: impl Into<String>) -> Self {
        Self {
            entity_id,
            reason: reason.into(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct MediaCacheKey {
    entity_id: String,
    uri: String,
    target: CzmlMediaTarget,
}

/// Optional bridge callback for custom URI -> media resolution.
pub type CzmlMediaResolver = Callback<CzmlMediaDescriptor, Option<MediaSource>>;

/// Reads CZML `properties.media` metadata and applies media to entity graphics.
///
/// Expected metadata schema:
///
/// ```json
/// {
///   "kind": "image" | "video",
///   "uri": "https://...",
///   "target": "billboard" | "rectangle" | "polygon",
///   "autoplay": true,
///   "loop": true,
///   "muted": true,
///   "playsinline": true,
///   "cross_origin": "anonymous"
/// }
/// ```
#[component(transparent)]
pub fn CzmlMediaBridge(
    /// CZML data source returned by `CzmlDataSource.on_loaded`.
    #[prop(optional, into)]
    data_source: JsSignal<Option<JsValue>>,
    /// Optional re-application trigger.
    #[prop(optional, into, default = ().into())]
    trigger: Signal<()>,
    /// Optional custom media resolver.
    #[prop(optional)]
    resolve_media: Option<CzmlMediaResolver>,
    /// Called with loading transitions for bridge application.
    #[prop(optional)]
    on_loading: Option<Callback<bool>>,
    /// Called with structured parse/apply errors.
    #[prop(optional)]
    on_error: Option<Callback<CzmlMediaBridgeError>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let media_cache = JsStoredValue::new_local(HashMap::<MediaCacheKey, MediaSource>::new());
        let request_gate = RequestGate::new();
        let request_gate_effect = request_gate.clone();

        Effect::new(move |_| {
            trigger.get();
            let data_source_value = data_source.get();
            let request_id = request_gate_effect.begin_request();

            let Some(source_js) = data_source_value else {
                clear_media_cache(media_cache);
                return;
            };
            if let Some(callback) = on_loading {
                callback.run(true);
            }

            apply_media_bridge_pass(
                source_js,
                request_id,
                media_cache,
                request_gate_effect.clone(),
                resolve_media,
                on_error,
            );

            if let Some(callback) = on_loading {
                callback.run(false);
            }
        });

        on_cleanup(move || {
            request_gate.close();
            clear_media_cache(media_cache);
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (data_source, trigger, resolve_media, on_loading, on_error);
    }
}

#[cfg(target_arch = "wasm32")]
fn apply_media_bridge_pass(
    source_js: JsValue,
    request_id: u64,
    media_cache: JsStoredValue<HashMap<MediaCacheKey, MediaSource>>,
    request_gate: RequestGate,
    resolve_media: Option<CzmlMediaResolver>,
    on_error: Option<Callback<CzmlMediaBridgeError>>,
) {
    if request_gate.is_stale(request_id) {
        return;
    }

    let data_source = match source_js.dyn_into::<CesiumCzmlDataSource>() {
        Ok(value) => value,
        Err(_) => {
            emit_bridge_error(
                on_error,
                None,
                "CzmlMediaBridge expected a Cesium CzmlDataSource handle",
            );
            return;
        }
    };

    let entities_array = match entity_values_array(&data_source) {
        Some(values) => values,
        None => {
            emit_bridge_error(
                on_error,
                None,
                "Unable to access dataSource.entities.values",
            );
            return;
        }
    };

    for entity in entities_array.iter() {
        if request_gate.is_stale(request_id) {
            return;
        }

        let descriptor = match parse_media_descriptor(&entity) {
            Ok(Some(value)) => value,
            Ok(None) => continue,
            Err(error) => {
                let entity_id = entity_id(&entity);
                emit_bridge_error(on_error, entity_id, error);
                continue;
            }
        };

        let cache_key = MediaCacheKey {
            entity_id: descriptor.entity_id.clone(),
            uri: descriptor.uri.clone(),
            target: descriptor.target,
        };

        let media_source = media_cache.with_value(|cache| cache.get(&cache_key).cloned());
        let media_source = match media_source {
            Some(value) => Some(value),
            None => {
                let resolved = resolve_media.and_then(|resolver| resolver.run(descriptor.clone()));
                match resolved {
                    Some(value) => Some(value),
                    None => default_media_source(
                        &descriptor,
                        request_id,
                        request_gate.clone(),
                        on_error,
                    ),
                }
            }
        };

        let Some(media_source) = media_source else {
            continue;
        };

        media_cache.update_value(|cache| {
            cache.insert(cache_key, media_source.clone());
        });

        if let Err(error) = apply_media_to_entity(&entity, &descriptor, &media_source) {
            emit_bridge_error(on_error, Some(descriptor.entity_id.clone()), error);
        }
    }
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

    // CZML custom properties may surface either as direct PropertyBag fields
    // (`entity.properties.media`) or only via `entity.properties.getValue()`.
    let properties_value = resolve_property_value(&properties);
    let media_property = Reflect::get(&properties, &JsValue::from_str("media"))
        .ok()
        .filter(|value| !value.is_null() && !value.is_undefined())
        .or_else(|| {
            Reflect::get(&properties_value, &JsValue::from_str("media"))
                .ok()
                .filter(|value| !value.is_null() && !value.is_undefined())
        });
    let Some(media_property) = media_property else {
        return Ok(None);
    };

    let mut media_value = resolve_property_value(&media_property);
    // Support wrappers like `{ value: { ... } }` by unwrapping one level.
    if media_value.is_object()
        && let Ok(nested) = Reflect::get(&media_value, &JsValue::from_str("value"))
        && !nested.is_null()
        && !nested.is_undefined()
    {
        media_value = resolve_property_value(&nested);
    }
    if media_value.is_null() || media_value.is_undefined() {
        return Ok(None);
    }

    if let Some(uri) = media_value.as_string() {
        return Ok(Some(CzmlMediaDescriptor {
            entity_id,
            kind: CzmlMediaKind::Image,
            target: infer_target(entity),
            uri,
            autoplay: false,
            loop_video: false,
            muted: false,
            plays_inline: false,
            cross_origin: None,
        }));
    }

    if !media_value.is_object() {
        return Err("properties.media must be an object or string".to_string());
    }

    let uri = read_string_field(&media_value, "uri")
        .or_else(|| read_string_field(&media_value, "url"))
        .ok_or_else(|| "properties.media.uri is required".to_string())?;

    let kind = match read_string_field(&media_value, "kind")
        .unwrap_or_else(|| "image".to_string())
        .to_lowercase()
        .as_str()
    {
        "image" => CzmlMediaKind::Image,
        "video" => CzmlMediaKind::Video,
        other => {
            return Err(format!(
                "Unsupported properties.media.kind '{}' (expected 'image' or 'video')",
                other
            ));
        }
    };

    let target = match read_string_field(&media_value, "target") {
        Some(value) => parse_target(&value)?,
        None => infer_target(entity),
    };

    let is_video = matches!(kind, CzmlMediaKind::Video);

    let autoplay = read_bool_field(&media_value, "autoplay").unwrap_or(is_video);
    let loop_video = read_bool_field(&media_value, "loop").unwrap_or(is_video);
    let muted = read_bool_field(&media_value, "muted").unwrap_or(is_video);
    let plays_inline = read_bool_field(&media_value, "playsinline")
        .or_else(|| read_bool_field(&media_value, "plays_inline"))
        .unwrap_or(is_video);

    let cross_origin = read_string_field(&media_value, "cross_origin")
        .or_else(|| read_string_field(&media_value, "crossOrigin"))
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
fn parse_target(value: &str) -> Result<CzmlMediaTarget, String> {
    match value.to_lowercase().as_str() {
        "billboard" => Ok(CzmlMediaTarget::Billboard),
        "rectangle" => Ok(CzmlMediaTarget::Rectangle),
        "polygon" => Ok(CzmlMediaTarget::Polygon),
        other => Err(format!(
            "Unsupported properties.media.target '{}' (expected billboard|rectangle|polygon)",
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
    let value = resolve_property_value(&value);
    value.as_string()
}

#[cfg(target_arch = "wasm32")]
fn read_bool_field(object: &JsValue, key: &str) -> Option<bool> {
    let value = Reflect::get(object, &JsValue::from_str(key)).ok()?;
    let value = resolve_property_value(&value);
    value.as_bool()
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
    on_error: Option<Callback<CzmlMediaBridgeError>>,
) -> Option<MediaSource> {
    match descriptor.kind {
        CzmlMediaKind::Image => {
            if descriptor.uri.starts_with("data:") {
                Some(MediaSource::DataUrl(descriptor.uri.clone()))
            } else {
                Some(MediaSource::Url(descriptor.uri.clone()))
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
    on_error: Option<Callback<CzmlMediaBridgeError>>,
) -> Option<MediaSource> {
    let document = web_sys::window()?.document()?;
    let video_element = document
        .create_element("video")
        .ok()?
        .dyn_into::<web_sys::HtmlVideoElement>()
        .ok()?;

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
                        emit_bridge_error(
                            on_error,
                            Some(entity_id),
                            "Video autoplay was blocked; user gesture may be required",
                        );
                    }
                });
            }
            Err(_) => {
                emit_bridge_error(
                    on_error,
                    Some(descriptor.entity_id.clone()),
                    "Video autoplay failed to start",
                );
            }
        }
    }

    Some(MediaSource::HtmlVideo(video_element))
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
fn clear_media_cache(media_cache: JsStoredValue<HashMap<MediaCacheKey, MediaSource>>) {
    media_cache.update_value(|cache| {
        for source in cache.values() {
            release_media_source(source);
        }
        cache.clear();
    });
}

#[cfg(target_arch = "wasm32")]
fn emit_bridge_error(
    on_error: Option<Callback<CzmlMediaBridgeError>>,
    entity_id: Option<String>,
    reason: impl Into<String>,
) {
    let error = CzmlMediaBridgeError::new(entity_id, reason);

    if let Some(callback) = on_error {
        callback.run(error);
    } else {
        let entity_label = error
            .entity_id
            .as_deref()
            .map(|id| format!("[{}] ", id))
            .unwrap_or_default();
        web_sys::console::warn_1(&JsValue::from_str(&format!(
            "CzmlMediaBridge {}{}",
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
