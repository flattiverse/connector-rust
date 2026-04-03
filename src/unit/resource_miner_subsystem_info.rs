use crate::utils::Atomic;
use crate::SubsystemStatus;

/// Visible snapshot of a resource miner subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct ResourceMinerSubsystemInfo {
    exists: Atomic<bool>,
    minimum_rate: Atomic<f32>,
    maximum_rate: Atomic<f32>,
    rate: Atomic<f32>,
    status: Atomic<SubsystemStatus>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
    mined_metal_this_tick: Atomic<f32>,
    mined_carbon_this_tick: Atomic<f32>,
    mined_hydrogen_this_tick: Atomic<f32>,
    mined_silicon_this_tick: Atomic<f32>,
}

impl ResourceMinerSubsystemInfo {
    /// Indicates whether the subsystem exists on the scanned unit.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// Minimum configurable mining rate.
    #[inline]
    pub fn minimum_rate(&self) -> f32 {
        self.minimum_rate.load()
    }

    /// Maximum configurable mining rate.
    #[inline]
    pub fn maximum_rate(&self) -> f32 {
        self.maximum_rate.load()
    }

    /// Configured mining rate for the reported tick.
    #[inline]
    pub fn rate(&self) -> f32 {
        self.rate.load()
    }

    /// Tick-local runtime status reported for the resource miner subsystem.
    /// The miner can fail, for example when the ship moves too fast or when no valid body is in
    /// range.
    #[inline]
    pub fn status(&self) -> SubsystemStatus {
        self.status.load()
    }

    /// Energy consumed by mining during the reported tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// Ions consumed by mining during the reported tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// Neutrinos consumed by mining during the reported tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    /// Metal mined during the reported tick.
    #[inline]
    pub fn mined_metal_this_tick(&self) -> f32 {
        self.mined_metal_this_tick.load()
    }

    /// Carbon mined during the reported tick.
    #[inline]
    pub fn mined_carbon_this_tick(&self) -> f32 {
        self.mined_carbon_this_tick.load()
    }

    /// Hydrogen mined during the reported tick.
    #[inline]
    pub fn mined_hydrogen_this_tick(&self) -> f32 {
        self.mined_hydrogen_this_tick.load()
    }

    /// Silicon mined during the reported tick.
    #[inline]
    pub fn mined_silicon_this_tick(&self) -> f32 {
        self.mined_silicon_this_tick.load()
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
        mined_metal_this_tick: f32,
        mined_carbon_this_tick: f32,
        mined_hydrogen_this_tick: f32,
        mined_silicon_this_tick: f32,
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
            self.mined_metal_this_tick.store(mined_metal_this_tick);
            self.mined_carbon_this_tick.store(mined_carbon_this_tick);
            self.mined_hydrogen_this_tick
                .store(mined_hydrogen_this_tick);
            self.mined_silicon_this_tick.store(mined_silicon_this_tick);
        } else {
            self.minimum_rate.store(0.0);
            self.maximum_rate.store(0.0);
            self.rate.store(0.0);
            self.status.store(SubsystemStatus::Off);
            self.consumed_energy_this_tick.store(0.0);
            self.consumed_ions_this_tick.store(0.0);
            self.consumed_neutrinos_this_tick.store(0.0);
            self.mined_metal_this_tick.store(0.0);
            self.mined_carbon_this_tick.store(0.0);
            self.mined_hydrogen_this_tick.store(0.0);
            self.mined_silicon_this_tick.store(0.0);
        }
    }
}
