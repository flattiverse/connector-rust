use crate::utils::Atomic;
use crate::SubsystemStatus;

/// Visible snapshot of an energy-cell
#[derive(Debug, Clone, Default)]
pub struct EnergyCellSubsystemInfo {
    exists: Atomic<bool>,
    efficiency: Atomic<f32>,
    collected_this_tick: Atomic<f32>,
    status: Atomic<SubsystemStatus>,
}

impl EnergyCellSubsystemInfo {
    /// Whether the subsystem exists.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// The charging efficiency.
    #[inline]
    pub fn efficiency(&self) -> f32 {
        self.efficiency.load()
    }

    /// The amount collected during the current server tick.
    #[inline]
    pub fn collected_this_tick(&self) -> f32 {
        self.collected_this_tick.load()
    }

    /// The status for the current server tick.
    #[inline]
    pub fn status(&self) -> SubsystemStatus {
        self.status.load()
    }

    pub(crate) fn update(
        &self,
        exists: bool,
        efficiency: f32,
        collected_this_tick: f32,
        status: SubsystemStatus,
    ) {
        self.exists.store(exists);
        if exists {
            self.efficiency.store(efficiency);
            self.collected_this_tick.store(collected_this_tick);
            self.status.store(status);
        } else {
            self.efficiency.store_default();
            self.collected_this_tick.store_default();
            self.status.store_default();
        }
    }
}
