//! Shared lifecycle helpers for ownership and async request gating.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// Tracks request generations and mount state so async completions can be ignored when stale.
#[derive(Debug, Clone)]
pub struct RequestGate {
    mounted: Arc<AtomicBool>,
    version: Arc<AtomicU64>,
}

impl RequestGate {
    /// Creates a new request gate in a mounted state.
    pub fn new() -> Self {
        Self {
            mounted: Arc::new(AtomicBool::new(true)),
            version: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Marks the start of a new request and returns its request id.
    pub fn begin_request(&self) -> u64 {
        self.version.fetch_add(1, Ordering::Relaxed).wrapping_add(1)
    }

    /// Returns true if a request id should no longer be applied.
    pub fn is_stale(&self, request_id: u64) -> bool {
        !self.mounted.load(Ordering::Relaxed) || self.version.load(Ordering::Relaxed) != request_id
    }

    /// Marks this gate as closed/unmounted and invalidates all in-flight requests.
    pub fn close(&self) {
        self.mounted.store(false, Ordering::Relaxed);
        self.version.fetch_add(1, Ordering::Relaxed);
    }
}

impl Default for RequestGate {
    fn default() -> Self {
        Self::new()
    }
}

/// Stores a single owned value and provides explicit replacement/cleanup hooks.
#[derive(Debug, Clone)]
pub struct OwnedSlot<T> {
    value: Option<T>,
}

impl<T> OwnedSlot<T> {
    /// Returns true when a value is currently owned.
    pub fn is_set(&self) -> bool {
        self.value.is_some()
    }

    /// Replaces the currently owned value.
    ///
    /// If a value already exists, `on_replace` is called before replacing it.
    pub fn replace_with<F>(&mut self, next: T, mut on_replace: F)
    where
        F: FnMut(&T),
    {
        if let Some(current) = self.value.as_ref() {
            on_replace(current);
        }
        self.value = Some(next);
    }

    /// Clears the currently owned value, calling `on_clear` if present.
    pub fn clear_with<F>(&mut self, mut on_clear: F)
    where
        F: FnMut(&T),
    {
        if let Some(current) = self.value.take() {
            on_clear(&current);
        }
    }
}

impl<T> Default for OwnedSlot<T> {
    fn default() -> Self {
        Self { value: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_gate_rejects_stale_requests_after_newer_request() {
        let gate = RequestGate::new();
        let first = gate.begin_request();
        let second = gate.begin_request();

        assert!(gate.is_stale(first));
        assert!(!gate.is_stale(second));
    }

    #[test]
    fn request_gate_rejects_requests_after_close() {
        let gate = RequestGate::new();
        let req = gate.begin_request();
        gate.close();

        assert!(gate.is_stale(req));
    }

    #[test]
    fn unmounting_one_data_source_does_not_remove_others() {
        let mut a = OwnedSlot::default();
        let mut b = OwnedSlot::default();
        let mut removed = Vec::new();

        a.replace_with("ds-a".to_string(), |_| {});
        b.replace_with("ds-b".to_string(), |_| {});
        a.clear_with(|v| removed.push(v.clone()));

        assert_eq!(removed, vec!["ds-a".to_string()]);
        assert!(b.is_set());
    }

    #[test]
    fn unmounting_one_primitive_does_not_remove_others() {
        let mut a = OwnedSlot::default();
        let mut b = OwnedSlot::default();
        let mut removed = Vec::new();

        a.replace_with(101_u32, |_| {});
        b.replace_with(202_u32, |_| {});
        b.clear_with(|v| removed.push(*v));

        assert_eq!(removed, vec![202_u32]);
        assert!(a.is_set());
    }

    #[test]
    fn listener_slot_detaches_on_cleanup_once() {
        let mut slot = OwnedSlot::default();
        let mut detached = Vec::new();

        slot.replace_with("listener-1".to_string(), |_| {});
        slot.clear_with(|v| detached.push(v.clone()));
        slot.clear_with(|v| detached.push(v.clone()));

        assert_eq!(detached, vec!["listener-1".to_string()]);
    }
}
