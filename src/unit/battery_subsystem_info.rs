use crate::utils::Atomic;
use crate::SubsystemStatus;

/// Visible snapshot of a battery subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct BatterySubsystemInfo {
    exists: Atomic<bool>,
    maximum: Atomic<f32>,
    current: Atomic<f32>,
    consumed_this_tick: Atomic<f32>,
    status: Atomic<SubsystemStatus>,
}

impl BatterySubsystemInfo {
    /// Whether the subsystem exists.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// The maximum storable amount.
    #[inline]
    pub fn maximum(&self) -> f32 {
        self.maximum.load()
    }

    /// The currently stored amount.
    #[inline]
    pub fn current(&self) -> f32 {
        self.current.load()
    }

    /// The amount consumed during the current server tick.
    #[inline]
    pub fn consumed_this_tick(&self) -> f32 {
        self.consumed_this_tick.load()
    }

    /// The status for the current server tick.
    #[inline]
    pub fn status(&self) -> SubsystemStatus {
        self.status.load()
    }

    pub(crate) fn update(
        &self,
        exists: bool,
        maximum: f32,
        current: f32,
        consumed_this_tick: f32,
        status: SubsystemStatus,
    ) {
        self.exists.store(exists);
        if exists {
            self.maximum.store(maximum);
            self.current.store(current);
            self.consumed_this_tick.store(consumed_this_tick);
            self.status.store(status);
        } else {
            self.maximum.store_default();
            self.current.store_default();
            self.consumed_this_tick.store_default();
            self.status.store_default();
        }
    }
}
