use crate::galaxy_hierarchy::{
    Controllable, Cost, RangeTolerance, ShipBalancing, SubsystemBase, SubsystemExt,
};
use crate::utils::Atomic;
use crate::{
    FlattiverseEvent, FlattiverseEventKind, GameError, GameErrorKind, SubsystemSlot,
    SubsystemStatus,
};
use std::sync::Weak;

/// Nebula collector subsystem of a controllable.
#[derive(Debug)]
pub struct NebulaCollectorSubsystem {
    base: SubsystemBase,
    minimum_rate: Atomic<f32>,
    maximum_rate: Atomic<f32>,
    rate: Atomic<f32>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
    collected_this_tick: Atomic<f32>,
    collected_hue_this_tick: Atomic<f32>,
}

impl NebulaCollectorSubsystem {
    pub(crate) fn new(controllable: Weak<Controllable>, exists: bool, slot: SubsystemSlot) -> Self {
        Self {
            base: SubsystemBase::new(controllable, "NebulaCollector".to_string(), exists, slot),
            minimum_rate: Atomic::from(0.0),
            maximum_rate: Atomic::from(0.1),
            rate: Atomic::from(0.0),
            consumed_energy_this_tick: Atomic::from(0.0),
            consumed_ions_this_tick: Atomic::from(0.0),
            consumed_neutrinos_this_tick: Atomic::from(0.0),
            collected_this_tick: Atomic::from(0.0),
            collected_hue_this_tick: Atomic::from(0.0),
        }
    }

    #[inline]
    pub(crate) fn create_classic_ship_nebula_collector(controllable: Weak<Controllable>) -> Self {
        Self::new(controllable, true, SubsystemSlot::NebulaCollector)
    }

    /// Minimum configurable collection rate.
    /// `0` means the collector is off.
    #[inline]
    pub fn minimum_rate(&self) -> f32 {
        self.minimum_rate.load()
    }

    /// Maximum configurable collection rate supported by the current controllable kind.
    #[inline]
    pub fn maximum_rate(&self) -> f32 {
        self.maximum_rate.load()
    }

    #[instrument(level = "debug", skip(self))]
    pub(crate) fn set_capabilities(&self, minimum_rate: f32, maximum_rate: f32) {
        if self.exists() {
            self.minimum_rate.store(minimum_rate);
            self.maximum_rate.store(maximum_rate);
        } else {
            self.minimum_rate.store(0.0);
            self.maximum_rate.store(0.0);
        }
        // TODO self.refresh_tier();
    }

    /// Rate currently mirrored from the server.
    /// The server may clear this value back to `0`, for example after movement or a paid zero-yield
    /// tick.
    #[inline]
    pub fn rate(&self) -> f32 {
        self.rate.load()
    }

    /// Energy consumed by the collector during the current server tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// Ions consumed by the collector during the current server tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// Neutrinos consumed by the collector during the current server tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    /// Nebula amount collected during the current server tick.
    #[inline]
    pub fn collected_this_tick(&self) -> f32 {
        self.collected_this_tick.load()
    }

    /// Hue of the nebula material collected during the current server tick.
    #[inline]
    pub fn collected_hue_this_tick(&self) -> f32 {
        self.collected_hue_this_tick.load()
    }

    /// Calculates the current placeholder tick cost for the given collection rate.
    pub fn calculate_cost(&self, rate: f32) -> Option<Cost> {
        if !self.exists() {
            None
        } else {
            let maximum_rate = self.maximum_rate();
            let rate =
                RangeTolerance::clamped_range(rate, self.minimum_rate(), maximum_rate).ok()?;

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
        if maximum_rate <= 0.0166 {
            6.0
        } else if maximum_rate <= 0.0243 {
            9.0
        } else if maximum_rate <= 0.0342 {
            14.0
        } else if maximum_rate <= 0.0463 {
            22.0
        } else {
            34.0
        }
    }

    /// Sets the target nebula-collection rate on the server.
    ///
    /// The current classic ship uses `rate in [0; 0.1]` with placeholder tick cost
    /// `energy = 1600 * rate^2`. The server executes the collector authoritatively: it requires low
    /// movement, searches for an in-range nebula, and may clear the mirrored rate back to `0` after
    /// a paid zero-yield tick.
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
                .nebula_collector_set(controllable.id(), rate)
                .await
        }
    }

    /// Convenience wrapper for [`Self::set`]`(0.0)`.
    #[inline]
    pub async fn off(&self) -> Result<(), GameError> {
        self.set(0.0).await
    }

    #[instrument(level = "debug", skip(self))]
    pub(crate) fn reset_runtime(&self) {
        self.rate.store_default();
        self.consumed_energy_this_tick.store_default();
        self.consumed_ions_this_tick.store_default();
        self.consumed_neutrinos_this_tick.store_default();
        self.collected_this_tick.store_default();
        self.collected_hue_this_tick.store_default();
        self.base.reset_runtime_status();
    }

    #[instrument(level = "debug", skip(self))]
    pub(crate) fn update_runtime(
        &self,
        rate: f32,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
        collected_this_tick: f32,
        collected_hue_this_tick: f32,
    ) {
        self.rate.store(rate);
        self.consumed_energy_this_tick
            .store(consumed_energy_this_tick);
        self.consumed_ions_this_tick.store(consumed_ions_this_tick);
        self.consumed_neutrinos_this_tick
            .store(consumed_neutrinos_this_tick);
        self.collected_this_tick.store(collected_this_tick);
        self.collected_hue_this_tick.store(collected_hue_this_tick);
        self.base.update_runtime_status(status);
    }

    pub(crate) fn create_runtime_event(&self) -> Option<FlattiverseEvent> {
        if !self.exists() || !self.base.should_emit_runtime_event() {
            None
        } else {
            Some(
                FlattiverseEventKind::NebulaCollectorSubsystem {
                    controllable: self.controllable(),
                    slot: self.slot(),
                    status: self.status(),
                    rate: self.rate(),
                    consumed_energy_this_tick: self.consumed_energy_this_tick(),
                    consumed_ions_this_tick: self.consumed_ions_this_tick(),
                    consumed_neutrinos_this_tick: self.consumed_neutrinos_this_tick(),
                    collected_this_tick: self.collected_this_tick(),
                    collected_hue_this_tick: self.collected_hue_this_tick(),
                }
                .into(),
            )
        }
    }

    // TODO pub fn refresh_tier(&self) {}
}

impl AsRef<SubsystemBase> for NebulaCollectorSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
