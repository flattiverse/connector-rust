use crate::galaxy_hierarchy::{
    ControllableInfoBase, ControllableInfoId, Galaxy, Identifiable, Player, Score,
};
use crate::network::InvalidArgumentKind;
use crate::unit::UnitKind;
use crate::{GameError, GameErrorKind};
use std::sync::{Arc, Weak};

/// Persistent roster entry for one player-owned controllable.
/// This owner-side identity survives deaths and exists independently from the visible
/// [`PlayerUnit`] mirror in a cluster.
#[derive(Debug)]
pub enum ControllableInfo {
    /// Roster entry for one classic-ship controllable of a player.
    Classic { base: ControllableInfoBase },
    /// Roster entry for one new-ship controllable of a player.
    New { base: ControllableInfoBase },
}

impl ControllableInfo {
    /// The id of this ControllableInfo.
    #[inline]
    pub fn id(&self) -> ControllableInfoId {
        self.base().id()
    }

    /// The name of this controllable.
    #[inline]
    pub fn name(&self) -> &str {
        self.base().name()
    }

    /// The galaxy instance this ControllableInfo belongs to.
    #[inline]
    pub fn galaxy(&self) -> Arc<Galaxy> {
        self.base().galaxy()
    }

    /// The player who owns this controllable entry.
    #[inline]
    pub fn player(&self) -> Arc<Player> {
        self.base().player()
    }

    /// True while this controllable currently has an alive in-world runtime.
    #[inline]
    pub fn alive(&self) -> bool {
        self.base().alive()
    }

    /// True while this controllable registration still exists on the server.
    #[inline]
    pub fn active(&self) -> bool {
        self.base().active()
    }

    /// Current live score of this controllable inside one galaxy session.
    #[inline]
    pub fn score(&self) -> &Score {
        self.base().score()
    }

    /// Runtime unit kind this controllable uses while it is alive in the world.
    #[inline]
    pub fn kind(&self) -> UnitKind {
        match self {
            ControllableInfo::Classic { .. } => UnitKind::ClassicShipPlayerUnit,
            ControllableInfo::New { .. } => UnitKind::NewShipPlayerUnit,
        }
    }

    pub(crate) fn deactivate(&self) {
        self.base().deactivate();
    }

    pub(crate) fn set_alive(&self) {
        self.base().set_alive();
    }

    pub(crate) fn set_dead(&self) {
        self.base().set_dead();
    }

    pub fn base(&self) -> &ControllableInfoBase {
        match self {
            ControllableInfo::Classic { base, .. } => base,
            ControllableInfo::New { base, .. } => base,
        }
    }

    pub(crate) fn from_packet(
        kind: UnitKind,
        galaxy: Weak<Galaxy>,
        player: Weak<Player>,
        id: ControllableInfoId,
        name: String,
        alive: bool,
    ) -> Result<Self, GameError> {
        let base = ControllableInfoBase::new(galaxy, player, id, name, alive);
        match kind {
            UnitKind::ClassicShipPlayerUnit => Ok(Self::Classic { base }),
            UnitKind::NewShipPlayerUnit => Ok(Self::New { base }),
            _ => Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::Unknown(Default::default()),
                parameter: "kind".to_string(),
            }
            .into()),
        }
    }
}

impl Identifiable<ControllableInfoId> for ControllableInfo {
    #[inline]
    fn id(&self) -> ControllableInfoId {
        self.base().id()
    }
}
