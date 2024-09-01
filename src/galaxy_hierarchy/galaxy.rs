use crate::galaxy_hierarchy::{
    Cluster, ClusterId, GameMode, Player, PlayerId, PlayerKind, Team, TeamId, UniversalArcHolder,
};
use crate::network::{ConnectError, ConnectionHandle};
use crate::runtime::Atomic;
use crate::{FlattiverseEvent, FlattiverseEventKind, GameError, GameErrorKind};
use async_channel::{Receiver, TryRecvError};
use std::sync::{Arc, RwLock};
use tracing::instrument;

type EventResult = Result<Option<FlattiverseEvent>, GameError>;

macro_rules! event_result {
    ($kind:ident $content:tt) => {
        Ok(Some({FlattiverseEventKind::$kind $content}.into()))
    };
}

#[derive(Debug)]
pub struct Galaxy {
    name: RwLock<String>,

    game_mode: Atomic<GameMode>,
    description: RwLock<String>,

    max_players: Atomic<u8>,
    max_spectators: Atomic<u16>,

    galaxy_max_total_ships: Atomic<u16>,
    galaxy_max_classic_ships: Atomic<u16>,
    galaxy_max_new_ships: Atomic<u16>,
    galaxy_max_bases: Atomic<u16>,

    team_max_total_ships: Atomic<u16>,
    team_max_classic_ships: Atomic<u16>,
    team_max_new_ships: Atomic<u16>,
    team_max_bases: Atomic<u16>,

    player_max_total_ships: Atomic<u8>,
    player_max_classic_ships: Atomic<u8>,
    player_max_new_ships: Atomic<u8>,
    player_max_bases: Atomic<u8>,

    maintenance: Atomic<bool>,
    active: Atomic<bool>,

    teams: UniversalArcHolder<TeamId, Team>,
    clusters: UniversalArcHolder<ClusterId, Cluster>,
    players: UniversalArcHolder<PlayerId, Player>,

    connection: ConnectionHandle,
    events: Receiver<FlattiverseEvent>,

    player: Atomic<PlayerId>,
}

impl Galaxy {
    pub const AUTH_ANONYMOUS: &'static str =
        "0000000000000000000000000000000000000000000000000000000000000000";
    pub const URI_BASE: &'static str = "www.flattiverse.com";

    #[cfg(not(feature = "dev-environment"))]
    pub const URI_GALAXY_DEFAULT: &'static str = "wss://www.flattiverse.com/game/galaxies/0";

    #[cfg(feature = "dev-environment")]
    pub const URI_GALAXY_DEFAULT: &'static str = "ws://localhost:5000";

    #[inline]
    pub async fn connect(
        galaxy: u16,
        auth: impl Into<Option<&str>>,
        team: impl Into<Option<&str>>,
    ) -> Result<Arc<Self>, GameError> {
        #[cfg(not(feature = "dev-environment"))]
        {
            Self::connect_to(
                &format!(
                    "{}{}",
                    &Self::URI_GALAXY_DEFAULT[..Self::URI_GALAXY_DEFAULT.len() - 1],
                    galaxy
                ),
                auth,
                team,
            )
            .await
        }
        #[cfg(feature = "dev-environment")]
        {
            Self::connect_to(
                &format!(
                    "{}{}",
                    &Self::URI_GALAXY_DEFAULT[..Self::URI_GALAXY_DEFAULT.len() - 4],
                    5000 + galaxy
                ),
                auth,
                team,
            )
            .await
        }
    }

    #[instrument(level = "trace", skip(auth, team))]
    pub async fn connect_to(
        uri: &str,
        auth: impl Into<Option<&str>>,
        team: impl Into<Option<&str>>,
    ) -> Result<Arc<Self>, GameError> {
        let mut session = None;
        let this = crate::network::connect(
            uri,
            auth.into().unwrap_or(Self::AUTH_ANONYMOUS),
            team.into(),
            |handle, event_receiver| {
                session = Some(
                    handle
                        .sessions
                        .get()
                        .expect("Failed to get initial session"),
                );
                let this = Arc::new(Self {
                    name: RwLock::default(),
                    game_mode: Atomic::from(GameMode::Mission),
                    description: RwLock::default(),
                    max_players: Atomic::from(0),
                    max_spectators: Atomic::from(0),
                    galaxy_max_total_ships: Atomic::from(0),
                    galaxy_max_classic_ships: Atomic::from(0),
                    galaxy_max_new_ships: Atomic::from(0),
                    galaxy_max_bases: Atomic::from(0),
                    team_max_total_ships: Atomic::from(0),
                    team_max_classic_ships: Atomic::from(0),
                    team_max_new_ships: Atomic::from(0),
                    team_max_bases: Atomic::from(0),
                    player_max_total_ships: Atomic::from(0),
                    player_max_classic_ships: Atomic::from(0),
                    player_max_new_ships: Atomic::from(0),
                    player_max_bases: Atomic::from(0),
                    maintenance: Atomic::from(false),
                    active: Atomic::from(true),
                    teams: UniversalArcHolder::with_capacity(33),
                    clusters: UniversalArcHolder::with_capacity(64),
                    players: UniversalArcHolder::with_capacity(193),
                    connection: handle,
                    events: event_receiver,
                    player: Atomic::from(PlayerId(0)),
                });

                this.teams.populate(Team::new(
                    Arc::downgrade(&this),
                    TeamId(32),
                    "Spectators",
                    128,
                    128,
                    128,
                ));

                this
            },
        )
        .await
        .map_err(|e| match e {
            ConnectError::GameError(e) => e,
            e => {
                debug!("Cannot connect to the flattiverse server: {e:?}");
                GameError::from(GameErrorKind::CantConnect)
            }
        })?;

        session
            .expect("Failed to get initial session")
            .response()
            .await?
            .read(|reader| {
                this.setup_self(reader.read_byte());
            });

        Ok(this)
    }

    fn setup_self(&self, id: u8) {
        debug_assert!(id < 193, "Id out of bounds.");

        let id = PlayerId(id);
        debug_assert!(self.players.has(id), "{id:?} not setup.");
        self.player.store(id);
    }

    /// Sends a chat message to all players in this [`Galaxy`].
    #[inline]
    pub async fn chat(&self, message: impl AsRef<str>) -> Result<(), GameError> {
        self.connection.chat_galaxy(message).await
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn ping_pong(&self, challenge: u16) -> Result<Option<FlattiverseEvent>, GameError> {
        debug!("Responding to ping with challenge={challenge:#04x}");
        self.connection.respond_to_ping(challenge)?;
        Ok(Some(
            FlattiverseEventKind::RespondedToPingMeasurement { challenge }.into(),
        ))
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn update_galaxy(
        self: &Arc<Self>,
        game_mode: GameMode,
        name: String,
        description: String,
        max_players: u8,
        max_spectators: u16,
        galaxy_max_total_ships: u16,
        galaxy_max_classic_ships: u16,
        galaxy_max_new_ships: u16,
        galaxy_max_bases: u16,
        team_max_total_ships: u16,
        team_max_classic_ships: u16,
        team_max_new_ships: u16,
        team_max_bases: u16,
        player_max_total_ships: u8,
        player_max_classic_ships: u8,
        player_max_new_ships: u8,
        player_max_bases: u8,
    ) -> Result<Option<FlattiverseEvent>, GameError> {
        debug!("Updating galaxy");
        self.game_mode.store(game_mode);
        *self.name.write().unwrap() = name;
        *self.description.write().unwrap() = description;
        self.max_players.store(max_players);
        self.max_spectators.store(max_spectators);
        self.galaxy_max_total_ships.store(galaxy_max_total_ships);
        self.galaxy_max_classic_ships
            .store(galaxy_max_classic_ships);
        self.galaxy_max_new_ships.store(galaxy_max_new_ships);
        self.galaxy_max_bases.store(galaxy_max_bases);
        self.team_max_total_ships.store(team_max_total_ships);
        self.team_max_classic_ships.store(team_max_classic_ships);
        self.team_max_new_ships.store(team_max_new_ships);
        self.team_max_bases.store(team_max_bases);
        self.player_max_total_ships.store(player_max_total_ships);
        self.player_max_classic_ships
            .store(player_max_classic_ships);
        self.player_max_new_ships.store(player_max_new_ships);
        self.player_max_bases.store(player_max_bases);

        event_result!(UpdatedGalaxy {
            galaxy: Arc::clone(self),
        })
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn update_team(
        self: &Arc<Self>,
        id: TeamId,
        red: u8,
        green: u8,
        blue: u8,
        name: String,
    ) -> Result<Option<FlattiverseEvent>, GameError> {
        debug!("Updating team with {id:?}");
        debug_assert!(id.0 < 32, "Invalid {id:?}");
        event_result!(UpdatedTeam {
            team: match self.teams.get_opt(id) {
                Some(team) => {
                    team.update(name, red, green, blue);
                    team
                }
                None =>
                    self.teams
                        .populate(Team::new(Arc::downgrade(self), id, name, red, green, blue)),
            },
        })
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn deactivate_team(
        &self,
        id: TeamId,
    ) -> Result<Option<FlattiverseEvent>, GameError> {
        debug!("Deactivating team with {id:?}");
        debug_assert!(id.0 < 32, "Invalid {id:?}");
        event_result!(DeactivatedTeam {
            team: {
                self.teams.get(id).deactivate();
                self.teams.remove(id)
            },
        })
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn update_cluster(
        &self,
        id: ClusterId,
        name: String,
    ) -> Result<Option<FlattiverseEvent>, GameError> {
        debug!("Updating cluster with {id:?}");
        debug_assert!(id.0 < 64, "Invalid {id:?}");
        event_result!(UpdatedCluster {
            cluster: match self.clusters.get_opt(id) {
                Some(cluster) => {
                    cluster.update(name);
                    cluster
                }
                None => self.clusters.populate(Cluster::new(id, name)),
            },
        })
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn deactivate_cluster(
        &self,
        id: ClusterId,
    ) -> Result<Option<FlattiverseEvent>, GameError> {
        debug!("Deactivating cluster with {id:?}");
        debug_assert!(id.0 < 64, "Invalid {id:?}");
        event_result!(DeactivatedCluster {
            cluster: {
                self.clusters.get(id).deactivate();
                self.clusters.remove(id)
            },
        })
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn create_player(
        self: &Arc<Self>,
        id: PlayerId,
        kind: PlayerKind,
        team: TeamId,
        name: String,
        ping: f32,
    ) -> Result<Option<FlattiverseEvent>, GameError> {
        debug!("Creating player with {id:?}");
        debug_assert!(id.0 < 193, "Invalid {id:?}");
        debug_assert!(self.players.has_not(id), "{id:?} does already exist.");
        debug_assert!(self.teams.has(team), "{team:?} does not exist.");
        event_result!(JoinedPlayer {
            player: self.players.populate(Player::new(
                Arc::downgrade(self),
                id,
                kind,
                self.teams.get(team),
                name,
                ping
            )),
        })
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn update_player(&self, id: PlayerId, ping: f32) -> EventResult {
        debug!("Updating player with {id:?}");
        debug_assert!(id.0 < 193, "Invalid {id:?}");
        debug_assert!(self.players.has(id), "{id:?} does not exist.");
        event_result!(UpdatedPlayer {
            player: {
                let player = self.players.get(id);
                player.update(ping);
                player
            }
        })
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn deactivate_player(&self, id: PlayerId) -> EventResult {
        debug!("Deactivating player with {id:?}");
        debug_assert!(id.0 < 193, "Invalid {id:?}");
        debug_assert!(self.players.has(id), "{id:?} does not exist.");
        event_result!(PartedPlayer {
            player: {
                self.players.get(id).deactivate();
                self.players.remove(id)
            }
        })
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn universe_tick(&self, number: i32) -> EventResult {
        debug!("Universe tick with #{number}");
        event_result!(GalaxyTick { tick: number })
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn chat_galaxy(self: &Arc<Self>, player: PlayerId, message: String) -> EventResult {
        debug!("Received galaxy chat message: {message:?}");
        debug_assert!(self.players.has(player), "{player:?} does not exist.");
        event_result!(GalaxyChat {
            player: self.players.get(player),
            destination: Arc::clone(self),
            message
        })
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn chat_team(self: &Arc<Self>, player: PlayerId, message: String) -> EventResult {
        debug!("Received team chat message: {message:?}");
        debug_assert!(self.players.has(player), "{player:?} does not exist.");
        event_result!(TeamChat {
            player: self.players.get(player),
            destination: self.player(),
            message
        })
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn chat_player(self: &Arc<Self>, player: PlayerId, message: String) -> EventResult {
        debug!("Received player chat message: {message:?}");
        debug_assert!(self.players.has(player), "{player:?} does not exist.");
        event_result!(PlayerChat {
            player: self.players.get(player),
            destination: self.player(),
            message
        })
    }

    /// Yourself.
    pub fn player(&self) -> Arc<Player> {
        self.players.get(self.player.load())
    }

    /// The name of the galaxy.
    pub fn name(&self) -> String {
        self.name.read().unwrap().clone()
    }

    /// The description of the galaxy.
    pub fn description(&self) -> String {
        self.description.read().unwrap().clone()
    }

    /// The game mode in effect of the galaxy.
    pub fn game_mode(&self) -> GameMode {
        self.game_mode.load()
    }

    /// The maximum number of players allowed to connect to the galaxy.
    pub fn max_players(&self) -> u8 {
        self.max_players.load()
    }

    /// The maximum number of spectators allowed to connect to the galaxy. If this value is 0 no spectators are allowed.
    pub fn max_spectators(&self) -> u16 {
        self.max_spectators.load()
    }

    /// The maximum amount of total ships allowed in the galaxy.
    pub fn galaxy_max_total_ships(&self) -> u16 {
        self.galaxy_max_total_ships.load()
    }

    /// The maximum amount of classic style ships allowed in the galaxy.
    pub fn galaxy_max_classic_ships(&self) -> u16 {
        self.galaxy_max_classic_ships.load()
    }

    /// The maximum amount of new style ships allowed in the galaxy.
    pub fn galaxy_max_new_ships(&self) -> u16 {
        self.galaxy_max_new_ships.load()
    }

    /// The maximum amount of bases allowed in the galaxy.
    pub fn galaxy_max_bases(&self) -> u16 {
        self.galaxy_max_bases.load()
    }

    /// The maximum amount of total hips allowed per team in the galaxy.
    pub fn team_max_total_ships(&self) -> u16 {
        self.team_max_total_ships.load()
    }

    /// The maximum amount of classic style ships allowed per team in teh galaxy.
    pub fn team_max_classic_ships(&self) -> u16 {
        self.team_max_classic_ships.load()
    }

    /// The maximum amount of new style ships allowed per team in teh galaxy.
    pub fn team_max_new_ships(&self) -> u16 {
        self.team_max_new_ships.load()
    }

    /// The maximum amount of bases allowed per team in the galaxy.
    pub fn team_max_bases(&self) -> u16 {
        self.team_max_bases.load()
    }

    /// The maximum amount of total ships allowed per player in the galaxy.
    pub fn player_max_total_ships(&self) -> u8 {
        self.player_max_total_ships.load()
    }

    /// The maximum amount of classic style ships allowed per player in the galaxy.
    pub fn player_max_classic_ships(&self) -> u8 {
        self.player_max_classic_ships.load()
    }

    /// The maximum amount of new style ships allowed per player in the galaxy.
    pub fn player_max_new_ships(&self) -> u8 {
        self.player_max_new_ships.load()
    }

    /// The maximum amount of bases allowed per player in the galaxy.
    pub fn player_max_bases(&self) -> u8 {
        self.player_max_bases.load()
    }

    /// `false`, if you have been disconnected.
    pub fn active(&self) -> bool {
        self.active.load()
    }

    /// `true` if a galaxy admin has enabled maintenance mode. When maintenance mode is enabled, new
    /// players or spectators cannot connect, and existing players cannot register new ships or
    /// continue existing ships. Some things in the galaxy (such as the game mode) can only be
    /// changed when maintenance mode is enabled, in order to maintain a consistent player state.
    pub fn maintenance(&self) -> bool {
        self.maintenance.load()
    }

    /// Awaits the next [`FlattiverseEvent`]
    pub async fn next_event(&self) -> Result<FlattiverseEvent, GameError> {
        self.events
            .recv()
            .await
            .map_err(|_| GameErrorKind::ConnectionTerminated.into())
    }

    /// Returns the next [`FlattiverseEvent`], if available.
    pub fn poll_next_event(&self) -> Result<Option<FlattiverseEvent>, GameError> {
        match self.events.try_recv() {
            Ok(event) => Ok(Some(event)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Closed) => Err(GameErrorKind::ConnectionTerminated.into()),
        }
    }

    /// Returns the underlying [`ConnectionHandle`] to the server.
    #[inline]
    pub(crate) fn connection(&self) -> &ConnectionHandle {
        &self.connection
    }
}
