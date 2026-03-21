use crate::utils::Atomic;
use crate::{SubsystemStatus, Vector};

/// Visible snapshot of a projectile weapon subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct ShotWeaponSubsystemInfo {
    exists: Atomic<bool>,
    minimum_relative_movement: Atomic<f32>,
    maximum_relative_movement: Atomic<f32>,
    minimum_ticks: Atomic<u16>,
    maximum_ticks: Atomic<u16>,
    minimum_load: Atomic<f32>,
    maximum_load: Atomic<f32>,
    minimum_damage: Atomic<f32>,
    maximum_damage: Atomic<f32>,
    relative_movement: Atomic<Vector>,
    ticks: Atomic<u16>,
    load: Atomic<f32>,
    damage: Atomic<f32>,
    status: Atomic<SubsystemStatus>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl ShotWeaponSubsystemInfo {
    /// Whether the subsystem exists.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// Minimum allowed relative movement for the shot.
    #[inline]
    pub fn minimum_relative_movement(&self) -> f32 {
        self.minimum_relative_movement.load()
    }

    /// Maximum allowed relative movement for the shot.
    #[inline]
    pub fn maximum_relative_movement(&self) -> f32 {
        self.maximum_relative_movement.load()
    }

    /// Minimum lifetime in ticks.
    #[inline]
    pub fn minimum_ticks(&self) -> u16 {
        self.minimum_ticks.load()
    }

    /// Maximum lifetime in ticks.
    #[inline]
    pub fn maximum_ticks(&self) -> u16 {
        self.maximum_ticks.load()
    }

    /// Minimum explosion load.
    #[inline]
    pub fn minimum_load(&self) -> f32 {
        self.minimum_load.load()
    }

    /// Maximum explosion load.
    #[inline]
    pub fn maximum_load(&self) -> f32 {
        self.maximum_load.load()
    }

    /// Minimum damage.
    #[inline]
    pub fn minimum_damage(&self) -> f32 {
        self.minimum_damage.load()
    }

    /// Maximum damage.
    #[inline]
    pub fn maximum_damage(&self) -> f32 {
        self.maximum_damage.load()
    }

    /// Reported movement vector for the last shot request.
    #[inline]
    pub fn relative_movement(&self) -> Vector {
        self.relative_movement.load()
    }

    /// Lifetime of the processed shot.
    #[inline]
    pub fn ticks(&self) -> u16 {
        self.ticks.load()
    }

    /// Explosion load applied when the shot expires.
    #[inline]
    pub fn load(&self) -> f32 {
        self.load.load()
    }

    /// Damage applied by the reported shot.
    #[inline]
    pub fn damage(&self) -> f32 {
        self.damage.load()
    }

    /// Status of the reported weapon system.
    #[inline]
    pub fn status(&self) -> SubsystemStatus {
        self.status.load()
    }

    /// Energy consumed by the shot during the tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// Ions consumed by teh shot during the tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// Neutrinos consumed by the shot during the tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    pub(crate) fn update(
        &self,
        exists: bool,
        minimum_relative_movement: f32,
        maximum_relative_movement: f32,
        minimum_ticks: u16,
        maximum_ticks: u16,
        minimum_load: f32,
        maximum_load: f32,
        minimum_damage: f32,
        maximum_damage: f32,
        relative_movement: Vector,
        ticks: u16,
        load: f32,
        damage: f32,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.exists.store(exists);
        if exists {
            self.minimum_relative_movement
                .store(minimum_relative_movement);
            self.maximum_relative_movement
                .store(maximum_relative_movement);
            self.minimum_ticks.store(minimum_ticks);
            self.maximum_ticks.store(maximum_ticks);
            self.minimum_load.store(minimum_load);
            self.maximum_load.store(maximum_load);
            self.minimum_damage.store(minimum_damage);
            self.maximum_damage.store(maximum_damage);
            self.relative_movement.store(relative_movement);
            self.ticks.store(ticks);
            self.load.store(load);
            self.damage.store(damage);
            self.status.store(status);
            self.consumed_energy_this_tick
                .store(consumed_energy_this_tick);
            self.consumed_ions_this_tick.store(consumed_ions_this_tick);
            self.consumed_neutrinos_this_tick
                .store(consumed_neutrinos_this_tick);
        } else {
            self.minimum_relative_movement.store_default();
            self.maximum_relative_movement.store_default();
            self.minimum_ticks.store_default();
            self.maximum_ticks.store_default();
            self.minimum_load.store_default();
            self.maximum_load.store_default();
            self.minimum_damage.store_default();
            self.maximum_damage.store_default();
            self.relative_movement.store_default();
            self.ticks.store_default();
            self.load.store_default();
            self.damage.store_default();
            self.status.store_default();
            self.consumed_energy_this_tick.store_default();
            self.consumed_ions_this_tick.store_default();
            self.consumed_neutrinos_this_tick.store_default();
        }
    }
}
