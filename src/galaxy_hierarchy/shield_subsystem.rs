use crate::galaxy_hierarchy::{
    Controllable, Cost, RangeTolerance, ShipBalancing, SubsystemBase, SubsystemExt,
};
use crate::utils::Atomic;
use crate::{
    FlattiverseEvent, FlattiverseEventKind, GameError, GameErrorKind, SubsystemSlot,
    SubsystemStatus,
};
use std::sync::Weak;

/// Shield subsystem of a controllable.
#[derive(Debug)]
pub struct ShieldSubsystem {
    base: SubsystemBase,
    maximum: Atomic<f32>,
    minimum_rate: Atomic<f32>,
    maximum_rate: Atomic<f32>,
    current: Atomic<f32>,
    active: Atomic<bool>,
    rate: Atomic<f32>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl ShieldSubsystem {
    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: SubsystemBase::new(controllable, name, exists, slot),
            maximum: Atomic::from(if exists { 20.0 } else { 0.0 }),
            minimum_rate: Atomic::from(0.0),
            maximum_rate: Atomic::from(0.125),
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
        self.maximum.load()
    }

    /// The current shield integrity.
    #[inline]
    pub fn current(&self) -> f32 {
        self.current.load()
    }

    /// The minimum configurable shield load rate.
    #[inline]
    pub fn minimum_rate(&self) -> f32 {
        self.minimum_rate.load()
    }

    /// The maximum configurable shield load rate.
    #[inline]
    pub fn maximum_rate(&self) -> f32 {
        self.maximum_rate.load()
    }

    pub(crate) fn set_maximum(&self, maximum: f32) {
        let maximum = if self.exists() {
            self.maximum.store(maximum);
            maximum
        } else {
            self.maximum.store(0.0);
            0.0
        };

        // TODO self.refresh_tier();

        if self.current() > maximum {
            self.current.store(maximum);
        }
    }

    pub(crate) fn set_rate_capabilities(&self, minimum_rate: f32, maximum_rate: f32) {
        if !self.exists() {
            self.minimum_rate.store(minimum_rate);
            self.maximum_rate.store(maximum_rate);
        } else {
            self.minimum_rate.store(0.0);
            self.maximum_rate.store(0.0);
        }

        // TODO self.refresh_tier();
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
    pub fn calculate_cost(&self, rate: f32) -> Option<Cost> {
        if !self.exists() {
            None
        } else {
            let maximum = self.maximum();
            let maximum_rate = self.maximum_rate();
            let rate =
                RangeTolerance::clamped_range(rate, self.minimum_rate(), maximum_rate).ok()?;

            if Self::rate_to_tier(maximum, maximum_rate) == 5 {
                Cost::default().with_ions(if maximum_rate > 0.0 {
                    0.9 * rate / maximum_rate
                } else {
                    0.0
                })
            } else {
                Cost::default().with_energy(ShipBalancing::calculate_shield_energy(
                    Self::rate_to_tier(maximum, maximum_rate),
                    rate,
                    maximum_rate,
                    Self::full_cost_from_capabilities(maximum, maximum_rate),
                ))
            }
            .into_values_checked()
        }
    }

    pub(crate) const fn rate_to_tier(maximum: f32, maximum_rate: f32) -> u8 {
        if maximum <= 20.5 && maximum_rate <= 0.101 {
            1
        } else if maximum <= 35.5 && maximum_rate <= 0.141 {
            2
        } else if maximum <= 50.5 && maximum_rate <= 0.181 {
            3
        } else if maximum <= 65.5 && maximum_rate <= 0.231 {
            4
        } else {
            5
        }
    }

    pub(crate) const fn full_cost_from_capabilities(maximum: f32, maximum_rate: f32) -> f32 {
        match Self::rate_to_tier(maximum, maximum_rate) {
            1 => 16.0,
            2 => 26.0,
            3 => 39.0,
            4 => 58.0,
            5 => 82.0,
            _ => 0.0,
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
            let rate =
                RangeTolerance::clamped_range(rate, self.minimum_rate(), self.maximum_rate())
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

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn reset_runtime(&self) {
        self.current.store(0.0);
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

    // TODO pub(crate) fn refresh_tier(&self) {}
}

impl AsRef<SubsystemBase> for ShieldSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
