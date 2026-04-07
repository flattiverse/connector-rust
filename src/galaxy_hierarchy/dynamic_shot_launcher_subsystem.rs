use crate::galaxy_hierarchy::{
    Controllable, Cost, RangeTolerance, ShipBalancing, SubsystemBase, SubsystemExt,
};
use crate::network::InvalidArgumentKind;
use crate::utils::Atomic;
use crate::{
    FlattiverseEvent, FlattiverseEventKind, GameError, GameErrorKind, SubsystemSlot,
    SubsystemStatus, Vector,
};
use std::sync::Weak;

/// Dynamic projectile launcher subsystem of a controllable.
#[derive(Debug)]
pub struct DynamicShotLauncherSubsystem {
    base: SubsystemBase,
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
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl DynamicShotLauncherSubsystem {
    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: SubsystemBase::new(controllable, name, exists, slot),
            minimum_relative_movement: Atomic::from(0.1),
            maximum_relative_movement: Atomic::from(3.0),
            minimum_ticks: Atomic::from(2),
            maximum_ticks: Atomic::from(140),
            minimum_load: Atomic::from(2.5),
            maximum_load: Atomic::from(25.0),
            minimum_damage: Atomic::from(1.0),
            maximum_damage: Atomic::from(20.0),
            relative_movement: Atomic::default(),
            ticks: Atomic::default(),
            load: Atomic::default(),
            damage: Atomic::default(),
            consumed_energy_this_tick: Atomic::default(),
            consumed_ions_this_tick: Atomic::default(),
            consumed_neutrinos_this_tick: Atomic::default(),
        }
    }

    /// The minimum allowed relative shot speed.
    #[inline]
    pub fn minimum_relative_movement(&self) -> f32 {
        self.minimum_relative_movement.load()
    }

    /// The maximum allowed relative shot speed.
    #[inline]
    pub fn maximum_relative_movement(&self) -> f32 {
        self.maximum_relative_movement.load()
    }

    /// The minimum allowed shot lifetime in ticks.
    #[inline]
    pub fn minimum_ticks(&self) -> u16 {
        self.minimum_ticks.load()
    }

    /// The maximum allowed shot lifetime in ticks.
    #[inline]
    pub fn maximum_ticks(&self) -> u16 {
        self.maximum_ticks.load()
    }

    /// The minimum allowed shot load.
    #[inline]
    pub fn minimum_load(&self) -> f32 {
        self.minimum_load.load()
    }

    /// The maximum allowed shot load.
    #[inline]
    pub fn maximum_load(&self) -> f32 {
        self.maximum_load.load()
    }

    /// The minimum allowed shot damage.
    #[inline]
    pub fn minimum_damage(&self) -> f32 {
        self.minimum_damage.load()
    }

    /// The maximum allowed shot damage.
    #[inline]
    pub fn maximum_damage(&self) -> f32 {
        self.maximum_damage.load()
    }

    pub(crate) fn set_capabilities(
        &self,
        minimum_relative_movement: f32,
        maximum_relative_movement: f32,
        minimum_ticks: u16,
        maximum_ticks: u16,
        minimum_load: f32,
        maximum_load: f32,
        minimum_damage: f32,
        maximum_damage: f32,
    ) {
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

        // TODO self.refresh_tier();
    }

    /// The last server-side shot movement request processed for the current tick.
    #[inline]
    pub fn relative_movement(&self) -> Vector {
        self.relative_movement.load()
    }

    /// The last server-side shot lifetime processed for the current tick.
    #[inline]
    pub fn ticks(&self) -> u16 {
        self.ticks.load()
    }

    /// The last server-side shot load processed for the current tick.
    #[inline]
    pub fn load(&self) -> f32 {
        self.load.load()
    }

    /// The last server-side shot damage processed for the current tick.
    #[inline]
    pub fn damage(&self) -> f32 {
        self.damage.load()
    }

    /// The energy consumed by the launcher during the current server tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// The ions consumed by the launcher during the current server tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// The neutrinos consumed by the launcher during the current server tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    /// Calculates the resource costs for one dynamic shot request.
    pub fn calculate_cost(
        &self,
        relative_movement: Vector,
        ticks: u16,
        load: f32,
        damage: f32,
    ) -> Option<Cost> {
        if !self.exists() {
            return None;
        }

        let relative_movement = RangeTolerance::clamped_range_vector(
            relative_movement,
            self.minimum_relative_movement(),
            self.maximum_relative_movement(),
        )
        .ok()?;

        #[allow(clippy::manual_range_contains)]
        if ticks < self.minimum_ticks() || ticks > self.maximum_ticks() {
            return None;
        }

        let load =
            RangeTolerance::clamped_range(load, self.minimum_load(), self.maximum_load()).ok()?;

        let damage =
            RangeTolerance::clamped_range(damage, self.minimum_damage(), self.maximum_damage())
                .ok()?;

        Cost::default()
            .with_energy(ShipBalancing::calculate_shot_launch_energy(
                relative_movement.length(),
                ticks,
                load,
                damage,
            ))
            .into_values_checked()
    }

    /// Requests one shot for the next server tick.
    /// The vector length, load, and damage values are clipped if they are only slightly outside
    /// the configured range. The tick count is not clipped.
    pub async fn shoot(
        &self,
        relative_movement: Vector,
        ticks: u16,
        load: f32,
        damage: f32,
    ) -> Result<(), GameError> {
        let controllable = self.controllable();

        if !controllable.active() || !self.exists() {
            Err(GameErrorKind::SpecifiedElementNotFound.into())
        } else if !controllable.alive() {
            Err(GameErrorKind::YouNeedToContinueFirst.into())
        } else {
            let relative_movement = RangeTolerance::clamped_range_vector(
                relative_movement,
                self.minimum_relative_movement(),
                self.maximum_relative_movement(),
            )
            .map_err(|reason| GameErrorKind::InvalidArgument {
                reason,
                parameter: "relative_movement".to_string(),
            })?;

            if ticks < self.minimum_ticks() {
                return Err(GameErrorKind::InvalidArgument {
                    reason: InvalidArgumentKind::TooSmall,
                    parameter: "ticks".to_string(),
                }
                .into());
            }

            if ticks > self.maximum_ticks() {
                return Err(GameErrorKind::InvalidArgument {
                    reason: InvalidArgumentKind::TooLarge,
                    parameter: "ticks".to_string(),
                }
                .into());
            }

            let load =
                RangeTolerance::clamped_range(load, self.minimum_load(), self.maximum_load())
                    .map_err(|reason| GameErrorKind::InvalidArgument {
                        reason,
                        parameter: "load".to_string(),
                    })?;

            let damage =
                RangeTolerance::clamped_range(damage, self.minimum_damage(), self.maximum_damage())
                    .map_err(|reason| GameErrorKind::InvalidArgument {
                        reason,
                        parameter: "damage".to_string(),
                    })?;

            controllable
                .cluster()
                .galaxy()
                .connection()
                .dynamic_shot_launcher_subsystem_shoot(
                    controllable.id(),
                    relative_movement,
                    ticks,
                    load,
                    damage,
                )
                .await
        }
    }

    pub(crate) fn reset_runtime(&self) {
        self.relative_movement.store_default();
        self.ticks.store_default();
        self.load.store_default();
        self.damage.store_default();
        self.consumed_energy_this_tick.store_default();
        self.consumed_ions_this_tick.store_default();
        self.consumed_neutrinos_this_tick.store_default();
        self.base.reset_runtime_status();
    }

    pub(crate) fn update_runtime(
        &self,
        relative_movement: Vector,
        ticks: u16,
        load: f32,
        damage: f32,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.relative_movement.store(relative_movement);
        self.ticks.store(ticks);
        self.load.store(load);
        self.damage.store(damage);
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
                FlattiverseEventKind::DynamicShotLauncherSubsystem {
                    controllable: self.controllable(),
                    slot: self.slot(),
                    status: self.status(),
                    relative_movement: self.relative_movement(),
                    ticks: self.ticks(),
                    load: self.load(),
                    damage: self.damage(),
                    consumed_energy_this_tick: self.consumed_energy_this_tick(),
                    consumed_ions_this_tick: self.consumed_ions_this_tick(),
                    consumed_neutrinos_this_tick: self.consumed_neutrinos_this_tick(),
                }
                .into(),
            )
        }
    }

    // TODO pub fn refresh_tier(&self) {}
}

impl AsRef<SubsystemBase> for DynamicShotLauncherSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
