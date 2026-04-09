use crate::galaxy_hierarchy::cost::Cost;
use crate::galaxy_hierarchy::{
    Controllable, RangeTolerance, ShipBalancing, SubsystemBase, SubsystemExt,
};
use crate::utils::Atomic;
use crate::{
    FlattiverseEvent, FlattiverseEventKind, GameError, GameErrorKind, SubsystemSlot,
    SubsystemStatus,
};
use std::sync::Weak;

/// Persistent shot-fabricator subsystem configuration and runtime state of a controllable.
#[derive(Debug)]
pub struct DynamicShotFabricatorSubsystem {
    base: SubsystemBase,
    maximum_rate: Atomic<f32>,
    active: Atomic<bool>,
    rate: Atomic<f32>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl DynamicShotFabricatorSubsystem {
    const MINIMUM_RATE_VALUE: f32 = 0.0;

    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: SubsystemBase::new(controllable, name, exists, slot),
            maximum_rate: Atomic::from(if exists { 0.025 } else { 0.0 }),
            active: Atomic::from(false),
            rate: Atomic::from(0.0),
            consumed_energy_this_tick: Atomic::from(0.0),
            consumed_ions_this_tick: Atomic::from(0.0),
            consumed_neutrinos_this_tick: Atomic::from(0.0),
        }
    }

    /// The minimum configurable shot fabrication rate.
    #[inline]
    pub fn minimum_rate(&self) -> f32 {
        Self::MINIMUM_RATE_VALUE
    }

    /// The maximum configurable shot fabrication rate.
    #[inline]
    pub fn maximum_rate(&self) -> f32 {
        self.maximum_rate.load()
    }

    /// True when the fabricator was active during the latest reported server tick.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    /// The configured shot fabrication rate.
    #[inline]
    pub fn rate(&self) -> f32 {
        self.rate.load()
    }

    /// The energy consumed by the fabricator during the current server tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// The ions consumed by the fabricator during the current server tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// The neutrinos consumed by the fabricator during the current server tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    /// Calculates the current placeholder resource costs for one fabrication tick at the specified
    /// rate. The current model consumes only energy.
    pub fn calculate_cost(&self, rate: f32) -> Option<Cost> {
        if !self.exists() {
            None
        } else {
            let maximum_rate = self.maximum_rate();
            let rate =
                RangeTolerance::clamped_range(rate, Self::MINIMUM_RATE_VALUE, maximum_rate).ok()?;

            Cost::default()
                .with_energy(ShipBalancing::calculate_engine_energy(
                    rate,
                    maximum_rate,
                    Self::full_cost_from_maximum_rate(maximum_rate),
                ))
                .into_values_checked()
        }
    }

    pub(crate) const fn full_cost_from_maximum_rate(maximum_rate: f32) -> f32 {
        if maximum_rate <= 0.00331 {
            2.64
        } else if maximum_rate <= 0.00496 {
            3.76
        } else if maximum_rate <= 0.00688 {
            5.50
        } else if maximum_rate <= 0.00908 {
            8.62
        } else if maximum_rate <= 0.01156 {
            13.86
        } else if maximum_rate <= 0.0121 {
            8.0
        } else if maximum_rate <= 0.0181 {
            11.0
        } else if maximum_rate <= 0.0251 {
            16.0
        } else if maximum_rate <= 0.0331 {
            25.0
        } else {
            39.0
        }
    }

    /// Sets the shot fabrication rate on the server.
    pub async fn set(&self, rate: f32) -> Result<(), GameError> {
        let controllable = self.controllable();

        if !controllable.active() || !self.exists() {
            Err(GameErrorKind::SpecifiedElementNotFound.into())
        } else if !controllable.alive() {
            Err(GameErrorKind::YouNeedToContinueFirst.into())
        } else {
            let rate =
                RangeTolerance::clamped_range(rate, Self::MINIMUM_RATE_VALUE, self.maximum_rate())
                    .map_err(|reason| GameErrorKind::InvalidArgument {
                        reason,
                        parameter: "rate".to_string(),
                    })?;

            controllable
                .cluster()
                .galaxy()
                .connection()
                .dynamic_shot_fabricator_subsystem_set(controllable.id(), rate)
                .await
        }
    }

    /// Turns the shot fabricator on.
    pub async fn on(&self) -> Result<(), GameError> {
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
                .dynamic_shot_fabricator_subsystem_on(controllable.id())
                .await
        }
    }

    /// Turns the shot fabricator off.
    pub async fn off(&self) -> Result<(), GameError> {
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
                .dynamic_shot_fabricator_subsystem_off(controllable.id())
                .await
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn set_maximum_rate(&self, maximum_rate: f32) {
        let maximum_rate = if self.exists() {
            self.maximum_rate.store(maximum_rate);
            maximum_rate
        } else {
            0.0
        };

        if self.rate() > maximum_rate {
            self.rate.store(maximum_rate);
        }

        // TODO self.refresh_tier();
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn reset_runtime(&self) {
        self.active.store(false);
        self.rate.store(0.0);
        self.consumed_energy_this_tick.store(0.0);
        self.consumed_ions_this_tick.store(0.0);
        self.consumed_neutrinos_this_tick.store(0.0);
        self.base.reset_runtime_status();
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn update_runtime(
        &self,
        active: bool,
        rate: f32,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.active.store(active);
        self.rate.store(rate);
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
                FlattiverseEventKind::DynamicShotFabricatorSubsystem {
                    controllable: self.controllable(),
                    slot: self.slot(),
                    status: self.status(),
                    active: self.active(),
                    rate: self.rate(),
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

impl AsRef<SubsystemBase> for DynamicShotFabricatorSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
