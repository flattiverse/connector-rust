use crate::galaxy_hierarchy::{
    AsSubsystemBase, DynamicShotMagazineSubsystem, SubsystemBase, SubsystemExt,
};
use crate::{FlattiverseEvent, FlattiverseEventKind, SubsystemStatus};

/// Dynamic interceptor magazine subsystem of a controllable.
#[derive(Debug)]
pub struct DynamicInterceptorMagazineSubsystem {
    base: DynamicShotMagazineSubsystem,
}

impl DynamicInterceptorMagazineSubsystem {
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
        if !self.exists() || !self.as_subsystem_base().should_emit_runtime_event() {
            None
        } else {
            Some(
                FlattiverseEventKind::DynamicInterceptorMagazineSubsystem {
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

impl AsRef<SubsystemBase> for DynamicInterceptorMagazineSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        self.base.as_ref()
    }
}
