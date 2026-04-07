use crate::galaxy_hierarchy::{Controllable, DynamicShotMagazineSubsystem, SubsystemBase};
use crate::{FlattiverseEvent, SubsystemSlot, SubsystemStatus};
use std::sync::Weak;

/// Static shot magazine subsystem of a modern ship.
#[derive(Debug)]
pub struct StaticShotMagazineSubsystem {
    base: DynamicShotMagazineSubsystem,
}

impl StaticShotMagazineSubsystem {
    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: DynamicShotMagazineSubsystem::new(controllable, name, exists, slot),
        }
    }

    /// The magazine capacity in shots.
    #[inline]
    pub fn maximum_shots(&self) -> f32 {
        self.base.maximum_shots()
    }

    /// The currently stored shots.
    #[inline]
    pub fn current_shots(&self) -> f32 {
        self.base.current_shots()
    }

    pub(crate) fn reset_runtime(&self) {
        self.base.reset_runtime()
    }

    pub(crate) fn update_runtime(&self, current_shots: f32, status: SubsystemStatus) {
        self.base.update_runtime(current_shots, status);
    }

    pub(crate) fn create_runtime_event(&self) -> Option<FlattiverseEvent> {
        self.base.create_runtime_event()
    }
}

impl AsRef<SubsystemBase> for StaticShotMagazineSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        self.base.as_ref()
    }
}
