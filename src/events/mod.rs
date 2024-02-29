use crate::hierarchy::{Cluster, Region};
use crate::hierarchy::{ControllableInfo, ShipDesign, ShipUpgrade};
use crate::unit::Unit;
use crate::{Controllable, Player, Team};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub enum FlattiverseEvent {
    PingMeasured(Duration),
    /// The [`Galaxy`] instance has been updated.
    GalaxyUpdated,
    /// The [`crate::hierarchy::Cluster`] of the given [`crate::hierarchy::Galaxy`] was created.
    ClusterCreated {
        cluster: Arc<Cluster>,
    },
    /// The [`crate::hierarchy::Cluster`] of the given [`crate::hierarchy::Galaxy`] was updated.
    ClusterUpdated {
        cluster: Arc<Cluster>,
    },
    /// The [`crate::hierarchy::Cluster`] of the given [`crate::hierarchy::Galaxy`] was removed.
    ClusterRemoved {
        cluster: Arc<Cluster>,
    },
    /// The [`crate::hierarchy::Region`] of the given id was created.
    RegionCreated {
        region: Arc<Region>,
    },
    /// The [`crate::hierarchy::Region`] of the given id was updated.
    RegionUpdated {
        region: Arc<Region>,
    },
    /// The [`crate::hierarchy::Region`] of the given id was removed.
    RegionRemoved {
        region: Arc<Region>,
    },
    /// The [`crate::Team`] of the given [`crate::hierarchy::Galaxy`] was created.
    TeamCreated {
        team: Arc<Team>,
    },
    /// The [`crate::Team`] of the given [`crate::hierarchy::Galaxy`] was updated.
    TeamUpdated {
        team: Arc<Team>,
    },
    /// The [`crate::Team`] of the given [`crate::hierarchy::Galaxy`] was removed.
    TeamRemoved {
        team: Arc<Team>,
    },
    /// The [`crate::hierarchy::ShipDesign`] of the given [`crate::hierarchy::Galaxy`] was created.
    ShipDesignCreated {
        ship_design: Arc<ShipDesign>,
    },
    /// The [`crate::hierarchy::ShipUpgrade`] of the given [`crate::hierarchy::ShipDesign`] in the
    /// given [`crate::hierarchy::Galaxy`] was upated.
    UpgradeUpdated {
        upgrade: Arc<ShipUpgrade>,
    },
    /// The [`crate::Player`] of the given [`crate::hierarchy::Galaxy`] has joined the game.
    PlayerJoined {
        player: Arc<Player>,
    },
    /// The [`crate::Player`] of the given [`crate::hierarchy::Galaxy`] has left the game.
    PlayerParted {
        player: Arc<Player>,
    },

    /// A new [`crate::unit::Unit`] became visible.
    SeeingNewUnit {
        unit: Arc<dyn Unit>,
    },
    /// A watched [`crate::unit::Unit`] updated.N
    SeeingUnitUpdated {
        unit: Arc<dyn Unit>,
    },
    /// A watched [`crate::unit::Unit`] vanished.
    SeeingUnitNoMore {
        unit: Arc<dyn Unit>,
    },

    /// The [`crate::hierarchy::ControllableInfo`] for the given values was created.
    ControllableInfoCreated {
        controllable_info: Arc<ControllableInfo>,
    },
    /// The [`crate::hierarchy::ControllableInfo`] for the given values was updated.
    ControllableInfoUpdated {
        controllable_info: Arc<ControllableInfo>,
    },
    /// The [`crate::hierarchy::ControllableInfo`] for the given values was removed.
    ControllableInfoRemoved {
        controllable_info: Arc<ControllableInfo>,
    },

    /// The [`crate::controllable::Controllable`] for the given values has joined the game.
    ControllableJoined {
        controllable: Arc<Controllable>,
    },
    /// The [`crate::controllable::Controllable`] for the given values was updated.
    ControllableUpdated {
        controllable: Arc<Controllable>,
    },
    /// The [`crate::controllable::Controllable`] for the given values hsa left the game.
    ControllableRemoved {
        controllable: Arc<Controllable>,
    },

    /// Received a message from the given player.
    PlayerChatMessageReceived {
        time: SystemTime,
        player: Arc<Player>,
        message: String,
    },
    /// Received a message from the given team.
    TeamChatMessageReceived {
        time: SystemTime,
        player: Arc<Player>,
        message: String,
    },
    /// Received a message from the given galaxy.
    GalaxyChatMessageReceived {
        time: SystemTime,
        player: Arc<Player>,
        message: String,
    },

    TickCompleted,
    ConnectionClosed,
}
