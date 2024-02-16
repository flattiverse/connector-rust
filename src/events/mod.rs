use std::time::Duration;

#[derive(Debug)]
pub enum FlattiverseEvent {
    PingMeasured(Duration),
    /// The [`crate::Galaxy`] of the given id was updaed.
    GalaxyUpdated(i32),
    /// The [`crate::Cluster`] of the given [`crate::Galaxy`] was updaed.
    ClusterUpdated {
        galaxy: i32,
        cluster: u8,
    },
    /// The [`crate::Team`] of the given [`crate::Galaxy`] was updated.
    TeamUpdated {
        galaxy: i32,
        team: u8,
    },
    /// The [`crate::Ship`] of the given [`crate::Galaxy`] was updated.
    ShipUpdated {
        galaxy: i32,
        ship: u8,
    },
    /// The [`crate::Upgrade`] of the given [`crate::Ship`] in the given [`crate::Galaxy`] was
    /// upated.
    UpgradeUpdated {
        galaxy: i32,
        ship: u8,
        upgrade: u8,
    },
    /// The [`crate::Player`] of the given [`crate::Galaxy`] was updated.
    PlayerUpdated {
        galaxy: i32,
        player: u8,
    },
}
