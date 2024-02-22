use crate::hierarchy::{ClusterId, RegionId, ShipDesignId, UpgradeId};
use crate::hierarchy::{ControllableInfoId, GlaxyId};
use crate::{Player, PlayerId, TeamId};
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
        ship: ShipDesignId,
    },
    /// The [`crate::Upgrade`] of the given [`crate::unit::ShipDesign`] in the given
    /// [`crate::hierarchy::Galaxy`] was upated.
    UpgradeUpdated {
        galaxy: GlaxyId,
        ship: ShipDesignId,
        upgrade: UpgradeId,
    },
    /// The [`crate::Player`] of the given [`crate::Galaxy`] was updated.
    PlayerUpdated {
        galaxy: GlaxyId,
        player: PlayerId,
    },
    PlayerRemoved {
        galaxy: GlaxyId,
        player: Player,
    },

    /// A new [`crate::unit::Unit`] became visible.
    SeeingNewUnit {
        galaxy: GlaxyId,
        cluster: ClusterId,
        name: String,
    },
    /// A watched [`crate::unit::Unit`] updated.N
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

    /// The [`crate::hierarchy::ControllableInfo`] for the given values was updated.
    ControllableInfoUpdated {
        galaxy: GlaxyId,
        cluster: ClusterId,
        player: PlayerId,
        controllable: ControllableInfoId,
    },
    /// The [`crate::hierarchy::ControllableInfo`] for the given values was removed.
    ControllableInfoRemoved {
        galaxy: GlaxyId,
        cluster: ClusterId,
        player: PlayerId,
        controllable: ControllableInfoId,
    },

    TickCompleted,
}
