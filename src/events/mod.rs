use crate::hierarchy::GlaxyId;
use crate::hierarchy::{ClusterId, RegionId};
use crate::unit::ShipId;
use crate::{PlayerId, TeamId, UpgradeId};
use std::time::Duration;

#[derive(Debug)]
pub enum FlattiverseEvent {
    PingMeasured(Duration),
    /// The [`crate::hierarchy::Cluster`] of the given id was updaed.
    GalaxyUpdated(GlaxyId),
    /// The [`crate::Cluster`] of the given [`crate::Galaxy`] was updaed.
    ClusterUpdated {
        galaxy: GlaxyId,
        cluster: ClusterId,
    },
    /// The [`crate::hierarchy::Region`] of the given id was updaed.
    RegionUpdated {
        galaxy: GlaxyId,
        cluster: ClusterId,
        region: RegionId,
    },
    /// The [`crate::Team`] of the given [`crate::Galaxy`] was updated.
    TeamUpdated {
        galaxy: GlaxyId,
        team: TeamId,
    },
    /// The [`crate::Ship`] of the given [`crate::Galaxy`] was updated.
    ShipUpdated {
        galaxy: GlaxyId,
        ship: ShipId,
    },
    /// The [`crate::Upgrade`] of the given [`crate::unit::Ship`] in the given
    /// [`crate::hierarchy::Galaxy`] was upated.
    UpgradeUpdated {
        galaxy: GlaxyId,
        ship: ShipId,
        upgrade: UpgradeId,
    },
    /// The [`crate::Player`] of the given [`crate::Galaxy`] was updated.
    PlayerUpdated {
        galaxy: GlaxyId,
        player: PlayerId,
    },

    /// A new [`crate::unit::Unit`] became visible.
    SeeingNewUnit {
        galaxy: GlaxyId,
        cluster: ClusterId,
        name: String,
    },

    /// A watched [`crate::unit::Unit`] updated.
    SeeingUnitUpdated {
        galaxy: GlaxyId,
        cluster: ClusterId,
        name: String,
    },

    /// A watched [`crate::unit::Unit`] vanished.
    SeeingUnitNoMore {
        galaxy: GlaxyId,
        cluster: ClusterId,
        name: String,
    },

    TickCompleted,
}
