use crate::galaxy_hierarchy::{
    Controllable, Cost, RangeTolerance, ShipBalancing, SubsystemBase, SubsystemExt,
};
use crate::utils::Atomic;
use crate::{
    FlattiverseEvent, FlattiverseEventKind, GameError, GameErrorKind, SubsystemSlot,
    SubsystemStatus,
};
use std::sync::Weak;

#[derive(Debug)]
pub struct ResourceMinerSubsystem {
    base: SubsystemBase,
    minimum_rate: Atomic<f32>,
    maximum_rate: Atomic<f32>,
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
    pub(crate) fn new(controllable: Weak<Controllable>, exists: bool, slot: SubsystemSlot) -> Self {
        Self {
            base: SubsystemBase::new(controllable, "ResourceMiner".to_string(), exists, slot),
            minimum_rate: Atomic::from(0.0),
            maximum_rate: Atomic::from(0.1),
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
    /// `0` means the miner is off.
    #[inline]
    pub fn minimum_rate(&self) -> f32 {
        self.minimum_rate.load()
    }

    /// Maximum configurable mining rate.
    #[inline]
    pub fn maximum_rate(&self) -> f32 {
        self.maximum_rate.load()
    }

    #[instrument(level = "trace", skip(self))]
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
            let maximum_rate = self.maximum_rate();
            let rate =
                RangeTolerance::clamped_range(rate, self.minimum_rate(), maximum_rate).ok()?;

            Cost::default()
                .with_energy(ShipBalancing::calculate_engine_energy(
                    rate,
                    maximum_rate,
                    Self::full_cost_from_max_rate(maximum_rate),
                ))
                .into_values_checked()
        }
    }

    pub(crate) const fn full_cost_from_max_rate(maximum_rate: f32) -> f32 {
        if maximum_rate <= 0.00221 {
            10.0
        } else if maximum_rate <= 0.00331 {
            14.0
        } else if maximum_rate <= 0.00461 {
            20.0
        } else if maximum_rate <= 0.00611 {
            30.0
        } else {
            44.0
        }
    }

    /// Sets the mining rate on the server.
    ///
    /// # Remarks
    /// The current classic ship uses `rate in [0; 0.01]` with placeholder tick cost
    /// `energy = 160000 * rate^2`. The server executes mining authoritatively: it requires low
    /// movement, mines in-range body resources, and may clear the mirrored rate back to `0` after
    /// movement or a paid zero-yield tick.
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
                .resource_miner_subsystem_set(controllable.id(), rate)
                .await
        }
    }

    /// Turns the resource miner off by setting the rate to zero.
    #[inline]
    pub async fn off(&self) -> Result<(), GameError> {
        self.set(0.0).await
    }

    #[instrument(level = "trace", skip(self))]
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

    #[instrument(level = "trace", skip(self))]
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

    // TODO pub(crate) fn refresh_tier(&self) {}
}

impl AsRef<SubsystemBase> for ResourceMinerSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
