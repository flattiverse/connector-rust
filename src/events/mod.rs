use crate::hierarchy::ClusterId;
use crate::hierarchy::GlaxyId;
use crate::unit::{ShipId, UpgradeId};
use crate::{PlayerId, TeamId};
use std::time::Duration;

#[derive(Debug)]
pub enum FlattiverseEvent {
    PingMeasured(Duration),
    /// The [`crate::Galaxy`] of the given id was updaed.
    GalaxyUpdated(GlaxyId),
    /// The [`crate::Cluster`] of the given [`crate::Galaxy`] was updaed.
    ClusterUpdated {
        galaxy: GlaxyId,
        cluster: ClusterId,
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
    /// The [`crate::Upgrade`] of the given [`crate::Ship`] in the given [`crate::Galaxy`] was
    /// upated.
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
}
