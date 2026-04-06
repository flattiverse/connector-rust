use crate::galaxy_hierarchy::{
    AsSubsystemBase, Controllable, Cost, DynamicShotLauncherSubsystem, RangeTolerance,
    SubsystemBase, SubsystemExt,
};
use crate::network::InvalidArgumentKind;
use crate::{
    FlattiverseEvent, FlattiverseEventKind, GameError, GameErrorKind, SubsystemSlot,
    SubsystemStatus, Vector,
};
use std::sync::Weak;

/// Dynamic interceptor launcher subsystem of a controllable.
#[derive(Debug)]
pub struct DynamicInterceptorLauncherSubsystem {
    base: DynamicShotLauncherSubsystem,
}

impl DynamicInterceptorLauncherSubsystem {
    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: DynamicShotLauncherSubsystem::new(controllable, name, exists, slot),
        }
    }

    /// The minimum allowed relative shot speed.
    #[inline]
    pub fn minimum_relative_movement(&self) -> f32 {
        self.base.minimum_relative_movement()
    }

    /// The maximum allowed relative shot speed.
    #[inline]
    pub fn maximum_relative_movement(&self) -> f32 {
        self.base.maximum_relative_movement()
    }

    /// The minimum allowed shot lifetime in ticks.
    #[inline]
    pub fn minimum_ticks(&self) -> u16 {
        self.base.minimum_ticks()
    }

    /// The maximum allowed shot lifetime in ticks.
    #[inline]
    pub fn maximum_ticks(&self) -> u16 {
        self.base.maximum_ticks()
    }

    /// The minimum allowed shot load.
    #[inline]
    pub fn minimum_load(&self) -> f32 {
        self.base.minimum_load()
    }

    /// The maximum allowed shot load.
    #[inline]
    pub fn maximum_load(&self) -> f32 {
        self.base.maximum_load()
    }

    /// The minimum allowed shot damage.
    #[inline]
    pub fn minimum_damage(&self) -> f32 {
        self.base.minimum_damage()
    }

    /// The maximum allowed shot damage.
    #[inline]
    pub fn maximum_damage(&self) -> f32 {
        self.base.maximum_damage()
    }

    /// The last server-side shot movement request processed for the current tick.
    #[inline]
    pub fn relative_movement(&self) -> Vector {
        self.base.relative_movement()
    }

    /// The last server-side shot lifetime processed for the current tick.
    #[inline]
    pub fn ticks(&self) -> u16 {
        self.base.ticks()
    }

    /// The last server-side shot load processed for the current tick.
    #[inline]
    pub fn load(&self) -> f32 {
        self.base.load()
    }

    /// The last server-side shot damage processed for the current tick.
    #[inline]
    pub fn damage(&self) -> f32 {
        self.base.damage()
    }

    /// The energy consumed by the launcher during the current server tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.base.consumed_energy_this_tick()
    }

    /// The ions consumed by the launcher during the current server tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.base.consumed_ions_this_tick()
    }

    /// The neutrinos consumed by the launcher during the current server tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.base.consumed_neutrinos_this_tick()
    }

    /// Calculates the resource costs for one dynamic shot request.
    #[inline]
    pub fn calculate_cost(
        &self,
        relative_movement: Vector,
        ticks: u16,
        load: f32,
        damage: f32,
    ) -> Option<Cost> {
        self.base
            .calculate_cost(relative_movement, ticks, load, damage)
    }

    /// Requests one interceptor for the next server tick.
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
                .dynamic_shot_interceptor_subsystem_shoot(
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
        self.base.reset_runtime();
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
        self.base.update_runtime(
            relative_movement,
            ticks,
            load,
            damage,
            status,
            consumed_energy_this_tick,
            consumed_ions_this_tick,
            consumed_neutrinos_this_tick,
        );
    }

    pub(crate) fn create_runtime_event(&self) -> Option<FlattiverseEvent> {
        if !self.exists() || !self.as_subsystem_base().should_emit_runtime_event() {
            None
        } else {
            Some(
                FlattiverseEventKind::DynamicInterceptorLauncherSubsystem {
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
}

impl AsRef<SubsystemBase> for DynamicInterceptorLauncherSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        self.base.as_ref()
    }
}
