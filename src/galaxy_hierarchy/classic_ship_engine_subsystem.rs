use crate::galaxy_hierarchy::{Controllable, Cost, RangeTolerance, SubsystemBase, SubsystemExt};
use crate::utils::Atomic;
use crate::{
    FlattiverseEvent, FlattiverseEventKind, GameError, GameErrorKind, SubsystemSlot,
    SubsystemStatus, Vector,
};
use std::sync::Weak;

/// Engine subsystem of a classic ship controllable.
#[derive(Debug)]
pub struct ClassicShipEngineSubsystem {
    base: SubsystemBase,
    maximum: f32,
    current: Atomic<Vector>,
    target: Atomic<Vector>,
    consumed_energy_this_tick: Atomic<f32>,
    consumed_ions_this_tick: Atomic<f32>,
    consumed_neutrinos_this_tick: Atomic<f32>,
}

impl ClassicShipEngineSubsystem {
    pub(crate) fn new(controllable: Weak<Controllable>) -> Self {
        Self {
            base: SubsystemBase::new(
                controllable,
                "Engine".to_string(),
                true,
                SubsystemSlot::PrimaryEnergy,
            ),
            maximum: 0.1,
            current: Atomic::default(),
            target: Atomic::default(),
            consumed_energy_this_tick: Atomic::default(),
            consumed_ions_this_tick: Atomic::default(),
            consumed_neutrinos_this_tick: Atomic::default(),
        }
    }

    /// The maximum configurable movement vector length.
    #[inline]
    pub fn maximum(&self) -> f32 {
        self.maximum
    }

    /// The current server-applied movement impulse.
    #[inline]
    pub fn current(&self) -> Vector {
        self.current.load()
    }

    /// The current target movement impulse.
    #[inline]
    pub fn target(&self) -> Vector {
        self.target.load()
    }

    /// The energy consumed by the engine during the current server tick.
    #[inline]
    pub fn consumed_energy_this_tick(&self) -> f32 {
        self.consumed_energy_this_tick.load()
    }

    /// The ions consumed by the engine during the current server tick.
    #[inline]
    pub fn consumed_ions_this_tick(&self) -> f32 {
        self.consumed_ions_this_tick.load()
    }

    /// The neutrinos consumed by the engine during the current server tick.
    #[inline]
    pub fn consumed_neutrinos_this_tick(&self) -> f32 {
        self.consumed_neutrinos_this_tick.load()
    }

    /// Calculates the current placeholder engine tick costs for the requested movement vector.
    /// The current formular is `energy = 12_000 * movement.length()^3`.
    /// Returns `None` when the subsystem does not exist or the movement is outside the valid range.
    /// Values just above the maximum are clipped to the maximum before the cost is calculated.
    pub fn calculate_cost(&self, movement: Vector) -> Option<Cost> {
        if !self.exists() {
            None
        } else {
            let clamped = RangeTolerance::clamped_maximum_vector(movement, self.maximum()).ok()?;

            Cost::default()
                .with_energy(clamped.length() * clamped.length() * clamped.length() * 12_000_f32)
                .into_values_checked()
        }
    }

    /// Sets the target movement impulse on the server.
    /// Values just above the maximum are clipped to the maximum before tey are sent.
    pub async fn set(&self, movement: Vector) -> Result<(), GameError> {
        let controllable = self.controllable();

        if !controllable.active() || !self.exists() {
            Err(GameErrorKind::SpecifiedElementNotFound.into())
        } else if !controllable.alive() {
            Err(GameErrorKind::YouNeedToContinueFirst.into())
        } else {
            let movement = RangeTolerance::clamped_maximum_vector(movement, self.maximum())
                .map_err(|reason| GameErrorKind::InvalidArgument {
                    reason,
                    parameter: "movement".to_string(),
                })?;

            controllable
                .cluster()
                .galaxy()
                .connection()
                .classic_ship_engine_subsystem_set(controllable.id(), movement)
                .await
        }
    }

    /// Turns the engine off by requesting a zero movement vector.
    pub async fn off(&self) -> Result<(), GameError> {
        self.set(Vector::default()).await
    }

    pub(crate) fn reset_runtime(&self) {
        self.current.store_default();
        self.target.store_default();
        self.consumed_energy_this_tick.store(0.0);
        self.consumed_ions_this_tick.store(0.0);
        self.consumed_neutrinos_this_tick.store(0.0);
        self.base.reset_runtime_status();
    }

    pub(crate) fn update_runtime(
        &self,
        current: Vector,
        target: Vector,
        status: SubsystemStatus,
        consumed_energy_this_tick: f32,
        consumed_ions_this_tick: f32,
        consumed_neutrinos_this_tick: f32,
    ) {
        self.current.store(current);
        self.target.store(target);
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
                FlattiverseEventKind::ClassicShipEngineSubsystem {
                    controllable: self.controllable(),
                    slot: self.slot(),
                    status: self.status(),
                    current: self.current(),
                    target: self.target(),
                    consumed_energy_this_tick: self.consumed_energy_this_tick(),
                    consumed_ions_this_tick: self.consumed_ions_this_tick(),
                    consumed_neutrinos_this_tick: self.consumed_neutrinos_this_tick(),
                }
                .into(),
            )
        }
    }
}

impl AsRef<SubsystemBase> for ClassicShipEngineSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
