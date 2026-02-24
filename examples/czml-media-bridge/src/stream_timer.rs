use leptos::wasm_bindgen::{JsCast, JsValue, closure::Closure};
use leptos::web_sys;

pub struct StreamTimer {
    id: i32,
    _callback: Closure<dyn FnMut()>,
}

impl StreamTimer {
    pub fn start(interval_ms: i32, callback: impl FnMut() + 'static) -> Result<Self, String> {
        let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);

        let Some(window) = web_sys::window() else {
            return Err("Window not available for stream timer".to_string());
        };

        let id = window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                interval_ms,
            )
            .map_err(js_error_to_string)?;

        Ok(Self {
            id,
            _callback: callback,
        })
    }

    pub fn clear(self) {
        if let Some(window) = web_sys::window() {
            window.clear_interval_with_handle(self.id);
        }
    }
}

pub fn clear_timer_slot(slot: &mut Option<StreamTimer>) {
    if let Some(timer) = slot.take() {
        timer.clear();
    }
}

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
