use crate::utils::Atomic;
use crate::SubsystemStatus;

/// Visible snapshot of a shield subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct ShieldSubsystemInfo {
    exists: Atomic<bool>,
    maximum: Atomic<f32>,
    current: Atomic<f32>,
    active: Atomic<bool>,
    rate: Atomic<f32>,
    status: Atomic<SubsystemStatus>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl ShieldSubsystemInfo {
    /// Indicates whether the subsystem exists on the scanned unit.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// The maximum shield integrity.
    #[inline]
    pub fn maximum(&self) -> f32 {
        self.maximum.load()
    }

    /// The current shield integrity.
    #[inline]
    pub fn current(&self) -> f32 {
        self.current.load()
    }

    /// Whether shield loading was active for the reported tick.
    /// A shield can exist while being inactive, for example when its configured rate is zero.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    /// Configured shield loading rate.
    /// Higher rates charge faster but also increase the quadratic tick cost.
    #[inline]
    pub fn rate(&self) -> f32 {
        self.rate.load()
    }

    /// Tick-local runtime status reported for the shield subsystem.
    #[inline]
    pub fn status(&self) -> SubsystemStatus {
        self.status.load()
    }

    /// Energy consumed by shield loading during the reported tick.
    /// This is usually zero if the shield was inactive or already full.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// Ions consumed by shield loading during the reported tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// Neutrinos consumed by shield loading during the reported tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    pub(crate) fn update(
        &self,
        exists: bool,
        maximum: f32,
        current: f32,
        active: bool,
        rate: f32,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.exists.store(exists);
        self.maximum.store(if exists { maximum } else { 0.0 });
        self.current.store(if exists { current } else { 0.0 });
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
