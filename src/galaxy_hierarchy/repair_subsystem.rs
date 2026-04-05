use crate::galaxy_hierarchy::{Controllable, Cost, RangeTolerance, SubsystemBase, SubsystemExt};
use crate::utils::Atomic;
use crate::{
    FlattiverseEvent, FlattiverseEventKind, GameError, GameErrorKind, SubsystemSlot,
    SubsystemStatus,
};
use std::sync::Weak;

/// Hull repair subsystem of a controllable.
#[derive(Debug)]
pub struct RepairSubsystem {
    base: SubsystemBase,
    rate: Atomic<f32>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
    repaired_hull_this_tick: Atomic<f32>,
}

impl RepairSubsystem {
    const MINIMUM_RATE_VALUE: f32 = 0.0;
    const MAXIMUM_RATE_VALUE: f32 = 0.1;
    const ENERGY_SCALE: f32 = 1_600.0;

    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: SubsystemBase::new(controllable, name, exists, slot),
            rate: Default::default(),
            consumed_energy_this_tick: Default::default(),
            consumed_ions_this_tick: Default::default(),
            consumed_neutrinos_this_tick: Default::default(),
            repaired_hull_this_tick: Default::default(),
        }
    }

    pub(crate) fn create_classic_ship_repair(controllable: Weak<Controllable>) -> Self {
        Self::new(
            controllable,
            "Repair".to_string(),
            true,
            SubsystemSlot::Repair,
        )
    }

    /// The minimum configurable repair rate.
    #[inline]
    pub fn minimum_rate(&self) -> f32 {
        Self::MINIMUM_RATE_VALUE
    }

    /// The maximum configurable repair rate.
    #[inline]
    pub fn maximum_rate(&self) -> f32 {
        Self::MAXIMUM_RATE_VALUE
    }

    /// The configured hull repair rate per tick.
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

    /// The amount of hull repaired during the current server tick.
    #[inline]
    pub fn repaired_hull_this_tick(&self) -> f32 {
        self.repaired_hull_this_tick.load()
    }

    /// Calculates the resource costs for one repair tick at the specified rate.
    pub fn calculate_cost(&self, rate: f32) -> Option<Cost> {
        if !self.exists() {
            None
        } else {
            let rate = RangeTolerance::clamped_range(
                rate,
                Self::MINIMUM_RATE_VALUE,
                Self::MAXIMUM_RATE_VALUE,
            )
            .ok()?;

            Cost::default()
                .with_energy(rate * rate * Self::ENERGY_SCALE)
                .into_values_checked()
        }
    }

    /// Sets the repair rate on the server.
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
                .repair_subsystem_set(controllable.id(), rate)
                .await
        }
    }

    pub(crate) fn reset_runtime(&self) {
        self.rate.store_default();
        self.consumed_energy_this_tick.store_default();
        self.consumed_ions_this_tick.store_default();
        self.consumed_neutrinos_this_tick.store_default();
        self.repaired_hull_this_tick.store_default();
        self.base.reset_runtime_status();
    }

    pub(crate) fn update_runtime(
        &self,
        rate: f32,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
        repaired_hull_this_tick: f32,
    ) {
        self.rate.store(rate);
        self.consumed_energy_this_tick
            .store(consumed_energy_this_tick);
        self.consumed_ions_this_tick.store(consumed_ions_this_tick);
        self.consumed_neutrinos_this_tick
            .store(consumed_neutrinos_this_tick);
        self.repaired_hull_this_tick.store(repaired_hull_this_tick);
        self.base.update_runtime_status(status);
    }

    pub(crate) fn create_runtime_event(&self) -> Option<FlattiverseEvent> {
        if !self.exists() || !self.base.should_emit_runtime_event() {
            None
        } else {
            Some(
                FlattiverseEventKind::RepairSubsystem {
                    controllable: self.controllable(),
                    slot: self.slot(),
                    status: self.status(),
                    rate: self.rate(),
                    consumed_energy_this_tick: self.consumed_energy_this_tick(),
                    consumed_ions_this_tick: self.consumed_ions_this_tick(),
                    consumed_neutrinos_this_tick: self.consumed_neutrinos_this_tick(),
                    repaired_hull_this_tick: self.repaired_hull_this_tick(),
                }
                .into(),
            )
        }
    }
}

impl AsRef<SubsystemBase> for RepairSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
