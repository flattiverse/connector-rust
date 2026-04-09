use crate::galaxy_hierarchy::{Controllable, SubsystemKind, SubsystemTierInfo};
use crate::{GameError, GameErrorKind, SubsystemSlot, SubsystemStatus};
use std::future::Future;
use std::sync::Arc;

pub(crate) trait SystemExtIntern {
    fn set_exists(&self, exists: bool);
    fn set_tier(&self, tier: u8);
    fn set_reported_tier(&self, tier: u8) {
        self.set_tier(tier);
    }
}

#[allow(private_bounds)]
pub trait SubsystemExt: SystemExtIntern {
    /// The [Controllable] this subsystem belongs to.
    fn controllable(&self) -> Arc<Controllable>;

    /// A human-readable subsystem name.
    fn name(&self) -> &str;

    /// Whether the controllable actually provides this subsystem.
    fn exists(&self) -> bool;

    /// The concrete slot this subsystem occupies.
    fn slot(&self) -> SubsystemSlot;

    /// Logical subsystem family independent of the concrete slot.
    fn kind(&self) -> SubsystemKind;

    /// Current installed tier reported by the server.
    /// Tier 0 means that this slot is currently not installed.
    fn tier(&self) -> u8;

    /// Current target tier while a tier change is in progress.
    /// Equals [`SubsystemExt::tier`] when no tier change is pending for this slot.
    fn target_tier(&self) -> u8;

    /// Remaining ticks of the currently running upgrade or downgrade affecting this slot.
    /// Returns 0 when no tier change is pending.
    fn remaining_tier_change_ticks(&self) -> u16;

    /// Full static tier catalog for this subsystem family on the current ship type.
    fn tier_infos(&self) -> Arc<Vec<SubsystemTierInfo>>;

    /// Metadata of the currently installed tier.
    fn tier_info(&self) -> &SubsystemTierInfo;

    /// Metadata of the currently targeted tier during a running tier change.
    fn target_tier_info(&self) -> &SubsystemTierInfo;

    /// Starts one upgrade step for this subsystem slot.
    /// This also works for currently missing subsystems at tier 0.
    fn upgrade(&self) -> impl Future<Output = Result<(), GameError>> + Send {
        let controllable = self.controllable();
        let slot = self.slot();

        async move {
            if !controllable.active() {
                Err(GameErrorKind::SpecifiedElementNotFound.into())
            } else {
                controllable
                    .cluster()
                    .galaxy()
                    .connection()
                    .subsystem_upgrade(controllable.id(), slot)
                    .await
            }
        }
    }

    /// Starts one downgrade step for this subsystem slot.
    fn downgrade(&self) -> impl Future<Output = Result<(), GameError>> + Send {
        let controllable = self.controllable();
        let slot = self.slot();

        async move {
            if !controllable.active() {
                Err(GameErrorKind::SpecifiedElementNotFound.into())
            } else {
                controllable
                    .cluster()
                    .galaxy()
                    .connection()
                    .subsystem_downgrade(controllable.id(), slot)
                    .await
            }
        }
    }

    /// The latest status reported by the server.
    fn status(&self) -> SubsystemStatus;
}
