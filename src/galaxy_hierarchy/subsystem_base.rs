use crate::galaxy_hierarchy::{
    Controllable, SubsystemExt, SubsystemKind, SubsystemTierInfo, SystemExtIntern,
};
use crate::unit::UnitKind;
use crate::utils::Atomic;
use crate::{SubsystemSlot, SubsystemStatus};
use arc_swap::ArcSwapWeak;
use std::sync::{Arc, Weak};

/// Base type for persistent controllable subsystems.
#[derive(Debug)]
pub struct SubsystemBase {
    pub(crate) controllable: ArcSwapWeak<Controllable>,
    name: String,
    exists: Atomic<bool>,
    slot: SubsystemSlot,
    tier: Atomic<u8>,
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
            controllable: ArcSwapWeak::new(controllable),
            name,
            exists: Atomic::from(exists),
            slot,
            tier: Atomic::from(0),
            status: Atomic::default(),
            hast_last_emitted_status: Atomic::default(),
            last_emitted_status: Atomic::default(),
        }
    }

    pub(crate) fn modern_ship(&self) -> bool {
        self.controllable
            .load()
            .upgrade()
            .map(|controllable| controllable.kind() == UnitKind::ModernShipPlayerUnit)
            .unwrap_or_default()
    }

    pub(crate) fn static_tier_infos(&self) -> &'static [SubsystemTierInfo] {
        todo!()
    }

    pub(crate) fn current_structural_load(&self) -> f32 {
        todo!()
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn reset_runtime_status(&self) {
        self.status.store(SubsystemStatus::Off);
        self.hast_last_emitted_status.store(false);
        self.last_emitted_status.store(SubsystemStatus::Off);
    }

    pub(crate) fn matches(left: f32, right: f32) -> bool {
        (left - right).abs() <= 0.0001
    }

    #[instrument(level = "trace", skip(self))]
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

impl<T: AsSubsystemBase> SystemExtIntern for T {
    #[instrument(level = "trace", skip(self))]
    fn set_exists(&self, exists: bool) {
        self.as_subsystem_base().exists.store(exists);

        // TODO refresh_tier()

        if !exists {
            self.as_subsystem_base().reset_runtime_status()
        }
    }

    #[inline]
    fn set_tier(&self, tier: u8) {
        self.as_subsystem_base().tier.store(tier);
    }
}

impl<T: AsSubsystemBase> SubsystemExt for T {
    #[inline]
    fn controllable(&self) -> Arc<Controllable> {
        self.as_subsystem_base()
            .controllable
            .load()
            .upgrade()
            .unwrap()
    }

    #[inline]
    fn name(&self) -> &str {
        &self.as_subsystem_base().name
    }

    #[inline]
    fn exists(&self) -> bool {
        self.as_subsystem_base().exists.load()
    }

    #[inline]
    fn slot(&self) -> SubsystemSlot {
        self.as_subsystem_base().slot
    }

    #[inline]
    fn kind(&self) -> SubsystemKind {
        todo!()
    }

    #[inline]
    fn tier(&self) -> u8 {
        self.as_subsystem_base().tier.load()
    }

    #[inline]
    fn target_tier(&self) -> u8 {
        todo!()
    }

    #[inline]
    fn remaining_tier_change_ticks(&self) -> u16 {
        todo!()
    }

    #[inline]
    fn tier_infos(&self) -> Arc<Vec<SubsystemTierInfo>> {
        todo!()
    }

    #[inline]
    fn tier_info(&self) -> &SubsystemTierInfo {
        todo!()
    }

    #[inline]
    fn target_tier_info(&self) -> &SubsystemTierInfo {
        todo!()
    }

    #[inline]
    fn status(&self) -> SubsystemStatus {
        self.as_subsystem_base().status.load()
    }
}
