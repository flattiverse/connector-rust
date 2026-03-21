use crate::galaxy_hierarchy::{Controllable, SubsystemBase, SubsystemExt};
use crate::utils::{Also, Atomic};
use crate::{FlattiverseEvent, FlattiverseEventKind, SubsystemSlot, SubsystemStatus};
use std::sync::Weak;

/// Passive battery subsystem of a controllable.
#[derive(Debug)]
pub struct BatterySubsystem {
    base: SubsystemBase,
    maximum: Atomic<f32>,
    current: Atomic<f32>,
    consumed_this_tick: Atomic<f32>,
}

impl BatterySubsystem {
    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        maximum: f32,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: SubsystemBase::new(controllable, name, exists, slot),
            maximum: Atomic::default(),
            current: Atomic::default(),
            consumed_this_tick: Atomic::default(),
        }
        .also(|it| it.set_maximum(maximum))
    }

    #[inline]
    pub(crate) fn create_classic_ship_energy_battery(controllable: Weak<Controllable>) -> Self {
        Self::new(
            controllable,
            "EnergyBattery".to_string(),
            true,
            20.000,
            SubsystemSlot::EnergyBattery,
        )
    }

    #[inline]
    pub(crate) fn create_missing_battery(
        controllable: Weak<Controllable>,
        name: String,
        slot: SubsystemSlot,
    ) -> Self {
        Self::new(controllable, name, false, 0.0, slot)
    }

    /// The maximum storable amount for this battery.
    #[inline]
    pub fn maximum(&self) -> f32 {
        self.maximum.load()
    }

    /// The current stored amount.
    #[inline]
    pub fn current(&self) -> f32 {
        self.current.load()
    }

    /// The currently free battery capacity.
    #[inline]
    pub fn free(&self) -> f32 {
        self.maximum() - self.current()
    }

    /// The amount consumed during the current server tick.
    #[inline]
    pub fn consumed_this_tick(&self) -> f32 {
        self.consumed_this_tick.load()
    }

    pub(crate) fn set_maximum(&mut self, maximum: f32) {
        self.maximum
            .store(if self.base.exists() { maximum } else { 0.0 });

        let current = self.current.load();
        let maximum = self.maximum.load();

        if current > maximum {
            self.current.store(maximum);
        }
    }

    pub(crate) fn reset_runtime(&self) {
        self.current.store(0.0);
        self.consumed_this_tick.store(0.0);
        self.base.reset_runtime_status();
    }

    pub(crate) fn update_runtime(
        &self,
        current: f32,
        consumed_this_tick: f32,
        status: SubsystemStatus,
    ) {
        self.current.store(current);
        self.consumed_this_tick.store(consumed_this_tick);
        self.base.update_runtime_status(status);
    }

    pub(crate) fn create_runtime_event(&self) -> Option<FlattiverseEvent> {
        if !self.base.exists() || !self.base.should_emit_runtime_event() {
            None
        } else {
            Some(
                FlattiverseEventKind::BatterySubsystem {
                    controllable: self.base.controllable(),
                    slot: self.base.slot(),
                    status: self.base.status(),
                    current: self.current(),
                    consumed_this_tick: self.consumed_this_tick(),
                }
                .into(),
            )
        }
    }
}

impl AsRef<SubsystemBase> for BatterySubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
