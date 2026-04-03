use crate::utils::Atomic;
use crate::SubsystemStatus;

/// Visible snapshot of a nebula collector subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct NebulaCollectorSubsystemInfo {
    exists: Atomic<bool>,
    minimum_rate: Atomic<f32>,
    maximum_rate: Atomic<f32>,
    rate: Atomic<f32>,
    status: Atomic<SubsystemStatus>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
    collected_this_tick: Atomic<f32>,
    collected_hue_this_tick: Atomic<f32>,
}

impl NebulaCollectorSubsystemInfo {
    /// Indicates whether the scanned unit actually has a nebula collector installed.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// Minimum configurable collection rate for the scanned unit.
    #[inline]
    pub fn minimum_rate(&self) -> f32 {
        self.minimum_rate.load()
    }

    /// Maximum configurable collection rate for the scanned unit.
    #[inline]
    pub fn maximum_rate(&self) -> f32 {
        self.maximum_rate.load()
    }

    /// Collector rate mirrored for the reported tick.
    #[inline]
    pub fn rate(&self) -> f32 {
        self.rate.load()
    }

    /// Tick-local runtime status reported for the collector.
    /// The collector can fail or switch off when movement, environment, or resource conditions do
    /// not allow collection.
    #[inline]
    pub fn status(&self) -> SubsystemStatus {
        self.status.load()
    }

    /// Energy consumed by the collector during the reported tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// Ions consumed by the collector during the reported tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// Neutrinos consumed by the collector during the reported tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    /// Nebula amount collected during the reported tick.
    #[inline]
    pub fn collected_this_tick(&self) -> f32 {
        self.collected_this_tick.load()
    }

    /// Hue of the nebula sample collected during the reported tick.
    /// This describes the fresh intake and can differ from the averaged cargo hue stored in
    /// [`CargoSubsystemInfo::nebula_hue()`].
    ///
    /// [`CargoSubsystemInfo::nebula_hue()`]: crate::unit::CargoSubsystemInfo::nebula_hue
    #[inline]
    pub fn collected_hue_this_tick(&self) -> f32 {
        self.collected_hue_this_tick.load()
    }

    pub(crate) fn update(
        &self,
        exists: bool,
        minimum_rate: f32,
        maximum_rate: f32,
        rate: f32,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
        collected_this_tick: f32,
        collected_hue_this_tick: f32,
    ) {
        self.exists.store(exists);
        if exists {
            self.minimum_rate.store(minimum_rate);
            self.maximum_rate.store(maximum_rate);
            self.rate.store(rate);
            self.status.store(status);
            self.consumed_energy_this_tick
                .store(consumed_energy_this_tick);
            self.consumed_ions_this_tick.store(consumed_ions_this_tick);
            self.consumed_neutrinos_this_tick
                .store(consumed_neutrinos_this_tick);
            self.collected_this_tick.store(collected_this_tick);
            self.collected_hue_this_tick.store(collected_hue_this_tick);
        } else {
            self.minimum_rate.store(0.0);
            self.maximum_rate.store(0.0);
            self.rate.store(0.0);
            self.status.store(SubsystemStatus::Off);
            self.consumed_energy_this_tick.store(0.0);
            self.consumed_ions_this_tick.store(0.0);
            self.consumed_neutrinos_this_tick.store(0.0);
            self.collected_this_tick.store(0.0);
            self.collected_hue_this_tick.store(0.0);
        }
    }
}
