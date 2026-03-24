use crate::galaxy_hierarchy::{Controllable, SubsystemBase, SubsystemExt};
use crate::utils::{Also, Atomic};
use crate::{FlattiverseEvent, FlattiverseEventKind, SubsystemSlot, SubsystemStatus};
use std::sync::Weak;

/// Hull integrity subsystem of a controllable.
#[derive(Debug)]
pub struct HullSubsystem {
    base: SubsystemBase,
    maximum: Atomic<f32>,
    current: Atomic<f32>,
}

impl HullSubsystem {
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
        }
        .also(|it| it.set_maximum(maximum))
    }

    pub(crate) fn create_classic_ship_hull(controllable: Weak<Controllable>) -> Self {
        Self::new(
            controllable,
            String::from("Hull"),
            true,
            50.0,
            SubsystemSlot::Hull,
        )
    }

    /// The maximum hull integrity.
    #[inline]
    pub fn maximum(&self) -> f32 {
        self.maximum.load()
    }

    /// The current hull integrity.
    #[inline]
    pub fn current(&self) -> f32 {
        self.current.load()
    }

    pub(crate) fn set_maximum(&mut self, maximum: f32) {
        self.maximum
            .store(if self.exists() { maximum } else { 0.0 });

        if self.current() > maximum {
            self.current.store(maximum)
        }
    }

    pub(crate) fn reset_runtime(&self) {
        self.current.store(0.0);
        self.base.reset_runtime_status();
    }

    pub(crate) fn update_runtime(&self, current: f32, status: SubsystemStatus) {
        self.current.store(current);
        self.base.update_runtime_status(status);
    }

    pub(crate) fn create_runtime_event(&self) -> Option<FlattiverseEvent> {
        if !self.exists() || !self.base.should_emit_runtime_event() {
            None
        } else {
            Some(
                FlattiverseEventKind::HullSubsystem {
                    controllable: self.controllable(),
                    slot: self.slot(),
                    status: self.status(),
                    current: self.current(),
                }
                .into(),
            )
        }
    }
}

impl AsRef<SubsystemBase> for HullSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
