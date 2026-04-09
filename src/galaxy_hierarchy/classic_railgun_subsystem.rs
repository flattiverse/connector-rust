use crate::galaxy_hierarchy::{Controllable, Cost, RailgunDirection, SubsystemBase, SubsystemExt};
use crate::utils::Atomic;
use crate::{
    FlattiverseEvent, FlattiverseEventKind, GameError, GameErrorKind, SubsystemSlot,
    SubsystemStatus,
};
use std::sync::Weak;

/// Railgun subsystem of a controllable.
#[derive(Debug)]
pub struct ClassicRailgunSubsystem {
    base: SubsystemBase,
    projectile_speed: Atomic<f32>,
    projectile_lifetime: Atomic<u16>,
    energy_cost: Atomic<f32>,
    metal_cost: Atomic<f32>,
    direction: Atomic<RailgunDirection>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl ClassicRailgunSubsystem {
    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: SubsystemBase::new(controllable, name, exists, slot),
            projectile_speed: Atomic::from(4.0),
            projectile_lifetime: Atomic::from(250),
            energy_cost: Atomic::from(300.0),
            metal_cost: Atomic::from(1.0),
            direction: Default::default(),
            consumed_energy_this_tick: Default::default(),
            consumed_ions_this_tick: Default::default(),
            consumed_neutrinos_this_tick: Default::default(),
        }
    }

    /// Rail projectile relative speed.
    #[inline]
    pub fn projectile_speed(&self) -> f32 {
        self.projectile_speed.load()
    }

    /// Rail projectile lifetime in ticks.
    #[inline]
    pub fn projectile_lifetime(&self) -> u16 {
        self.projectile_lifetime.load()
    }

    /// Energy consumed by one rail shot.
    #[inline]
    pub fn energy_cost(&self) -> f32 {
        self.energy_cost.load()
    }

    /// Metal consumed by one rail shot.
    #[inline]
    pub fn metal_cost(&self) -> f32 {
        self.metal_cost.load()
    }

    #[instrument(level = "debug", skip(self))]
    pub(crate) fn set_capabilities(
        &self,
        projectile_speed: f32,
        projectile_lifetime: u16,
        energy_cost: f32,
        metal_cost: f32,
    ) {
        if self.exists() {
            self.projectile_speed.store(projectile_speed);
            self.projectile_lifetime.store(projectile_lifetime);
            self.energy_cost.store(energy_cost);
            self.metal_cost.store(metal_cost);
        } else {
            self.projectile_speed.store(0.0);
            self.projectile_lifetime.store(0);
            self.energy_cost.store(0.0);
            self.metal_cost.store(0.0);
        }

        // TODO self.refresh_tier();
    }

    /// The direction processed during the current server tick.
    #[inline]
    pub fn direction(&self) -> RailgunDirection {
        self.direction.load()
    }

    /// The energy consumed by the railgun during the current server tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// The ions consumed by the railgun during the current server tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// The neutrinos consumed by the railgun during the current server tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    /// Calculates the resource costs for one rail shot.
    pub fn calculate_cost(&self) -> Option<Cost> {
        if !self.exists() {
            None
        } else {
            Some(Cost {
                energy: self.energy_cost(),
                ions: 0.0,
                neutrinos: 0.0,
            })
        }
    }

    /// Fires the railgun forward.
    pub async fn fire_front(&self) -> Result<(), GameError> {
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
                .fire_railgun_subsystem_front(controllable.id())
                .await
        }
    }

    /// Fires the railgun backward.
    pub async fn fire_back(&self) -> Result<(), GameError> {
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
                .fire_railgun_subsystem_back(controllable.id())
                .await
        }
    }

    #[instrument(level = "debug", skip(self))]
    pub(crate) fn reset_runtime(&self) {
        self.direction.store_default();
        self.consumed_energy_this_tick.store_default();
        self.consumed_ions_this_tick.store_default();
        self.consumed_neutrinos_this_tick.store_default();
        self.base.reset_runtime_status();
    }

    #[instrument(level = "debug", skip(self))]
    pub(crate) fn update_runtime(
        &self,
        direction: RailgunDirection,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.direction.store(direction);
        self.consumed_energy_this_tick
            .store(consumed_energy_this_tick);
        self.consumed_ions_this_tick.store(consumed_ions_this_tick);
        self.consumed_neutrinos_this_tick
            .store(consumed_neutrinos_this_tick);
        self.base.update_runtime_status(status);
    }

    pub(crate) fn create_runtime_event(&self) -> Option<FlattiverseEvent> {
        if !self.exists() || !self.base.should_emit_runtime_event() {
            None
        } else {
            Some(
                FlattiverseEventKind::ClassicRailgunSubsystem {
                    controllable: self.controllable(),
                    slot: self.slot(),
                    status: self.status(),
                    direction: self.direction(),
                    consumed_energy_this_tick: self.consumed_energy_this_tick(),
                    consumed_ions_this_tick: self.consumed_ions_this_tick(),
                    consumed_neutrinos_this_tick: self.consumed_neutrinos_this_tick(),
                }
                .into(),
            )
        }
    }
}

impl AsRef<SubsystemBase> for ClassicRailgunSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
