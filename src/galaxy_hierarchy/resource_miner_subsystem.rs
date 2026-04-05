use crate::galaxy_hierarchy::{Controllable, Cost, RangeTolerance, SubsystemBase, SubsystemExt};
use crate::utils::Atomic;
use crate::{
    FlattiverseEvent, FlattiverseEventKind, GameError, GameErrorKind, SubsystemSlot,
    SubsystemStatus,
};
use std::sync::Weak;

#[derive(Debug)]
pub struct ResourceMinerSubsystem {
    base: SubsystemBase,
    rate: Atomic<f32>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
    mined_metal_this_tick: Atomic<f32>,
    mined_carbon_this_tick: Atomic<f32>,
    mined_hydrogen_this_tick: Atomic<f32>,
    mined_silicon_this_tick: Atomic<f32>,
}

impl ResourceMinerSubsystem {
    const MINIMUM_RATE_VALUE: f32 = 0.0;
    const MAXIMUM_RATE_VALUE: f32 = 0.01;
    const ENERGY_SCALE: f32 = 160_000.0;

    pub(crate) fn new(controllable: Weak<Controllable>, exists: bool, slot: SubsystemSlot) -> Self {
        Self {
            base: SubsystemBase::new(controllable, "ResourceMiner".to_string(), exists, slot),
            rate: Default::default(),
            consumed_energy_this_tick: Default::default(),
            consumed_ions_this_tick: Default::default(),
            consumed_neutrinos_this_tick: Default::default(),
            mined_metal_this_tick: Default::default(),
            mined_carbon_this_tick: Default::default(),
            mined_hydrogen_this_tick: Default::default(),
            mined_silicon_this_tick: Default::default(),
        }
    }

    pub(crate) fn create_classic_ship_resource_miner(controllable: Weak<Controllable>) -> Self {
        Self::new(controllable, true, SubsystemSlot::ResourceMiner)
    }

    /// Minimum configurable mining rate.
    #[inline]
    pub fn minimum_rate(&self) -> f32 {
        Self::MINIMUM_RATE_VALUE
    }

    /// Maximum configurable mining rate.
    #[inline]
    pub fn maximum_rate(&self) -> f32 {
        Self::MAXIMUM_RATE_VALUE
    }

    /// Configured mining rate for the tick.
    #[inline]
    pub fn rate(&self) -> f32 {
        self.rate.load()
    }

    /// Energy consumed during the current server tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// Ions consumed during the current server tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// Neutrinos consumed during the current server tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    /// Metal mined during the current server tick.
    #[inline]
    pub fn mined_metal_this_tick(&self) -> f32 {
        self.mined_metal_this_tick.load()
    }

    /// Carbon mined during the current server tick.
    #[inline]
    pub fn mined_carbon_this_tick(&self) -> f32 {
        self.mined_carbon_this_tick.load()
    }

    /// Hydrogen mined during the current server tick.
    #[inline]
    pub fn mined_hydrogen_this_tick(&self) -> f32 {
        self.mined_hydrogen_this_tick.load()
    }

    /// Silicon mined during the current server tick.
    #[inline]
    pub fn mined_silicon_this_tick(&self) -> f32 {
        self.mined_silicon_this_tick.load()
    }

    /// Calculates the resource costs for one mining tick at the specified rate.
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

    /// Sets the mining rate on the server.
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
                .resource_miner_subsystem_set(controllable.id(), rate)
                .await
        }
    }

    /// Turns the resource miner off by setting the rate to zero.
    #[inline]
    pub async fn off(&self) -> Result<(), GameError> {
        self.set(0.0).await
    }

    pub(crate) fn reset_runtime(&self) {
        self.rate.store_default();
        self.consumed_energy_this_tick.store_default();
        self.consumed_ions_this_tick.store_default();
        self.consumed_neutrinos_this_tick.store_default();
        self.mined_metal_this_tick.store_default();
        self.mined_carbon_this_tick.store_default();
        self.mined_hydrogen_this_tick.store_default();
        self.mined_silicon_this_tick.store_default();
        self.base.reset_runtime_status();
    }

    pub(crate) fn update_runtime(
        &self,
        rate: f32,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
        mined_metal_this_tick: f32,
        mined_carbon_this_tick: f32,
        mined_hydrogen_this_tick: f32,
        mined_silicon_this_tick: f32,
    ) {
        self.rate.store(rate);
        self.consumed_energy_this_tick
            .store(consumed_energy_this_tick);
        self.consumed_ions_this_tick.store(consumed_ions_this_tick);
        self.consumed_neutrinos_this_tick
            .store(consumed_neutrinos_this_tick);
        self.mined_metal_this_tick.store(mined_metal_this_tick);
        self.mined_carbon_this_tick.store(mined_carbon_this_tick);
        self.mined_hydrogen_this_tick
            .store(mined_hydrogen_this_tick);
        self.mined_silicon_this_tick.store(mined_silicon_this_tick);
        self.base.update_runtime_status(status);
    }

    pub(crate) fn create_runtime_event(&self) -> Option<FlattiverseEvent> {
        if !self.exists() || !self.base.should_emit_runtime_event() {
            None
        } else {
            Some(
                FlattiverseEventKind::ResourceMinerSubsystem {
                    controllable: self.controllable(),
                    slot: self.slot(),
                    status: self.status(),
                    rate: self.rate(),
                    consumed_energy_this_tick: self.consumed_energy_this_tick(),
                    consumed_ions_this_tick: self.consumed_ions_this_tick(),
                    consumed_neutrinos_this_tick: self.consumed_neutrinos_this_tick(),
                    mined_metal_this_tick: self.mined_metal_this_tick(),
                    mined_carbon_this_tick: self.mined_carbon_this_tick(),
                    mined_hydrogen_this_tick: self.mined_hydrogen_this_tick(),
                    mined_silicon_this_tick: self.mined_silicon_this_tick(),
                }
                .into(),
            )
        }
    }
}

impl AsRef<SubsystemBase> for ResourceMinerSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
