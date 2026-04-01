use crate::galaxy_hierarchy::{Controllable, Cost, RangeTolerance, SubsystemBase, SubsystemExt};
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
    relative_movement: Atomic<Vector>,
    ticks: Atomic<u16>,
    load: Atomic<f32>,
    damage: Atomic<f32>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl DynamicShotLauncherSubsystem {
    const RELATIVE_MOVEMENT_MINIMUM: f32 = 0.1;
    const RELATIVE_MOVEMENT_MAXIMUM: f32 = 3.0;
    const TICKS_MINIMUM: u16 = 2;
    const TICKS_MAXIMUM: u16 = 140;
    const LOAD_MINIMUM: f32 = 2.5;
    const LOAD_MAXIMUM: f32 = 25.0;
    const DAMAGE_MINIMUM: f32 = 1.0;
    const DAMAGE_MAXIMUM: f32 = 20.0;

    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: SubsystemBase::new(controllable, name, exists, slot),
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
        Self::RELATIVE_MOVEMENT_MINIMUM
    }

    /// The maximum allowed relative shot speed.
    #[inline]
    pub fn maximum_relative_movement(&self) -> f32 {
        Self::RELATIVE_MOVEMENT_MAXIMUM
    }

    /// The minimum allowed shot lifetime in ticks.
    #[inline]
    pub fn minimum_ticks(&self) -> u16 {
        Self::TICKS_MINIMUM
    }

    /// The maximum allowed shot lifetime in ticks.
    #[inline]
    pub fn maximum_ticks(&self) -> u16 {
        Self::TICKS_MAXIMUM
    }

    /// The minimum allowed shot load.
    #[inline]
    pub fn minimum_load(&self) -> f32 {
        Self::LOAD_MINIMUM
    }

    /// The maximum allowed shot load.
    #[inline]
    pub fn maximum_load(&self) -> f32 {
        Self::LOAD_MAXIMUM
    }

    /// The minimum allowed shot damage.
    #[inline]
    pub fn minimum_damage(&self) -> f32 {
        Self::DAMAGE_MINIMUM
    }

    /// The maximum allowed shot damage.
    #[inline]
    pub fn maximum_damage(&self) -> f32 {
        Self::DAMAGE_MAXIMUM
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
            Self::RELATIVE_MOVEMENT_MINIMUM,
            Self::RELATIVE_MOVEMENT_MAXIMUM,
        )
        .ok()?;

        #[allow(clippy::manual_range_contains)]
        if ticks < Self::TICKS_MINIMUM || ticks > Self::TICKS_MAXIMUM {
            return None;
        }

        let load =
            RangeTolerance::clamped_range(load, Self::LOAD_MINIMUM, Self::LOAD_MAXIMUM).ok()?;
        let damage =
            RangeTolerance::clamped_range(damage, Self::DAMAGE_MINIMUM, Self::DAMAGE_MAXIMUM)
                .ok()?;

        let speed = relative_movement.length();
        let speed01 = (speed - Self::RELATIVE_MOVEMENT_MINIMUM)
            / (Self::RELATIVE_MOVEMENT_MAXIMUM - Self::RELATIVE_MOVEMENT_MINIMUM);
        let ticks01 = (ticks - Self::TICKS_MINIMUM) as f32
            / (Self::TICKS_MAXIMUM - Self::TICKS_MINIMUM) as f32;
        let load01 = (load - Self::LOAD_MINIMUM) / (Self::LOAD_MAXIMUM - Self::LOAD_MINIMUM);
        let damage01 =
            (damage - Self::DAMAGE_MINIMUM) / (Self::DAMAGE_MAXIMUM - Self::DAMAGE_MINIMUM);

        Cost::default()
            .with_energy(
                10.0 + 250.0 * speed01 * speed01 * speed01
                    + 240.0 * ticks01 * ticks01
                    + 600.0 * load01 * load01
                    + 700.0 * damage01 * damage01,
            )
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
                Self::RELATIVE_MOVEMENT_MINIMUM,
                Self::RELATIVE_MOVEMENT_MAXIMUM,
            )
            .map_err(|reason| GameErrorKind::InvalidArgument {
                reason,
                parameter: "relative_movement".to_string(),
            })?;

            if ticks < Self::TICKS_MINIMUM {
                return Err(GameErrorKind::InvalidArgument {
                    reason: InvalidArgumentKind::TooSmall,
                    parameter: "ticks".to_string(),
                }
                .into());
            }

            if ticks > Self::TICKS_MAXIMUM {
                return Err(GameErrorKind::InvalidArgument {
                    reason: InvalidArgumentKind::TooLarge,
                    parameter: "ticks".to_string(),
                }
                .into());
            }

            let load = RangeTolerance::clamped_range(load, Self::LOAD_MINIMUM, Self::LOAD_MAXIMUM)
                .map_err(|reason| GameErrorKind::InvalidArgument {
                    reason,
                    parameter: "load".to_string(),
                })?;

            let damage =
                RangeTolerance::clamped_range(damage, Self::DAMAGE_MINIMUM, Self::DAMAGE_MAXIMUM)
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
}

impl AsRef<SubsystemBase> for DynamicShotLauncherSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
