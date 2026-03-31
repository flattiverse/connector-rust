use crate::utils::Atomic;
use std::sync::atomic::Ordering;

#[derive(Default, Debug)]
pub struct ProgressState {
    current: Atomic<i64>,
    total: Atomic<i64>,
    updates: Atomic<i64>,
    finished: Atomic<bool>,
}

impl ProgressState {
    /// Current amount already transferred or decoded by the ongoing chunked operation.
    #[inline]
    pub fn current(&self) -> i64 {
        self.current.load()
    }

    /// Total amount reported by the server for the current chunked operation.
    #[inline]
    pub fn total(&self) -> i64 {
        self.total.load()
    }

    /// Number of progress reports that have been applied to this instance.
    #[inline]
    pub fn updates(&self) -> i64 {
        self.updates.load()
    }

    /// Whether the connector has observed the final chunk of the operation.
    #[inline]
    pub fn finished(&self) -> bool {
        self.finished.load()
    }

    /// Progress normalized to the range `[0; 1]`.
    /// Returns `0` while no total is known yet and `1` once a zero-length operation finished.
    pub fn progress_normalized(&self) -> f32 {
        let total = self.total();
        if total == 0 {
            if self.finished() {
                1.0
            } else {
                0.0
            }
        } else {
            let current = self.current();
            current as f32 / total as f32
        }
    }

    pub(crate) fn reset(&self) {
        self.current.store(0);
        self.total.store(0);
        self.updates.store(0);
        self.finished.store(false);
    }

    pub(crate) fn report(&self, current: i64, total: i64) {
        self.current.store(current);
        self.total.store(total);
        self.updates
            .visit_container(|updates| updates.fetch_add(1, Ordering::Relaxed));
        self.finished.store(current == total);
    }
}
