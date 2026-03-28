use crate::galaxy_hierarchy::{Controllable, SubsystemBase, SubsystemExt};
use crate::utils::Atomic;
use crate::{FlattiverseEvent, FlattiverseEventKind, SubsystemSlot, SubsystemStatus};
use std::sync::Weak;

#[derive(Debug)]
pub struct DynamicShotMagazineSubsystem {
    base: SubsystemBase,
    current_shots: Atomic<f32>,
}

impl DynamicShotMagazineSubsystem {
    const MAXIMUM_SHOTS_VALUE: f32 = 5.0;

    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: SubsystemBase::new(controllable, name, exists, slot),
            current_shots: Atomic::default(),
        }
    }

    /// The magazine capacity in shots.
    #[inline]
    pub fn maximum_shots(&self) -> f32 {
        Self::MAXIMUM_SHOTS_VALUE
    }

    /// The currently stored shots.
    #[inline]
    pub fn current_shots(&self) -> f32 {
        self.current_shots.load()
    }

    pub(crate) fn reset_runtime(&self) {
        self.current_shots.store_default();
        self.base.reset_runtime_status();
    }

    pub(crate) fn update_runtime(&self, current_shots: f32, status: SubsystemStatus) {
        self.current_shots.store(current_shots);
        self.base.update_runtime_status(status);
    }

    pub(crate) fn create_runtime_event(&self) -> Option<FlattiverseEvent> {
        if !self.exists() || !self.base.should_emit_runtime_event() {
            None
        } else {
            Some(
                FlattiverseEventKind::DynamicShotMagazineSubsystem {
                    controllable: self.controllable(),
                    slot: self.slot(),
                    status: self.status(),
                    current_shots: self.current_shots(),
                }
                .into(),
            )
        }
    }
}

impl AsRef<SubsystemBase> for DynamicShotMagazineSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
