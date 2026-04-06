use crate::galaxy_hierarchy::{
    Controllable, Cost, RailgunDirection, RailgunSubsystem, SubsystemBase, SubsystemExt,
};
use crate::{FlattiverseEvent, GameError, GameErrorKind, SubsystemSlot, SubsystemStatus};
use std::sync::Weak;

/// Railgun subsystem of a modern ship.
#[derive(Debug)]
pub struct ModernRailgunSubsystem {
    base: RailgunSubsystem,
}

impl ModernRailgunSubsystem {
    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: RailgunSubsystem::new(controllable, name, exists, slot),
        }
    }

    /// Rail projectile relative speed.
    #[inline]
    pub fn projectile_speed(&self) -> f32 {
        self.base.projectile_speed()
    }

    /// Rail projectile lifetime in ticks.
    #[inline]
    pub fn projectile_lifetime(&self) -> f32 {
        self.base.projectile_lifetime()
    }

    /// Energy consumed by one rail shot.
    #[inline]
    pub fn energy_cost(&self) -> f32 {
        self.base.energy_cost()
    }

    /// Metal consumed by one rail shot.
    #[inline]
    pub fn metal_cost(&self) -> f32 {
        self.base.metal_cost()
    }

    /// The direction processed during the current server tick.
    #[inline]
    pub fn direction(&self) -> RailgunDirection {
        self.base.direction()
    }

    /// The energy consumed by the railgun during the current server tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.base.consumed_energy_this_tick()
    }

    /// The ions consumed by the railgun during the current server tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.base.consumed_ions_this_tick()
    }

    /// The neutrinos consumed by the railgun during the current server tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.base.consumed_neutrinos_this_tick()
    }

    /// Calculates the resource costs for one rail shot.
    #[inline]
    pub fn calculate_cost(&self) -> Option<Cost> {
        self.base.calculate_cost()
    }

    /// Fires the railgun.
    pub async fn fire(&self) -> Result<(), GameError> {
        let controllable = self.controllable();

        if !controllable.active() || !self.exists() {
            Err(GameErrorKind::SpecifiedElementNotFound.into())
        } else if !controllable.alive() {
            Err(GameErrorKind::YouNeedToContinueFirst.into())
        } else {
            controllable
                .cluster()
                .galaxy()
                .connection()
                .modern_railgun_subsystem_fire(controllable.id(), self.slot())
                .await
        }
    }

    #[inline]
    pub(crate) fn reset_runtime(&self) {
        self.base.reset_runtime()
    }

    #[inline]
    pub(crate) fn update_runtime(
        &self,
        direction: RailgunDirection,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.base.update_runtime(
            direction,
            status,
            consumed_energy_this_tick,
            consumed_ions_this_tick,
            consumed_neutrinos_this_tick,
        );
    }

    #[inline]
    pub(crate) fn create_runtime_event(&self) -> Option<FlattiverseEvent> {
        self.base.create_runtime_event()
    }
}

impl AsRef<SubsystemBase> for ModernRailgunSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        self.base.as_ref()
    }
}
