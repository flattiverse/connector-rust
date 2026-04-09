use crate::galaxy_hierarchy::{Controllable, SubsystemBase, SubsystemExt};
use crate::utils::Atomic;
use crate::{FlattiverseEvent, FlattiverseEventKind, SubsystemSlot, SubsystemStatus};
use std::sync::Weak;

/// Passive armor subsystem of a controllable.
#[derive(Debug)]
pub struct ArmorSubsystem {
    base: SubsystemBase,
    reduction: Atomic<f32>,
    blocked_direct_damage_this_tick: Atomic<f32>,
    blocked_radiation_damage_this_tick: Atomic<f32>,
}

impl ArmorSubsystem {
    pub(crate) fn new(
        controllable: Weak<Controllable>,
        name: String,
        exists: bool,
        slot: SubsystemSlot,
    ) -> Self {
        Self {
            base: SubsystemBase::new(controllable, name, exists, slot),
            reduction: Atomic::from(if exists { 0.5 } else { 0.0 }),
            blocked_direct_damage_this_tick: Default::default(),
            blocked_radiation_damage_this_tick: Default::default(),
        }
    }

    pub(crate) fn create_classic_ship_armor(controllable: Weak<Controllable>) -> Self {
        Self::new(
            controllable,
            "Armor".to_string(),
            false,
            SubsystemSlot::Armor,
        )
    }

    /// Flat damage reduction applied before the hull.
    #[inline]
    pub fn reduction(&self) -> f32 {
        self.reduction.load()
    }

    /// Direct damage blocked during the current tick.
    #[inline]
    pub fn blocked_direct_damage_this_tick(&self) -> f32 {
        self.blocked_direct_damage_this_tick.load()
    }

    /// Radiation damage blocked during the current tick.
    #[inline]
    pub fn blocked_radiation_damage_this_tick(&self) -> f32 {
        self.blocked_radiation_damage_this_tick.load()
    }

    /// Total damage blocked during the current tick.
    #[inline]
    pub fn blocked_total_this_tick(&self) -> f32 {
        self.blocked_direct_damage_this_tick() + self.blocked_radiation_damage_this_tick()
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn set_reduction(&self, reduction: f32) {
        self.reduction
            .store(if self.exists() { reduction } else { 0.0 });

        // TODO self.refresh_tier();
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn reset_runtime(&self) {
        self.blocked_direct_damage_this_tick.store_default();
        self.blocked_radiation_damage_this_tick.store_default();
        self.base.reset_runtime_status();
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn update_runtime(
        &self,
        blocked_direct_damage_this_tick: f32,
        blocked_radiation_damage_this_tick: f32,
        status: SubsystemStatus,
    ) {
        self.blocked_direct_damage_this_tick
            .store(blocked_direct_damage_this_tick);
        self.blocked_radiation_damage_this_tick
            .store(blocked_radiation_damage_this_tick);
        self.base.update_runtime_status(status);
    }

    pub(crate) fn create_runtime_event(&self) -> Option<FlattiverseEvent> {
        if !self.exists() || !self.base.should_emit_runtime_event() {
            None
        } else {
            Some(
                FlattiverseEventKind::ArmorSubsystem {
                    controllable: self.controllable(),
                    slot: self.slot(),
                    status: self.status(),
                    reduction: self.reduction(),
                    blocked_direct_damage_this_tick: self.blocked_direct_damage_this_tick(),
                    blocked_radiation_damage_this_tick: self.blocked_radiation_damage_this_tick(),
                }
                .into(),
            )
        }
    }

    // TODO pub fn refresh_tier(&self) {}
}

impl AsRef<SubsystemBase> for ArmorSubsystem {
    #[inline]
    fn as_ref(&self) -> &SubsystemBase {
        &self.base
    }
}
