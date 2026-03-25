use crate::galaxy_hierarchy::{Controllable, RangeTolerance, SubsystemBase, SubsystemExt};
use crate::utils::Atomic;
use crate::{
    FlattiverseEvent, FlattiverseEventKind, GameError, GameErrorKind, SubsystemSlot,
    SubsystemStatus,
};
use std::sync::Weak;

#[derive(Debug)]
pub struct ShieldSubsystem {
    base: SubsystemBase,
    current: Atomic<f32>,
    active: Atomic<bool>,
    rate: Atomic<f32>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl ShieldSubsystem {
    const MAXIMUM_VALUE: f32 = 20.0;
    const MINIMUM_RATE_VALUE: f32 = 0.0;
    const MAXIMUM_RATE_VALUE: f32 = 0.125;
    const ENERGY_SCALE: f32 = 1600.0;

    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: SubsystemBase::new(controllable, name, exists, slot),
            current: Default::default(),
            active: Default::default(),
            rate: Default::default(),
            consumed_energy_this_tick: Default::default(),
            consumed_ions_this_tick: Default::default(),
            consumed_neutrinos_this_tick: Default::default(),
        }
    }

    #[inline]
    pub(crate) fn create_classic_ship_shield(controllable: Weak<Controllable>) -> Self {
        Self::new(
            controllable,
            "Shield".to_string(),
            true,
            SubsystemSlot::Shield,
        )
    }

    /// The maximum shield integrity.
    #[inline]
    pub fn maximum(&self) -> f32 {
        if self.exists() {
            Self::MAXIMUM_VALUE
        } else {
            0.0
        }
    }

    /// The current shield integrity.
    #[inline]
    pub fn current(&self) -> f32 {
        self.current.load()
    }

    /// The minimum configurable shield load rate.
    #[inline]
    pub fn minimum_rate(&self) -> f32 {
        Self::MINIMUM_RATE_VALUE
    }

    /// The maximum configurable shield load rate.
    #[inline]
    pub fn maximum_rate(&self) -> f32 {
        Self::MAXIMUM_RATE_VALUE
    }

    /// Whether shield loading is active.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    /// The configured shield load rate per tick.
    #[inline]
    pub fn rate(&self) -> f32 {
        self.rate.load()
    }

    /// The energy consumed during the current server tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// The ions consumed during the current server tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// The neutrinos consumed during the current server tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    /// Calculates the resource costs for one shield loading tick at the specified rate.
    pub fn calculate_cost(&self, rate: f32) -> Option<ShieldCost> {
        let mut cost = ShieldCost::default();

        if !self.exists() {
            return None;
        }

        let rate =
            RangeTolerance::clamped_range(rate, Self::MINIMUM_RATE_VALUE, Self::MAXIMUM_RATE_VALUE)
                .ok()?;

        cost.energy = rate * rate * Self::ENERGY_SCALE;

        if cost.energy.is_nan() || cost.energy.is_infinite() {
            None
        } else {
            Some(cost)
        }
    }

    /// Sets the shield load rate on the server.
    pub async fn set(&self, rate: f32) -> Result<(), GameError> {
        let controllable = self.controllable();

        if !controllable.active() || !self.exists() {
            Err(GameErrorKind::SpecifiedElementNotFound.into())
        } else if !controllable.alive() {
            Err(GameErrorKind::YouNeedToContinueFirst.into())
        } else {
            let rate = RangeTolerance::clamped_range(
                rate,
                Self::MINIMUM_RATE_VALUE,
                Self::MAXIMUM_RATE_VALUE,
            )
            .map_err(|reason| GameErrorKind::InvalidArgument {
                reason,
                parameter: "rate".to_string(),
            })?;

            controllable
                .cluster()
                .galaxy()
                .connection()
                .shield_subsystem_set(controllable.id(), rate)
                .await
        }
    }

    /// Turns shield loading on.
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
                .shield_subsystem_on(controllable.id())
                .await
        }
    }

    /// Turns shield loading off.
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
                .shield_subsystem_off(controllable.id())
                .await
        }
    }

    pub(crate) fn reset_runtime(&self) {
        self.current.store(0.0);
        self.active.store(false);
        self.rate.store(0.0);
        self.consumed_energy_this_tick.store(0.0);
        self.consumed_ions_this_tick.store(0.0);
        self.consumed_neutrinos_this_tick.store(0.0);
        self.base.reset_runtime_status();
    }

    pub(crate) fn update_runtime(
        &self,
        current: f32,
        active: bool,
        rate: f32,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.current.store(current);
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
                FlattiverseEventKind::ShieldSubsystem {
                    controllable: self.controllable(),
                    slot: self.slot(),
                    status: self.status(),
                    current: self.current(),
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
}

impl AsRef<SubsystemBase> for ShieldSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ShieldCost {
    pub energy: f32,
    pub ions: f32,
    pub neutrinos: f32,
}
