use crate::utils::Atomic;
use crate::SubsystemStatus;

/// Visible snapshot of an armor subsystem on a scanned player unit.
#[derive(Debug, Clone, Default)]
pub struct ArmorSubsystemInfo {
    exists: Atomic<bool>,
    reduction: Atomic<f32>,
    status: Atomic<SubsystemStatus>,
    blocked_direct_damage_this_tick: Atomic<f32>,
    blocked_radiation_damage_this_tick: Atomic<f32>,
}

impl ArmorSubsystemInfo {
    /// Indicates whether the subsystem exists on the scanned unit.
    #[inline]
    pub fn exists(&self) -> bool {
        self.exists.load()
    }

    /// Flat damage reduction applied before hull damage is computed.
    /// Armor has no own hit points; it only reduces incoming direct and radiation damage.
    #[inline]
    pub fn reduction(&self) -> f32 {
        self.reduction.load()
    }

    /// Tick-local runtime status reported for the armor subsystem.
    #[inline]
    pub fn status(&self) -> SubsystemStatus {
        self.status.load()
    }

    /// Direct collision or weapon damage blocked during the current server tick.
    #[inline]
    pub fn blocked_direct_damage_this_tick(&self) -> f32 {
        self.blocked_direct_damage_this_tick.load()
    }

    /// Radiation damage blocked during the current server tick.
    #[inline]
    pub fn blocked_radiation_damage_this_tick(&self) -> f32 {
        self.blocked_radiation_damage_this_tick.load()
    }

    pub(crate) fn update(
        &self,
        exists: bool,
        reduction: f32,
        status: SubsystemStatus,
        blocked_direct_damage_this_tick: f32,
        blocked_radiation_damage_this_tick: f32,
    ) {
        self.exists.store(exists);
        if exists {
            self.reduction.store(reduction);
            self.status.store(status);
            self.blocked_direct_damage_this_tick
                .store(blocked_direct_damage_this_tick);
            self.blocked_radiation_damage_this_tick
                .store(blocked_radiation_damage_this_tick);
        } else {
            self.reduction.store(0.0);
            self.status.store(SubsystemStatus::Off);
            self.blocked_direct_damage_this_tick.store(0.0);
            self.blocked_radiation_damage_this_tick.store(0.0);
        }
    }
}
