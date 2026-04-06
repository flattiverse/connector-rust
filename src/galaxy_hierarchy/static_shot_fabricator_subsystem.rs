use crate::galaxy_hierarchy::{
    Controllable, Cost, DynamicShotFabricatorSubsystem, RangeTolerance, SubsystemBase, SubsystemExt,
};
use crate::{FlattiverseEvent, GameError, GameErrorKind, SubsystemSlot, SubsystemStatus};
use std::sync::Weak;

#[derive(Debug)]
pub struct StaticShotFabricatorSubsystem {
    base: DynamicShotFabricatorSubsystem,
}

impl StaticShotFabricatorSubsystem {
    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: DynamicShotFabricatorSubsystem::new(controllable, name, exists, slot),
        }
    }

    /// The minimum configurable shot fabrication rate.
    #[inline]
    pub fn minimum_rate(&self) -> f32 {
        self.base.minimum_rate()
    }

    /// The maximum configurable shot fabrication rate.
    #[inline]
    pub fn maximum_rate(&self) -> f32 {
        self.base.maximum_rate()
    }

    /// True when the fabricator was active during the latest reported server tick.
    #[inline]
    pub fn active(&self) -> bool {
        self.base.active()
    }

    /// The configured shot fabrication rate.
    #[inline]
    pub fn rate(&self) -> f32 {
        self.base.rate()
    }

    /// The energy consumed by the fabricator during the current server tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.base.consumed_energy_this_tick()
    }

    /// The ions consumed by the fabricator during the current server tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.base.consumed_ions_this_tick()
    }

    /// The neutrinos consumed by the fabricator during the current server tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.base.consumed_neutrinos_this_tick()
    }

    /// Calculates the current placeholder resource costs for one fabrication tick at the specified
    /// rate. The current model consumes only energy.
    #[inline]
    pub fn calculate_cost(&self, rate: f32) -> Option<Cost> {
        self.base.calculate_cost(rate)
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
                RangeTolerance::clamped_range(rate, self.minimum_rate(), self.maximum_rate())
                    .map_err(|reason| GameErrorKind::InvalidArgument {
                        reason,
                        parameter: "rate".to_string(),
                    })?;

            controllable
                .cluster()
                .galaxy()
                .connection()
                .static_shot_fabricator_subsystem_set(controllable.id(), self.slot(), rate)
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
                .static_shot_fabricator_subsystem_on(controllable.id(), self.slot())
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
                .static_shot_fabricator_subsystem_off(controllable.id(), self.slot())
                .await
        }
    }

    #[inline]
    pub(crate) fn set_maximum_rate(&self, maximum_rate: f32) {
        self.base.set_maximum_rate(maximum_rate);
    }

    #[inline]
    pub(crate) fn reset_runtime(&self) {
        self.base.reset_runtime()
    }

    #[inline]
    pub(crate) fn update_runtime(
        &self,
        active: bool,
        rate: f32,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.base.update_runtime(
            active,
            rate,
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

impl AsRef<SubsystemBase> for StaticShotFabricatorSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        self.base.as_ref()
    }
}
