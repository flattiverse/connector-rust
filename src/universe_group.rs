use crate::events::added_unit_event::AddedUnitEvent;
use crate::events::tick_processed_event::TickProcessedEvent;
use crate::events::{ApplicableEvent, FailureEvent};
use crate::game_mode::GameMode;
use crate::network::connection::{Connection, ConnectionEvent, OpenError};
use crate::network::connection_handle::SendQueryError;
use crate::network::query::{QueryCommand, QueryError, QueryResponse};
use crate::network::ServerEvent;
use crate::players::Player;
use crate::team::Team;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;

pub struct UniverseGroup {
    // players[0-63] are real players, players[64] is a substitute, if the server treats us as
    // non-player, like a spectator or admin.
    pub(crate) players: [Option<Player>; 65],
    pub(crate) player: usize,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) mode: GameMode,
    pub(crate) max_players: u32,
    pub(crate) max_ships_per_player: u32,
    pub(crate) max_ships_per_team: u32,
    pub(crate) max_bases_per_player: u32,
    pub(crate) max_bases_per_team: u32,
    pub(crate) spectators: bool,
    pub(crate) register_ship_limit: u32,
    pub(crate) teams: [Option<Team>; 16],
    // universes: [Option<Universe>; 64],
    // controllables: [Option<Controllable>; 32],
    // systems: HashMap<PlayerUnitSystemIdentifier, PlayerUnitSystemUpgradepath>,
    receiver: mpsc::UnboundedReceiver<ConnectionEvent>,
}

impl UniverseGroup {
    #[inline]
    pub async fn join(name: &str, api_key: &str) -> Result<UniverseGroup, JoinError> {
        Self::join_url(
            &format!("www.flattiverse.com/api/universes/{name}.ws"),
            api_key,
        )
        .await
    }

    pub async fn join_url(url: &str, api_key: &str) -> Result<UniverseGroup, JoinError> {
        let (handle, receiver) = Connection::connect_to(url, api_key).await?.spawn();

        let response = handle.send_query(QueryCommand::WhoAmI).await?.await??;
        let player_index = response
            .get_integer()
            .ok_or_else(|| JoinError::FailedToRetrieveMyOwnPlayerId(response))?;

        Ok(Self {
            players: {
                const EMPTY: Option<Player> = None;
                [EMPTY; 65]
            },
            player: player_index as usize,
            name: "Unknown".to_string(),
            description: "Unknown".to_string(),
            mode: GameMode::Mission,
            max_players: 0,
            max_ships_per_player: 0,
            max_ships_per_team: 0,
            max_bases_per_player: 0,
            max_bases_per_team: 0,
            spectators: false,
            register_ship_limit: 0,
            teams: Default::default(),
            receiver,
        })
    }

    /// The connected player.
    pub fn player(&self) -> &Player {
        self.players[self.player].as_ref().unwrap()
    }

    pub fn poll_next_event(&mut self) -> Option<Result<FlattiverseEvent, EventError>> {
        loop {
            match self.receiver.try_recv() {
                Err(TryRecvError::Empty) => return None,
                Err(TryRecvError::Disconnected) => return Some(Err(EventError::Disconnected)),
                Ok(ConnectionEvent::PingMeasured(duration)) => {
                    return Some(Ok(FlattiverseEvent::PingMeasured(duration)));
                }
                Ok(ConnectionEvent::ServerEvent(event)) => {
                    if let Some(update) = self.on_server_event(event) {
                        return Some(update);
                    }
                }
            }
        }
    }

    fn on_server_event(
        &mut self,
        event: ServerEvent,
    ) -> Option<Result<FlattiverseEvent, EventError>> {
        debug!("Applying ServerEvent {event:?}");
        match event {
            ServerEvent::Raw(raw) => return Some(Ok(FlattiverseEvent::Raw(raw))),
            ServerEvent::Failure(failure) => return Some(Ok(FlattiverseEvent::Failure(failure))),
            ServerEvent::PlayerFullUpdate(update) => update.apply(self),
            ServerEvent::UnitAdded(event) => return Some(Ok(FlattiverseEvent::UnitAdded(event))),
            ServerEvent::TickProcessed(event) => {
                return Some(Ok(FlattiverseEvent::TickProcessed(event)))
            }
            ServerEvent::UniverseGroupInfo(info) => info.apply(self),
        }
        None
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
    #[error("Unexpected query response when retrieving own player id: {0:?}")]
    FailedToRetrieveMyOwnPlayerId(QueryResponse),
    #[error("Unexpected event for initialization: {0:?}")]
    InvalidInitializationEvent(ConnectionEvent),
    #[error("Connection closed unexpectedly")]
    ConnectionClosed,
}

#[derive(Debug, thiserror::Error)]
pub enum EventError {
    #[error("The connection is no more")]
    Disconnected,
}

#[derive(Debug)]
pub enum FlattiverseEvent {
    /// The result of a periodic ping measurement.
    PingMeasured(Duration),
    /// A Fallback event for debugging purposes, if the event sent from the server is unknown to the
    /// connector.
    Raw(HashMap<String, serde_json::Value>),
    /// This event indicates some critical out-of-game failure like a problem with the
    /// data-transport, etc.. Consider upgrading the connector if this happens and it
    /// is not due to a lost connection.
    Failure(FailureEvent),
    UnitAdded(AddedUnitEvent),
    TickProcessed(TickProcessedEvent),
}
