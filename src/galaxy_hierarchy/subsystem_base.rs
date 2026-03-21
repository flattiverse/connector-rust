use crate::galaxy_hierarchy::{Controllable, SubsystemExt};
use crate::utils::Atomic;
use crate::{SubsystemSlot, SubsystemStatus};
use std::sync::{Arc, Weak};

/// Base type for persistent controllable subsystems.
#[derive(Debug)]
pub struct SubsystemBase {
    controllable: Weak<Controllable>,
    name: String,
    exists: bool,
    slot: SubsystemSlot,
    status: Atomic<SubsystemStatus>,
    hast_last_emitted_status: Atomic<bool>,
    last_emitted_status: Atomic<SubsystemStatus>,
}

impl SubsystemBase {
    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            controllable,
            name,
            exists,
            slot,
            status: Atomic::default(),
            hast_last_emitted_status: Atomic::default(),
            last_emitted_status: Atomic::default(),
        }
    }

    pub(crate) fn reset_runtime_status(&self) {
        self.status.store(SubsystemStatus::Off);
        self.hast_last_emitted_status.store(false);
        self.last_emitted_status.store(SubsystemStatus::Off);
    }

    pub(crate) fn update_runtime_status(&self, status: SubsystemStatus) {
        if self.exists() {
            self.status.store(status);
        } else {
            self.status.store(SubsystemStatus::Off)
        }
    }

    /// Returns whether a runtime event should be emitted for status transactions or active ticks.
    pub(crate) fn should_emit_runtime_event(&self) -> bool {
        let status = self.status.load();
        match status {
            SubsystemStatus::Worked | SubsystemStatus::Failed => {
                self.last_emitted_status.store(status);
                self.hast_last_emitted_status.store(true);
                true
            }
            _ if !self.hast_last_emitted_status.load()
                || self.last_emitted_status.load() != status =>
            {
                self.last_emitted_status.store(status);
                self.hast_last_emitted_status.store(true);
                true
            }
            _ => false,
        }
    }
}

pub trait AsSubsystemBase {
    fn as_subsystem_base(&self) -> &SubsystemBase;
}

impl AsSubsystemBase for SubsystemBase {
    #[inline(always)]
    fn as_subsystem_base(&self) -> &SubsystemBase {
        self
    }
}

impl<T: AsRef<SubsystemBase>> AsSubsystemBase for T {
    #[inline]
    fn as_subsystem_base(&self) -> &SubsystemBase {
        AsRef::<SubsystemBase>::as_ref(self)
    }
}

impl<T: AsSubsystemBase> SubsystemExt for T {
    #[inline]
    fn controllable(&self) -> Arc<Controllable> {
        self.as_subsystem_base().controllable.upgrade().unwrap()
    }

    #[inline]
    fn name(&self) -> &str {
        &self.as_subsystem_base().name
    }

    /// Whether the controllable actually provides this subsystem.
    #[inline]
    fn exists(&self) -> bool {
        self.as_subsystem_base().exists
    }

    /// The concrete slot this subsystem occupies.
    #[inline]
    fn slot(&self) -> SubsystemSlot {
        self.as_subsystem_base().slot
    }

    /// The latest status reported by the server.
    #[inline]
    fn status(&self) -> SubsystemStatus {
        self.as_subsystem_base().status.load()
    }
}
