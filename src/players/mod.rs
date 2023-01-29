use serde_derive::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};
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

/// Specifies a player that is currently connected to the [`crate::universe_group::UniverseGroup`].
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    /// The internal ID of the player.
    pub id: i32,
    /// The name of the player.
    pub name: String,
    /// The rank of the player.
    pub rank: u32,
    /// All-Time-Kills of the player.
    pub kills: u64,
    /// All-Time-Deaths of the player.
    pub deaths: u64,
    pub(crate) ping: AtomicU32,
}

impl Player {
    /// The ping of the player.
    pub fn ping(&self) -> Duration {
        Duration::from_millis(u64::from(self.ping.load(Ordering::Relaxed)))
    }
}
