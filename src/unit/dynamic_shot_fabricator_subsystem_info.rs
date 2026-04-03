use crate::utils::Atomic;
use crate::SubsystemStatus;

/// Visible snapshot of a dynamic shot fabricator subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct DynamicShotFabricatorSubsystemInfo {
    exists: Atomic<bool>,
    minimum_rate: Atomic<f32>,
    maximum_rate: Atomic<f32>,
    active: Atomic<bool>,
    rate: Atomic<f32>,
    status: Atomic<SubsystemStatus>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl DynamicShotFabricatorSubsystemInfo {
    /// Indicates whether the subsystem exists on the scanned unit.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// The minimum configurable shot fabrication rate.
    #[inline]
    pub fn minimum_rate(&self) -> f32 {
        self.minimum_rate.load()
    }

    /// The maximum configurable shot fabrication rate.
    #[inline]
    pub fn maximum_rate(&self) -> f32 {
        self.maximum_rate.load()
    }

    /// Whether the fabricator was active during the reported tick.
    /// This is separate from [`Self::rate()`] because a non-zero configured rate can still be
    /// inactive.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    /// Configured shot fabrication rate.
    #[inline]
    pub fn rate(&self) -> f32 {
        self.rate.load()
    }

    /// Tick-local runtime status reported for the shot fabricator subsystem.
    #[inline]
    pub fn status(&self) -> SubsystemStatus {
        self.status.load()
    }

    /// Energy consumed by fabrication during the reported tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// Ions consumed by fabrication during the reported tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// Neutrinos consumed by fabrication during the reported tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    pub(crate) fn update(
        &self,
        exists: bool,
        minimum_rate: f32,
        maximum_rate: f32,
        active: bool,
        rate: f32,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.exists.store(exists);
        self.minimum_rate
            .store(if exists { minimum_rate } else { 0.0 });
        self.maximum_rate
            .store(if exists { maximum_rate } else { 0.0 });
        self.active.store(exists && active);
        self.rate.store(if exists { rate } else { 0.0 });
        self.status
            .store(if exists { status } else { SubsystemStatus::Off });
        self.consumed_energy_this_tick.store(if exists {
            consumed_energy_this_tick
        } else {
            0.0
        });
        self.consumed_ions_this_tick
            .store(if exists { consumed_ions_this_tick } else { 0.0 });
        self.consumed_neutrinos_this_tick.store(if exists {
            consumed_neutrinos_this_tick
        } else {
            0.0
        });
    }
}
