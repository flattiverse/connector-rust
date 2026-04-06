use crate::galaxy_hierarchy::{
    Controllable, Cost, DynamicShotLauncherSubsystem, RangeTolerance, SubsystemBase, SubsystemExt,
};
use crate::network::InvalidArgumentKind;
use crate::{FlattiverseEvent, GameError, GameErrorKind, SubsystemSlot, SubsystemStatus, Vector};
use std::sync::Weak;

/// Static shot launcher subsystem of a modern ship.
#[derive(Debug)]
pub struct StaticShotLauncherSubsystem {
    base: DynamicShotLauncherSubsystem,
}

impl StaticShotLauncherSubsystem {
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

    #[inline]
    pub fn relative_speed(&self) -> f32 {
        self.base.relative_movement().length()
    }

    /// Calculates the resource costs for one dynamic shot request.
    #[inline]
    pub fn calculate_cost(
        &self,
        relative_speed: f32,
        ticks: u16,
        load: f32,
        damage: f32,
    ) -> Option<Cost> {
        self.base.calculate_cost(
            Vector::from_angle_length(0.0, relative_speed),
            ticks,
            load,
            damage,
        )
    }

    pub async fn shoot(
        &self,
        relative_speed: f32,
        ticks: u16,
        load: f32,
        damage: f32,
    ) -> Result<(), GameError> {
        let controllable = self.controllable();

        if !controllable.active() || !self.exists() {
            Err(GameErrorKind::SpecifiedElementNotFound.into())
        } else if !controllable.active() {
            Err(GameErrorKind::YouNeedToContinueFirst.into())
        } else {
            let relative_speed = RangeTolerance::clamped_range(
                relative_speed,
                self.minimum_relative_movement(),
                self.maximum_relative_movement(),
            )
            .map_err(|reason| GameErrorKind::InvalidArgument {
                reason,
                parameter: "relative_speed".to_string(),
            })?;

            let ticks = if ticks < self.minimum_ticks() {
                return Err(GameErrorKind::InvalidArgument {
                    reason: InvalidArgumentKind::TooSmall,
                    parameter: "ticks".to_string(),
                }
                .into());
            } else if ticks > self.maximum_ticks() {
                return Err(GameErrorKind::InvalidArgument {
                    reason: InvalidArgumentKind::TooLarge,
                    parameter: "ticks".to_string(),
                }
                .into());
            } else {
                ticks
            };

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
                .static_shot_launcher_subsystem_shoot(
                    controllable.id(),
                    self.slot(),
                    relative_speed,
                    ticks,
                    load,
                    damage,
                )
                .await
        }
    }

    #[inline]
    pub(crate) fn reset_runtime(&self) {
        self.base.reset_runtime();
    }

    #[inline]
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

    #[inline]
    pub(crate) fn create_runtime_event(&self) -> Option<FlattiverseEvent> {
        self.base.create_runtime_event()
    }
}

impl AsRef<SubsystemBase> for StaticShotLauncherSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        self.base.as_ref()
    }
}
