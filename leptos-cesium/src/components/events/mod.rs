//! Event builders for Cesium viewer and entities.

/// Placeholder macro for Cesium event bindings.
#[macro_export]
macro_rules! cesium_events {
    ($($tt:tt)*) => {
        compile_error!("Cesium events macro not implemented yet");
    };
}

pub use cesium_events;
