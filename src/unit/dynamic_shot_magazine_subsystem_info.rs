use crate::utils::Atomic;
use crate::SubsystemStatus;

/// Visible snapshot of a dynamic shot magazine subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct DynamicShotMagazineSubsystemInfo {
    exists: Atomic<bool>,
    maximum_shots: Atomic<f32>,
    current_shots: Atomic<f32>,
    status: Atomic<SubsystemStatus>,
}

impl DynamicShotMagazineSubsystemInfo {
    /// Indicates whether the subsystem exists on the scanned unit.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// The magazine capacity in shots.
    #[inline]
    pub fn maximum_shots(&self) -> f32 {
        self.maximum_shots.load()
    }

    /// Currently available ammunition measured in shots.
    #[inline]
    pub fn current_shots(&self) -> f32 {
        self.current_shots.load()
    }

    /// Tick-local runtime status reported for the shot magazine subsystem.
    #[inline]
    pub fn status(&self) -> SubsystemStatus {
        self.status.load()
    }

    pub(crate) fn update(
        &self,
        exists: bool,
        maximum_shots: f32,
        current_shots: f32,
        status: SubsystemStatus,
    ) {
        self.exists.store(exists);
        self.maximum_shots
            .store(if exists { maximum_shots } else { 0.0 });
        self.current_shots
            .store(if exists { current_shots } else { 0.0 });
        self.status
            .store(if exists { status } else { SubsystemStatus::Off });
    }
}
