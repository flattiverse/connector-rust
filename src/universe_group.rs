use crate::controllable::{Controllable, ControllableId};
use crate::error::GameError;
use crate::events::added_unit_event::AddedUnitEvent;
use crate::events::removed_unit_event::RemovedUnitEvent;
use crate::events::tick_processed_event::TickProcessedEvent;
use crate::events::{ApplicableEvent, FailureEvent};
use crate::game_mode::GameMode;
use crate::network::connection::{Connection, ConnectionEvent, OpenError};
use crate::network::connection_handle::{ConnectionHandle, SendQueryError};
use crate::network::query::{QueryCommand, QueryError, QueryResponse};
use crate::network::ServerEvent;
use crate::players::{Player, PlayerId};
use crate::team::Team;
use crate::units::player_unit::PlayerUnitSystems;
use crate::units::player_unit_system::PlayerUnitSystem;
use crate::units::player_unit_system_kind::PlayerUnitSystemKind;
use crate::units::player_unit_system_upgradepath::PlayerUnitSystemUpgradePath;
use crate::universe::Universe;
use crate::vector::Vector;
use std::collections::HashMap;
use std::future::Future;
use std::ops::Index;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;

pub struct UniverseGroup {
    pub(crate) connection: Arc<ConnectionHandle>,
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
    pub(crate) universes: [Option<Universe>; 64],
    pub(crate) controllables: [Option<Controllable>; 32],
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

        let response = handle.send_query(QueryCommand::WhoAmI).await?.await?;
        let player_index = response
            .get_integer()
            .ok_or_else(|| JoinError::FailedToRetrieveMyOwnPlayerId(response))?;

        Ok(Self {
            connection: handle,
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
            universes: {
                const EMPTY: Option<Universe> = None;
                [EMPTY; 64]
            },
            controllables: Default::default(),
            receiver,
        })
    }

    /// Creates a new ship instantly. Theres is no building process or resources gathering involved.
    /// However, the number of ships that can be registered in this manner may be limited by the
    /// rules of the [`UniverseGroup`] (see [`UniverseGroup`].`register_ship_limit` for example).
    ///
    /// This will create a **dead** ship. To bring it to life, you need to call
    /// [`Controllable::r#continue`] on the ship. Typically, you would call
    /// [`UniverseGroup::new_ship`] followed by [`Controllable::r#continue`].
    pub async fn new_ship(
        &mut self,
        name: impl Into<String>,
    ) -> Result<impl Future<Output = Result<ControllableId, QueryError>>, GameError> {
        // need to hold it the whole time
        let number_of_controllables = self.controllables.iter().flatten().count() as u32;

        if number_of_controllables >= self.register_ship_limit {
            return Err(GameError::ExceededNonBuiltUnits);
        } else if number_of_controllables >= self.max_ships_per_player {
            return Err(GameError::ExceededShipsPerPlayer);
        }

        let name = GameError::checked_name(name.into())?;
        let free_id = self
            .controllables
            .iter()
            .enumerate()
            .find_map(|(index, controllable)| {
                if controllable.is_none() {
                    Some(ControllableId(index))
                } else {
                    None
                }
            })
            .ok_or(GameError::ExceededShipsPerPlayer)?;

        let query = self
            .connection
            .send_query(QueryCommand::NewControllable {
                controllable: free_id,
                name: name.clone(),
            })
            .await?;

        self.controllables[free_id.0] = Some(Controllable {
            connection: Arc::clone(&self.connection),
            name,
            id: free_id,
            radius: 0.0,
            position: Vector::default(),
            movement: Vector::default(),
            direction: 0.0,
            team: None,
            gravity: 0.0,
            energy_output: 0.0,
            alive: false,
            turn_rate: 0.0,
            systems: self.default_player_unit_systems(),
        });

        Ok({
            async move {
                match query.await {
                    Ok(response) => {
                        debug!("NewShip response {response:?}");
                        Ok(free_id)
                    }
                    Err(e) => {
                        // TODO well well well...
                        Err(e)
                    }
                }
            }
        })
    }

    fn default_player_unit_systems(&self) -> PlayerUnitSystems {
        PlayerUnitSystems {
            hull: PlayerUnitSystem {
                level: 0,
                value: None,
                kind: PlayerUnitSystemKind::Hull,
                system: PlayerUnitSystemUpgradePath {
                    required_component: None,
                    kind: PlayerUnitSystemKind::Hull,
                    level: 0,
                    energy: 0.0,
                    particles: 0.0,
                    iron: 0.0,
                    carbon: 0.0,
                    silicon: 0.0,
                    platinum: 0.0,
                    gold: 0.0,
                    time: 0,
                    value0: 0.0,
                    value1: 0.0,
                    value2: 0.0,
                    area_increase: 0.0,
                    weight_increase: 0.0,
                },
            },
        }
    }

    /// The connected player.
    #[inline]
    pub fn player(&self) -> &Player {
        self.players[self.player].as_ref().unwrap()
    }

    /// Get access to your [`Controllable`] by its unique [`ControllableId`]
    #[inline]
    pub fn get_controllable(&self, id: ControllableId) -> Option<&Controllable> {
        self.controllables.get(id.0).and_then(|c| c.as_ref())
    }

    /// Get access to your [`Controllable`] by name
    #[inline]
    pub fn get_controllable_by_name(&self, name: &str) -> Option<&Controllable> {
        self.controllables
            .iter()
            .find_map(|c| c.as_ref().filter(|c| c.name == name))
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
            ServerEvent::Raw(raw) => Some(Ok(FlattiverseEvent::Raw(raw))),
            ServerEvent::Failure(failure) => Some(Ok(FlattiverseEvent::Failure(failure))),
            ServerEvent::PlayerFullUpdate(update) => {
                let id = update.player.id;
                update.apply(self);
                Some(Ok(FlattiverseEvent::PlayerFullUpdate(id)))
            }
            ServerEvent::PlayerPartialUpdate(update) => {
                let id = update.id;
                update.apply(self);
                Some(Ok(FlattiverseEvent::PlayerPartialUpdate(id)))
            }
            ServerEvent::PlayerRemoved(update) => {
                let id = update.id;
                update.apply(self);
                Some(Ok(FlattiverseEvent::PlayerRemoved(id)))
            }
            ServerEvent::UnitRemoved(event) => Some(Ok(FlattiverseEvent::UnitRemoved(event))),
            ServerEvent::UnitAdded(event) => Some(Ok(FlattiverseEvent::UnitAdded(event))),
            ServerEvent::TickProcessed(event) => Some(Ok(FlattiverseEvent::TickProcessed(event))),
            ServerEvent::UniverseGroupInfo(info) => {
                info.apply(self);
                Some(Ok(FlattiverseEvent::UniverseGroupInfo))
            }
        }
    }
}

impl Index<ControllableId> for UniverseGroup {
    type Output = Controllable;

    #[inline]
    fn index(&self, index: ControllableId) -> &Self::Output {
        self.controllables[index.0].as_ref().unwrap()
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
    /// This event notifies about the meta information a [`UniverseGroup`] has, like name,
    /// description, teams, rules...
    /// You actually don't need to parse this event because it's also parsed by the connector and
    /// the results are presented in fields on the [`UniverseGroup`].
    UniverseGroupInfo,
    /// This event updates all information about a [`Player`].
    PlayerFullUpdate(PlayerId),
    /// This event contains only mutable information about a [`Player`].
    PlayerPartialUpdate(PlayerId),
    /// This event informs of the disconnect of a player from the [`UniverseGroup`].
    PlayerRemoved(PlayerId),
    /// This event informs of the removal of a unit from the [`UniverseGroup`].
    UnitRemoved(RemovedUnitEvent),
    /// This event informs of the addition of a unit to the [`UniverseGroup`].
    UnitAdded(AddedUnitEvent),
    /// This event informs of the completion of a tick in the [`UniverseGroup`].
    TickProcessed(TickProcessedEvent),
}
