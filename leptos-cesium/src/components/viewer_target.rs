//! Viewer-level target focus components (`Viewer.flyTo` / `Viewer.zoomTo`).

use leptos::prelude::*;
use wasm_bindgen::JsValue;

use crate::bindings::{DataSource, Entity};
#[cfg(target_arch = "wasm32")]
use crate::bindings::{HeadingPitchRange, Viewer, ViewerFlyToOptions};
#[cfg(target_arch = "wasm32")]
use crate::components::use_cesium_context;
use crate::core::{JsSignal, ThreadSafeJsValue};

/// Target accepted by `Viewer.flyTo` and `Viewer.zoomTo`.
#[derive(Clone)]
pub enum ViewerTarget {
    DataSource(ThreadSafeJsValue<DataSource>),
    Entity(ThreadSafeJsValue<Entity>),
    JsValue(ThreadSafeJsValue<JsValue>),
}

impl From<DataSource> for ViewerTarget {
    fn from(value: DataSource) -> Self {
        Self::DataSource(ThreadSafeJsValue::new(value))
    }
}

impl From<Entity> for ViewerTarget {
    fn from(value: Entity) -> Self {
        Self::Entity(ThreadSafeJsValue::new(value))
    }
}

impl From<JsValue> for ViewerTarget {
    fn from(value: JsValue) -> Self {
        Self::JsValue(ThreadSafeJsValue::new(value))
    }
}

#[cfg(target_arch = "wasm32")]
impl ViewerTarget {
    fn to_js_value(&self) -> JsValue {
        match self {
            ViewerTarget::DataSource(value) => JsValue::from(value.value().clone()),
            ViewerTarget::Entity(value) => JsValue::from(value.value().clone()),
            ViewerTarget::JsValue(value) => value.value().clone(),
        }
    }
}

/// Triggered wrapper around `Viewer.flyTo(target, options?)`.
#[component(transparent)]
pub fn ViewerFlyToTarget(
    #[prop(into)] trigger: Signal<()>,
    #[prop(optional, into)] target: JsSignal<Option<ViewerTarget>>,
    #[prop(optional, into)] duration: Signal<Option<f64>>,
    #[prop(optional, into)] maximum_height: Signal<Option<f64>>,
    #[prop(optional, into)] pitch_adjust_height: Signal<Option<f64>>,
    #[prop(optional, into)] fly_over_longitude: Signal<Option<f64>>,
    #[prop(optional, into)] fly_over_longitude_weight: Signal<Option<f64>>,
    /// Optional heading/pitch/range offset.
    #[prop(optional, into)]
    offset: Signal<Option<(f64, f64, f64)>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("ViewerFlyToTarget must be inside ViewerContainer");
        let mut is_first_run = true;

        Effect::new(move |_| {
            trigger.get();
            if is_first_run {
                is_first_run = false;
                return;
            }

            // Trigger-driven action: read all arguments untracked.
            let Some(target) = target.get_untracked() else {
                return;
            };
            let duration = duration.get_untracked();
            let maximum_height = maximum_height.get_untracked();
            let pitch_adjust_height = pitch_adjust_height.get_untracked();
            let fly_over_longitude = fly_over_longitude.get_untracked();
            let fly_over_longitude_weight = fly_over_longitude_weight.get_untracked();
            let offset = offset.get_untracked();

            viewer_context.with_viewer(|viewer: Viewer| {
                let target_js = target.to_js_value();
                let mut options = ViewerFlyToOptions::new();
                let mut has_options = false;

                if let Some(duration) = duration {
                    options = options.duration(duration);
                    has_options = true;
                }
                if let Some(value) = maximum_height {
                    options = options.maximum_height(value);
                    has_options = true;
                }
                if let Some(value) = pitch_adjust_height {
                    options = options.pitch_adjust_height(value);
                    has_options = true;
                }
                if let Some(value) = fly_over_longitude {
                    options = options.fly_over_longitude(value);
                    has_options = true;
                }
                if let Some(value) = fly_over_longitude_weight {
                    options = options.fly_over_longitude_weight(value);
                    has_options = true;
                }
                if let Some((heading, pitch, range)) = offset {
                    options = options.offset(HeadingPitchRange::new(heading, pitch, range));
                    has_options = true;
                }

                let _ = if has_options {
                    viewer.fly_to_target_with_options(&target_js, &options.build())
                } else {
                    viewer.fly_to_target(&target_js)
                };
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            trigger,
            target,
            duration,
            maximum_height,
            pitch_adjust_height,
            fly_over_longitude,
            fly_over_longitude_weight,
            offset,
        );
    }
}

/// Triggered wrapper around `Viewer.zoomTo(target, offset?)`.
#[component(transparent)]
pub fn ViewerZoomToTarget(
    #[prop(into)] trigger: Signal<()>,
    #[prop(optional, into)] target: JsSignal<Option<ViewerTarget>>,
    /// Optional heading/pitch/range offset.
    #[prop(optional, into)]
    offset: Signal<Option<(f64, f64, f64)>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context =
            use_cesium_context().expect("ViewerZoomToTarget must be inside ViewerContainer");
        let mut is_first_run = true;

        Effect::new(move |_| {
            trigger.get();
            if is_first_run {
                is_first_run = false;
                return;
            }

            // Trigger-driven action: read all arguments untracked.
            let Some(target) = target.get_untracked() else {
                return;
            };
            let offset = offset.get_untracked();

            viewer_context.with_viewer(|viewer: Viewer| {
                let target_js = target.to_js_value();
                let _ = if let Some((heading, pitch, range)) = offset {
                    let offset = HeadingPitchRange::new(heading, pitch, range);
                    viewer.zoom_to_with_offset(&target_js, &JsValue::from(offset))
                } else {
                    viewer.zoom_to(&target_js)
                };
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (trigger, target, offset);
    }
}

/// Triggered setter for `viewer.clockTrackedDataSource`.
#[component(transparent)]
pub fn ViewerSetClockTrackedDataSource(
    #[prop(into)] trigger: Signal<()>,
    #[prop(optional, into)] data_source: JsSignal<Option<DataSource>>,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let viewer_context = use_cesium_context()
            .expect("ViewerSetClockTrackedDataSource must be inside ViewerContainer");
        let mut is_first_run = true;

        Effect::new(move |_| {
            trigger.get();
            if is_first_run {
                is_first_run = false;
                return;
            }

            let data_source = data_source.get_untracked();
            viewer_context.with_viewer(|viewer: Viewer| {
                viewer.set_clock_tracked_data_source(data_source.as_ref());
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (trigger, data_source);
    }
}
