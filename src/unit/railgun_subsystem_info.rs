use crate::galaxy_hierarchy::RailgunDirection;
use crate::utils::Atomic;
use crate::SubsystemStatus;

/// Visible snapshot of a railgun subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct RailgunSubsystemInfo {
    exists: Atomic<bool>,
    energy_cost: Atomic<f32>,
    metal_cost: Atomic<f32>,
    direction: Atomic<RailgunDirection>,
    status: Atomic<SubsystemStatus>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl RailgunSubsystemInfo {
    /// Indicates whether the subsystem exists on the scanned unit.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// The energy cost per rail shot.
    #[inline]
    pub fn energy_cost(&self) -> f32 {
        self.energy_cost.load()
    }

    /// The metal cost per rail shot.
    #[inline]
    pub fn metal_cost(&self) -> f32 {
        self.metal_cost.load()
    }

    /// Direction fired or processed during the reported tick.
    /// The current railgun model uses a fixed front/back direction choice instead of a freely aimed
    /// vector.
    #[inline]
    pub fn direction(&self) -> RailgunDirection {
        self.direction.load()
    }

    /// Tick-local runtime status reported for the railgun subsystem.
    #[inline]
    pub fn status(&self) -> SubsystemStatus {
        self.status.load()
    }

    /// Energy consumed by the railgun during the reported tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// Ions consumed by the railgun during the reported tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// Neutrinos consumed by the railgun during the reported tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    pub(crate) fn update(
        &self,
        exists: bool,
        energy_cost: f32,
        metal_cost: f32,
        direction: RailgunDirection,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.exists.store(exists);
        if exists {
            self.energy_cost.store(energy_cost);
            self.metal_cost.store(metal_cost);
            self.direction.store(direction);
            self.status.store(status);
            self.consumed_energy_this_tick
                .store(consumed_energy_this_tick);
            self.consumed_ions_this_tick.store(consumed_ions_this_tick);
            self.consumed_neutrinos_this_tick
                .store(consumed_neutrinos_this_tick);
        } else {
            self.energy_cost.store(0.0);
            self.metal_cost.store(0.0);
            self.direction.store(RailgunDirection::None);
            self.status.store(SubsystemStatus::Off);
            self.consumed_energy_this_tick.store(0.0);
            self.consumed_ions_this_tick.store(0.0);
            self.consumed_neutrinos_this_tick.store(0.0);
        }
    }
}
