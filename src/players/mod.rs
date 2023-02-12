use crate::error::GameError;
use crate::network::connection_handle::{ConnectionHandle, SendQueryError};
use crate::network::query::{QueryCommand, QueryResult};
use crate::team::TeamId;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::ops::Deref;
use std::sync::Weak;
use std::time::Duration;

/// States the kind of player.
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    #[serde(skip, default)]
    pub(crate) connection: Weak<ConnectionHandle>,
    /// The internal ID of the player.
    pub id: PlayerId,
    /// The name of the player.
    pub name: String,
    /// The kind of the player.
    #[serde(rename = "playerKind")]
    pub kind: PlayerKind,
    /// Whether the player is an admin.
    pub admin: bool,
    /// The team the player is in.
    pub team: TeamId,
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

impl Debug for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("kind", &self.kind)
            .field("admin", &self.admin)
            .field("team", &self.team)
            .field("pvp_score", &self.pvp_score)
            .field("rank", &self.rank)
            .field("kills", &self.kills)
            .field("deaths", &self.deaths)
            .field("collisions", &self.collisions)
            .field("ping", &self.ping)
            .finish_non_exhaustive()
    }
}

impl Player {
    fn connection(&self) -> Result<impl Deref<Target = ConnectionHandle>, GameError> {
        if let Some(connection) = self.connection.upgrade() {
            Ok(connection)
        } else {
            Err(GameError::SendQueryError(SendQueryError::ConnectionGone))
        }
    }

    /// The ping of the player.
    #[inline]
    pub fn ping(&self) -> Option<Duration> {
        self.ping.map(|ping| Duration::from_millis(u64::from(ping)))
    }

    pub async fn chat(
        &self,
        message: impl Into<String>,
    ) -> Result<impl Future<Output = QueryResult>, GameError> {
        let message = GameError::checked_message(message.into())?;
        Ok(self
            .connection()?
            .send_query(QueryCommand::ChatPlayer {
                player: self.id,
                message,
            })
            .await?)
    }
}
