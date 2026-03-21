use crate::galaxy_hierarchy::{Controllable, SubsystemBase, SubsystemExt};
use crate::utils::{Also, Atomic};
use crate::{FlattiverseEvent, FlattiverseEventKind, SubsystemSlot};
use std::sync::Weak;

/// Passive energy cell subsystem of a controllable
#[derive(Debug)]
pub struct EnergyCellSubsystem {
    base: SubsystemBase,
    efficiency: Atomic<f32>,
    collected_this_tick: Atomic<f32>,
}

impl EnergyCellSubsystem {
    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        efficiency: f32,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: SubsystemBase::new(controllable, name, exists, slot),
            efficiency: Atomic::default(),
            collected_this_tick: Atomic::default(),
        }
        .also(|it| it.set_efficiency(efficiency))
    }

    #[inline]
    pub(crate) fn create_classic_ship_energy_cell(controllable: Weak<Controllable>) -> Self {
        Self::new(
            controllable,
            "EnergyCell".to_string(),
            true,
            0.4,
            SubsystemSlot::EnergyCell,
        )
    }

    #[inline]
    pub(crate) fn create_missing_cell(
        controllable: Weak<Controllable>,
        name: String,
        slot: SubsystemSlot,
    ) -> Self {
        Self::new(controllable, name, false, 0.0, slot)
    }

    /// The loading efficiency of this cell.
    #[inline]
    pub fn efficiency(&self) -> f32 {
        self.efficiency.load()
    }

    /// The amount collected through this cell during the current server tick.
    #[inline]
    pub fn collected_this_tick(&self) -> f32 {
        self.collected_this_tick.load()
    }

    pub(crate) fn set_efficiency(&self, efficiency: f32) {
        self.efficiency
            .store(if self.base.exists() { efficiency } else { 0.0 });
    }

    pub(crate) fn reset_runtime(&self) {
        self.collected_this_tick.store(0.0);
        self.base.reset_runtime_status();
    }

    pub(crate) fn create_runtime_event(&self) -> Option<FlattiverseEvent> {
        if !self.base.exists() || !self.base.should_emit_runtime_event() {
            None
        } else {
            Some(
                FlattiverseEventKind::EnergyCellSubsystem {
                    controllable: self.base.controllable(),
                    slot: self.base.slot(),
                    status: self.base.status(),
                    collected_this_tick: self.collected_this_tick(),
                }
                .into(),
            )
        }
    }
}

impl AsRef<SubsystemBase> for EnergyCellSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
