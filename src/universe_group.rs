use crate::game_mode::GameMode;
use crate::network::connection::{Connection, ConnectionEvent, OpenError};
use crate::network::connection_handle::SendQueryError;
use crate::network::query::{QueryCommand, QueryError, QueryResponse};
use crate::network::ServerEvent;
use crate::players::Player;
use crate::team::Team;
use tokio::sync::mpsc;

pub struct UniverseGroup {
    // players[0-63] are real players, players[64] is a substitute, if the server treats us as
    // non-player, like a spectator or admin.
    players: [Option<Player>; 65],
    player: usize,
    pub name: String,
    pub description: String,
    pub mode: GameMode,
    pub max_players: u32,
    pub max_ships_per_player: u32,
    pub max_ships_per_team: u32,
    pub max_base_per_player: u32,
    pub max_base_per_team: u32,
    pub spectators: bool,
    pub teams: Vec<Team>,
    receiver: mpsc::UnboundedReceiver<ConnectionEvent>,
}

impl UniverseGroup {
    pub async fn join(name: &str, api_key: &str) -> Result<UniverseGroup, JoinError> {
        let (handle, mut receiver) = Connection::connect_to(
            &format!("www.flattiverse.com/api/universes/{name}.ws"),
            api_key,
        )
        .await?
        .spawn();

        let response = handle.send_query(QueryCommand::WhoAmI)?.await??;
        let player_index = match response.get_integer() {
            Some(value) => value,
            None => {
                return Err(JoinError::InvalidResponse(QueryCommand::WhoAmI, response));
            }
        };

        let event = receiver.recv().await.ok_or(JoinError::ConnectionClosed)?;
        let mut this = Self {
            players: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None,
            ],
            player: player_index as usize,
            name: name.to_string(),
            description: String::default(),
            mode: GameMode::Mission,
            max_players: 0,
            max_ships_per_player: 0,
            max_ships_per_team: 0,
            max_base_per_player: 0,
            max_base_per_team: 0,
            spectators: false,
            teams: Vec::default(),
            receiver,
        };

        if let ConnectionEvent::ServerEvent(ServerEvent::UniverseGroupInfo(info)) = event {
            info.update(&mut this);
            Ok(this)
        } else {
            Err(JoinError::InvalidInitializationEvent(event))
        }
    }

    /// The connected player.
    pub fn player(&self) -> &Player {
        self.players[self.player].as_ref().unwrap()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JoinError {
    #[error("Failed to open the connection: {0}")]
    OpenError(#[from] OpenError),
    #[error("Failed to send required initialization queries: {0}")]
    SendQueryError(#[from] SendQueryError),
    #[error("Initialization failed: {0}")]
    QueryError(#[from] QueryError),
    #[error("Unexpected query response for {0:?}: {1:?}")]
    InvalidResponse(QueryCommand, QueryResponse),
    #[error("Unexpected event for initialization: {0:?}")]
    InvalidInitializationEvent(ConnectionEvent),
    #[error("Connection closed unexpectedly")]
    ConnectionClosed,
}
