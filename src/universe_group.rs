use crate::controllable::{Controllable, ControllableId, ControllableState};
use crate::error::GameError;
use crate::events::added_unit_event::AddedUnitEvent;
use crate::events::chat_multicast_event::ChatMulticastEvent;
use crate::events::chat_teamcast_event::ChatTeamcastEvent;
use crate::events::chat_unicast_event::ChatUnicastEvent;
use crate::events::death_controllable_event::DeathControllableEvent;
use crate::events::removed_unit_event::RemovedUnitEvent;
use crate::events::tick_processed_event::TickProcessedEvent;
use crate::events::updated_unit_event::UpdatedUnitEvent;
use crate::events::{ApplicableEvent, Completable, FailureEvent};
use crate::game_mode::GameMode;
use crate::network::connection::{Connection, ConnectionEvent, OpenError};
use crate::network::connection_handle::{ConnectionHandle, SendQueryError};
use crate::network::query::{QueryCommand, QueryError, QueryResponse};
use crate::network::ServerEvent;
use crate::players::{Player, PlayerId};
use crate::team::{Team, TeamId};
use crate::units::player_unit_system_identifier::PlayerUnitSystemIdentifier;
use crate::units::player_unit_system_kind::PlayerUnitSystemKind;
use crate::units::player_unit_system_upgradepath::PlayerUnitSystemUpgradePath;
use crate::units::unit_kind::UnitKind;
use crate::universe::{Universe, UniverseId};
use std::collections::HashMap;
use std::future::Future;
use std::ops::Index;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Handle;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::{mpsc, Mutex};

pub struct UniverseGroup {
    pub(crate) connection: Arc<ConnectionHandle>,
    // players[0-63] are real players, players[64] is a substitute, if the server treats us as
    // non-player, like a spectator or admin.
    pub(crate) players: [Option<Player>; 65],
    pub(crate) player: PlayerId,
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
    pub(crate) controllables: [Option<Arc<Controllable>>; 32],
    pub(crate) systems: HashMap<PlayerUnitSystemIdentifier, PlayerUnitSystemUpgradePath>,
    receiver: mpsc::UnboundedReceiver<ConnectionEvent>,
}

impl UniverseGroup {
    pub const BASE_URL_JOIN_UNIVERSE_GROUP: &'static str =
        "wss://www.flattiverse.com/api/universes/";

    #[inline]
    pub async fn join(
        name: &str,
        api_key: &str,
        team: impl Into<Option<&str>>,
    ) -> Result<UniverseGroup, JoinError> {
        Self::join_url(
            &format!("{}{name}.ws", Self::BASE_URL_JOIN_UNIVERSE_GROUP),
            api_key,
            team,
        )
        .await
    }

    pub async fn join_url(
        url: &str,
        api_key: &str,
        team: impl Into<Option<&str>>,
    ) -> Result<UniverseGroup, JoinError> {
        let (handle, receiver) = Connection::connect_to(url, api_key, team.into())
            .await?
            .spawn(Handle::current());

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
            player: PlayerId(player_index as usize),
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
            systems: Default::default(),
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
    pub fn new_ship(
        &mut self,
        name: impl Into<String>,
    ) -> impl Future<Output = Result<ControllableId, GameError>> {
        // need to hold it the whole time
        let free_id = (|| {
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

            self.controllables[free_id.0] = Some(Arc::new(Controllable {
                connection: Arc::clone(&self.connection),
                name: name.clone(),
                id: free_id,
                direction: 0.0,
                team: None,
                alive: false,
                state: Arc::new(Mutex::new(ControllableState {
                    movement: Default::default(),
                    position: Default::default(),
                    radius: 0.0,
                    gravity: 0.0,
                    energy_output: 0.0,
                    turn_rate: 0.0,
                    requested_scan_direction: 0.0,
                    requested_scan_width: 0.0,
                    requested_scan_range: 0.0,
                    scan_direction: 0.0,
                    scan_width: 0.0,
                    scan_range: 0.0,
                    systems: Default::default(),
                })),
            }));

            Ok((name, free_id, Arc::clone(&self.connection)))
        })();

        async move {
            let (name, free_id, connection) = free_id?;
            let query = connection
                .send_query(QueryCommand::NewControllable {
                    controllable: free_id,
                    name,
                })
                .await?;

            match query.await {
                Ok(response) => {
                    debug!("NewShip response {response:?}");
                    Ok(free_id)
                }
                Err(e) => {
                    // TODO well well well... shit, actually need to clean up ...
                    Err(e.into())
                }
            }
        }
    }

    pub fn chat(
        &self,
        message: impl Into<String>,
    ) -> impl Future<Output = Result<QueryResponse, GameError>> + 'static {
        let connection = Arc::clone(&self.connection);
        let message = GameError::checked_message(message.into());
        async move {
            Ok(connection
                .send_query(QueryCommand::ChatUniverseGroup { message: message? })
                .await?
                .await?)
        }
    }

    pub(crate) fn get_player_unit_system_upgrade_path(
        &self,
        system: PlayerUnitSystemKind,
        level: impl Into<Option<u32>>,
    ) -> Option<&PlayerUnitSystemUpgradePath> {
        self.systems.get(&PlayerUnitSystemIdentifier {
            system,
            level: level.into(),
        })
    }

    /// You yourself as [`PlayerId'].
    #[inline]
    pub fn player_id(&self) -> PlayerId {
        self.player
    }

    /// You yourself as [`Player'] instance.
    #[inline]
    pub fn player(&self) -> &Player {
        self.players
            .get(self.player.0)
            .and_then(|p| p.as_ref())
            .expect("Players not initialized yet")
    }

    /// Iterate over all known [`Player`]s
    #[inline]
    pub fn iter_players(&self) -> impl Iterator<Item = &Player> + '_ {
        self.players.iter().flatten()
    }

    /// Ge a [`Player`] by its unique [`PlayerId`].
    #[inline]
    pub fn get_player(&self, id: PlayerId) -> Option<&Player> {
        self.players.get(id.0).and_then(|p| p.as_ref())
    }

    /// Ge a [`Player`] by its unique name.
    #[inline]
    pub fn get_player_by_name(&self, name: &str) -> Option<&Player> {
        self.players
            .iter()
            .find_map(|p| p.as_ref().filter(|p| p.name == name))
    }

    /// Iterate over all known [`Team`]s
    #[inline]
    pub fn iter_teams(&self) -> impl Iterator<Item = &Team> + '_ {
        self.teams.iter().flatten()
    }

    /// Ge a [`Team`] by its unique [`TeamId`].
    #[inline]
    pub fn get_team(&self, id: TeamId) -> Option<&Team> {
        self.teams.get(id.0).and_then(|p| p.as_ref())
    }

    /// Ge a [`Team`] by its unique name.
    #[inline]
    pub fn get_team_by_name(&self, name: &str) -> Option<&Team> {
        self.teams
            .iter()
            .find_map(|t| t.as_ref().filter(|t| t.name == name))
    }

    /// Iterate over all your [`Controllable`]s
    #[inline]
    pub fn iter_controllables(&self) -> impl Iterator<Item = &Arc<Controllable>> + '_ {
        self.controllables.iter().flatten()
    }

    /// Get access to your [`Controllable`] by its unique [`ControllableId`].
    #[inline]
    pub fn get_controllable(&self, id: ControllableId) -> Option<&Arc<Controllable>> {
        self.controllables.get(id.0).and_then(|c| c.as_ref())
    }

    /// Get access to your [`Controllable`] by its unique name.
    #[inline]
    pub fn get_controllable_by_name(&self, name: &str) -> Option<&Arc<Controllable>> {
        self.controllables
            .iter()
            .find_map(|c| c.as_ref().filter(|c| c.name == name))
    }

    /// Waits for the next [`FlattiverseEvent`], potentially waiting forever.
    pub async fn next_event(&mut self) -> Result<FlattiverseEvent, EventError> {
        loop {
            let connection_event = self.receiver.recv().await.ok_or(EventError::Disconnected)?;
            if let Some(result) = self.on_connection_event(connection_event).await {
                return result;
            }
        }
    }

    /// Polls the next [`FlattiverseEvent`], potentially returning `None` - but immediately.
    pub async fn poll_next_event(&mut self) -> Option<Result<FlattiverseEvent, EventError>> {
        loop {
            match self.receiver.try_recv() {
                Err(TryRecvError::Empty) => return None,
                Err(TryRecvError::Disconnected) => return Some(Err(EventError::Disconnected)),
                Ok(event) => {
                    if let Some(result) = self.on_connection_event(event).await {
                        return Some(result);
                    }
                }
            }
        }
    }

    async fn on_connection_event(
        &mut self,
        event: ConnectionEvent,
    ) -> Option<Result<FlattiverseEvent, EventError>> {
        match event {
            ConnectionEvent::PingMeasured(duration) => {
                Some(Ok(FlattiverseEvent::PingMeasured(duration)))
            }
            ConnectionEvent::ServerEvent(event) => self.on_server_event(event).await,
        }
    }

    async fn on_server_event(
        &mut self,
        event: ServerEvent,
    ) -> Option<Result<FlattiverseEvent, EventError>> {
        debug!("Applying ServerEvent {event:?}");
        match event {
            ServerEvent::Raw(raw) => Some(Ok(FlattiverseEvent::Raw(raw))),
            ServerEvent::Failure(failure) => Some(Ok(FlattiverseEvent::Failure(failure))),
            ServerEvent::ChatMulticast(event) => Some(Ok(FlattiverseEvent::ChatMulticast(event))),
            ServerEvent::ChatTeamcastEvent(event) => {
                Some(Ok(FlattiverseEvent::ChatTeamcastEvent(event)))
            }
            ServerEvent::ChatUnicastEvent(event) => {
                Some(Ok(FlattiverseEvent::ChatUnicastEvent(event)))
            }
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
            ServerEvent::UnitAdded(mut event) => {
                event.complete(self);
                if let UnitKind::PlayerUnit(player_unit) = &event.unit.kind {
                    if self.player == player_unit.player
                        && self
                            .controllables
                            .iter()
                            .flatten()
                            .any(|c| c.id == player_unit.controllable)
                    {
                        self.controllables[player_unit.controllable.0]
                            .as_ref()
                            .unwrap()
                            .update_systems(player_unit)
                            .await;
                    }
                }
                Some(Ok(FlattiverseEvent::UnitAdded(event)))
            }
            ServerEvent::UnitUpdated(mut event) => {
                event.complete(self);
                if let UnitKind::PlayerUnit(player_unit) = &event.unit.kind {
                    if self.player == player_unit.player
                        && self
                            .controllables
                            .iter()
                            .flatten()
                            .any(|c| c.id == player_unit.controllable)
                    {
                        self.controllables[player_unit.controllable.0]
                            .as_ref()
                            .unwrap()
                            .update_systems(player_unit)
                            .await;
                    }
                }
                Some(Ok(FlattiverseEvent::UnitUpdated(event)))
            }
            ServerEvent::UnitRemoved(event) => {
                if let (Some(player), Some(controllable)) = (event.player, event.controllable) {
                    if self.player == player
                        && self
                            .controllables
                            .iter()
                            .flatten()
                            .any(|c| c.id == controllable)
                    {
                        self.controllables[controllable.0]
                            .as_ref()
                            .unwrap()
                            .die()
                            .await;
                    }
                }
                Some(Ok(FlattiverseEvent::UnitRemoved(event)))
            }
            ServerEvent::TickProcessed(event) => Some(Ok(FlattiverseEvent::TickProcessed(event))),
            ServerEvent::UniverseGroupInfo(info) => {
                info.apply(self);
                Some(Ok(FlattiverseEvent::UniverseGroupInfo))
            }
            ServerEvent::ControllableUpdated(event) => {
                let id = event.controllable;
                event.apply(self).await;
                Some(Ok(FlattiverseEvent::ControllableUpdated(id)))
            }
            ServerEvent::ControllableDeath(event) => {
                Some(Ok(FlattiverseEvent::ControllableDied(event)))
            }
        }
    }
}

impl Index<PlayerId> for UniverseGroup {
    type Output = Player;

    #[inline]
    fn index(&self, index: PlayerId) -> &Self::Output {
        self.players[index.0].as_ref().unwrap()
    }
}

impl Index<TeamId> for UniverseGroup {
    type Output = Team;

    #[inline]
    fn index(&self, index: TeamId) -> &Self::Output {
        self.teams[index.0].as_ref().unwrap()
    }
}

impl Index<ControllableId> for UniverseGroup {
    type Output = Arc<Controllable>;

    #[inline]
    fn index(&self, index: ControllableId) -> &Self::Output {
        self.controllables[index.0].as_ref().unwrap()
    }
}

impl Index<UniverseId> for UniverseGroup {
    type Output = Universe;

    #[inline]
    fn index(&self, index: UniverseId) -> &Self::Output {
        self.universes[index.0].as_ref().unwrap()
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

#[derive(Debug, Clone)]
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
    /// This event informs about a chat-message to everyone.
    ChatMulticast(ChatMulticastEvent),
    /// This event informs about a chat-message to a team.
    ChatTeamcastEvent(ChatTeamcastEvent),
    /// This event informs about a chat-message to a player.
    ChatUnicastEvent(ChatUnicastEvent),
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
    /// This event informs of the update of a unit in the [`UniverseGroup`]
    UnitUpdated(UpdatedUnitEvent),
    /// This event informs of the completion of a tick in the [`UniverseGroup`].
    TickProcessed(TickProcessedEvent),
    /// This event informs of the update of a controllable in the [`UniverseGroup`].
    ControllableUpdated(ControllableId),
    /// This event informs of the untimely demise of a controllable in the [`UniverseGroup`].
    ControllableDied(DeathControllableEvent),
}
