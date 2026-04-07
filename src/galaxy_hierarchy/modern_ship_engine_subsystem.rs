use crate::galaxy_hierarchy::{
    Controllable, Cost, RangeTolerance, ShipBalancing, SubsystemBase, SubsystemExt,
};
use crate::utils::Atomic;
use crate::{
    FlattiverseEvent, FlattiverseEventKind, GameError, GameErrorKind, SubsystemSlot,
    SubsystemStatus,
};
use std::sync::Weak;

/// Engine subsystem of one modern-ship thruster slot.
#[derive(Debug)]
pub struct ModernShipEngineSubsystem {
    base: SubsystemBase,
    maximum_thrust: Atomic<f32>,
    maximum_thrust_change_per_tick: Atomic<f32>,
    current_thrust: Atomic<f32>,
    target_thrust: Atomic<f32>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl ModernShipEngineSubsystem {
    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: SubsystemBase::new(controllable, name, exists, slot),
            maximum_thrust: Atomic::from(0.0),
            maximum_thrust_change_per_tick: Atomic::from(0.0),
            current_thrust: Atomic::from(0.0),
            target_thrust: Atomic::from(0.0),
            consumed_energy_this_tick: Atomic::from(0.0),
            consumed_ions_this_tick: Atomic::from(0.0),
            consumed_neutrinos_this_tick: Atomic::from(0.0),
        }
    }

    #[inline]
    pub fn maximum_thrust(&self) -> f32 {
        self.maximum_thrust.load()
    }

    // TODO fn tier_infos(&self) -> Arc<Vec<SubsystemTierInfo>> { todo!() }

    #[inline]
    pub fn maximum_forward_thrust(&self) -> f32 {
        self.maximum_thrust()
    }

    #[inline]
    pub fn maximum_reverse_thrust(&self) -> f32 {
        self.maximum_thrust()
    }

    #[inline]
    pub fn maximum_thrust_change_per_tick(&self) -> f32 {
        self.maximum_thrust_change_per_tick.load()
    }

    #[inline]
    pub fn current_thrust(&self) -> f32 {
        self.current_thrust.load()
    }

    #[inline]
    pub fn target_thrust(&self) -> f32 {
        self.target_thrust.load()
    }

    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    pub fn calculate_cost(&self, thrust: f32) -> Option<Cost> {
        if !self.exists() {
            None
        } else {
            let maximum_thrust = self.maximum_thrust();
            let thrust = RangeTolerance::clamped_range(
                thrust,
                -self.maximum_thrust(),
                self.maximum_thrust(),
            )
            .ok()?;

            let absolut_thrust = thrust.abs();

            Cost::default()
                .with_energy(ShipBalancing::calculate_engine_energy(
                    absolut_thrust,
                    maximum_thrust,
                    Self::full_cost_maximum_thrust(maximum_thrust),
                ))
                .into_values_checked()
        }
    }

    pub async fn set_thrust(&self, thrust: f32) -> Result<(), GameError> {
        let controllable = self.controllable();

        if !controllable.active() || !self.exists() {
            Err(GameErrorKind::SpecifiedElementNotFound.into())
        } else if !controllable.alive() {
            Err(GameErrorKind::YouNeedToContinueFirst.into())
        } else {
            let thrust = RangeTolerance::clamped_range(
                thrust,
                -self.maximum_thrust(),
                self.maximum_thrust(),
            )
            .map_err(|reason| GameErrorKind::InvalidArgument {
                reason,
                parameter: "thrust".to_string(),
            })?;

            controllable
                .cluster()
                .galaxy()
                .connection()
                .set_modern_ship_engine_subsystem_thrust(controllable.id(), self.slot(), thrust)
                .await
        }
    }

    #[inline]
    pub async fn off(&self) -> Result<(), GameError> {
        self.set_thrust(0.0).await
    }

    pub(crate) fn set_capabilities(
        &self,
        maximum_forward_thrust: f32,
        maximum_reverse_thrust: f32,
        maximum_thrust_change_per_tick: f32,
    ) {
        debug_assert!(
            (maximum_forward_thrust - maximum_reverse_thrust).abs() < 0.0001,
            "Modern engine capabilities are expected to be symetric."
        );
        if self.exists() {
            self.maximum_thrust
                .store(maximum_forward_thrust.max(maximum_reverse_thrust));
            self.maximum_thrust_change_per_tick
                .store(maximum_thrust_change_per_tick);
        } else {
            self.maximum_thrust.store(0.0);
            self.maximum_thrust_change_per_tick.store(0.0);
        }
    }

    pub(crate) const fn full_cost_maximum_thrust(maximum_thrust: f32) -> f32 {
        if maximum_thrust <= 0.0161 {
            3.8
        } else if maximum_thrust <= 0.0231 {
            5.6
        } else if maximum_thrust <= 0.0311 {
            7.2
        } else {
            10.0
        }
    }

    pub(crate) fn reset_runtime(&self) {
        self.current_thrust.store_default();
        self.target_thrust.store_default();
        self.consumed_energy_this_tick.store_default();
        self.consumed_ions_this_tick.store_default();
        self.consumed_neutrinos_this_tick.store_default();
        self.base.reset_runtime_status();
    }

    pub(crate) fn update_runtime(
        &self,
        current_thrust: f32,
        target_thrust: f32,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.current_thrust.store(current_thrust);
        self.target_thrust.store(target_thrust);
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
                FlattiverseEventKind::ModernShipEngineSubsystem {
                    controllable: self.controllable(),
                    slot: self.slot(),
                    status: self.status(),
                    current_thrust: self.current_thrust(),
                    target_thrust: self.target_thrust(),
                    consumed_energy_this_tick: self.consumed_energy_this_tick(),
                    consumed_ions_this_tick: self.consumed_ions_this_tick(),
                    consumed_neutrinos_this_tick: self.consumed_neutrinos_this_tick(),
                }
                .into(),
            )
        }
    }

    // TODO pub(crate) fn  refresh_tier(&self) {}

    // TODO pub(crate) fn ReplaceMaximumThrust
}

impl AsRef<SubsystemBase> for ModernShipEngineSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
