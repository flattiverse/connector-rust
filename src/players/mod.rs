use serde_derive::{Deserialize, Serialize};
use std::time::Duration;

/// States the kind of player.
#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerKind {
    /// The player is a player participating in the game and also blocking a player slot.
    Player,
    /// The player is a spectator which used the Api-Key
    /// `0x0000000000000000000000000000000000000000000000000000000000000000`. This is only possible
    /// in universe groups which allow spectators.
    Spectator,
    /// The player is an admin. Admins can't participate in the game but they can alter game-states
    /// via admin commands.
    Admin,
}

#[repr(transparent)]
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct PlayerId(pub(crate) usize);

/// Specifies a player that is currently connected to the [`crate::universe_group::UniverseGroup`].
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    /// The internal ID of the player.
    pub id: PlayerId,
    /// The name of the player.
    pub name: String,
    /// The kind of the player.
    #[serde(rename = "playerKind")]
    pub kind: PlayerKind,
    /// Whether the player is an admin.
    pub admin: bool,
    /// The ELO ranking of the player.
    #[serde(rename = "pvpScore")]
    pub pvp_score: f64,
    /// The rank of the player.
    pub rank: i32,
    /// All-Time-Kills of the player.
    pub kills: u64,
    /// All-Time-Deaths of the player.
    pub deaths: u64,
    /// All-Time-Collisions of the player.
    pub collisions: u64,
    pub(crate) ping: Option<u32>,
}

impl Player {
    /// The ping of the player.
    #[inline]
    pub fn ping(&self) -> Option<Duration> {
        self.ping.map(|ping| Duration::from_millis(u64::from(ping)))
    }
}
