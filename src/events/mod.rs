use crate::hierarchy::{ClusterId, RegionId, ShipDesignId, ShipUpgradeId};
use crate::hierarchy::{ControllableInfoId, GalaxyId};
use crate::{ControllableId, PlayerId, TeamId};
use std::time::Duration;

#[derive(Debug)]
pub enum FlattiverseEvent {
    PingMeasured(Duration),
    /// The [`crate::hierarchy::Cluster`] of the given id was updated.
    GalaxyUpdated(GalaxyId),
    /// The [`crate::Cluster`] of the given [`crate::hierarchy::Galaxy`] was updated.
    ClusterUpdated {
        galaxy: GalaxyId,
        cluster: ClusterId,
    },
    /// The [`crate::Cluster`] of the given [`crate::hierarchy::Galaxy`] was removed.
    ClusterRemoved {
        galaxy: GalaxyId,
        cluster: ClusterId,
    },
    /// The [`crate::hierarchy::Region`] of the given id was updated.
    RegionUpdated {
        galaxy: GalaxyId,
        cluster: ClusterId,
        region: RegionId,
    },
    /// The [`crate::hierarchy::Region`] of the given id was removed.
    RegionRemoved {
        galaxy: GalaxyId,
        cluster: ClusterId,
        region: RegionId,
    },
    /// The [`crate::Team`] of the given [`crate::hierarchy::Galaxy`] was updated.
    TeamUpdated {
        galaxy: GalaxyId,
        team: TeamId,
    },
    /// The [`crate::Team`] of the given [`crate::hierarchy::Galaxy`] was removed.
    TeamRemoved {
        galaxy: GalaxyId,
        team: TeamId,
    },
    /// The [`crate::hierarchy::ShipDesign`] of the given [`crate::hierarchy::Galaxy`] was updated.
    ShipDesignUpdated {
        galaxy: GalaxyId,
        ship_design: ShipDesignId,
    },
    /// The [`crate::hierarchy::ShipUpgrade`] of the given [`crate::hierarchy::ShipDesign`] in the
    /// given [`crate::hierarchy::Galaxy`] was upated.
    UpgradeUpdated {
        galaxy: GalaxyId,
        ship: ShipDesignId,
        upgrade: ShipUpgradeId,
    },
    /// The [`crate::Player`] of the given [`crate::hierarchy::Galaxy`] was updated.
    PlayerUpdated {
        galaxy: GalaxyId,
        player: PlayerId,
    },
    /// The [`crate::Player`] of the given [`crate::hierarchy::Galaxy`] was removed.
    PlayerRemoved {
        galaxy: GalaxyId,
        player: PlayerId,
    },

    /// A new [`crate::unit::Unit`] became visible.
    SeeingNewUnit {
        galaxy: GalaxyId,
        cluster: ClusterId,
        name: String,
    },
    /// A watched [`crate::unit::Unit`] updated.N
    SeeingUnitUpdated {
        galaxy: GalaxyId,
        cluster: ClusterId,
        name: String,
    },
    /// A watched [`crate::unit::Unit`] vanished.
    SeeingUnitNoMore {
        galaxy: GalaxyId,
        cluster: ClusterId,
        name: String,
    },

    /// The [`crate::hierarchy::ControllableInfo`] for the given values was updated.
    ControllableInfoUpdated {
        galaxy: GalaxyId,
        player: PlayerId,
        controllable_info: ControllableInfoId,
    },
    /// The [`crate::hierarchy::ControllableInfo`] for the given values was removed.
    ControllableInfoRemoved {
        galaxy: GalaxyId,
        player: PlayerId,
        controllable_info: ControllableInfoId,
    },

    /// The [`crate::controllable::Controllable`] for the given values was updated.
    ControllableUpdated {
        galaxy: GalaxyId,
        controllable: ControllableId,
    },
    /// The [`crate::controllable::Controllable`] for the given values was removed.
    ControllableRemoved {
        galaxy: GalaxyId,
        controllable: ControllableId,
    },

    TickCompleted,
}
