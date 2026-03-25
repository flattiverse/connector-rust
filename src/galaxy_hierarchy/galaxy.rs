use crate::galaxy_hierarchy::{
    BuildDisclosure, Cluster, ClusterId, Controllable, ControllableId, ControllableInfo,
    ControllableInfoId, GameMode, Player, PlayerId, PlayerKind, RuntimeDisclosure, Team, TeamId,
    UniversalArcHolder,
};
use crate::network::{ConnectError, ConnectionHandle, PacketReader};
use crate::unit::{Unit, UnitExtSealed, UnitKind};
use crate::utils::GuardedArcStringDeref;
use crate::utils::{Also, Atomic};
use crate::{
    ClusterSnapshot, FlattiverseEvent, FlattiverseEventKind, GalaxySettingsSnapshot, GameError,
    GameErrorKind, PlayerUnitDestroyedReason, TeamSnapshot,
};
use arc_swap::ArcSwap;
use async_channel::{Receiver, TryRecvError};
use std::ops::Deref;
use std::sync::Arc;
use tracing::instrument;

pub type EventSink = Vec<FlattiverseEvent>;

macro_rules! event {
    ($sink:expr, $kind:ident $content:tt) => {
        $sink.push({FlattiverseEventKind::$kind $content}.into());
    };
}

#[derive(Debug)]
pub struct Galaxy {
    name: ArcSwap<String>,

    game_mode: Atomic<GameMode>,
    description: ArcSwap<String>,

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
    requires_self_disclosure: Atomic<bool>,
    active: Atomic<bool>,
    received_compiled_with: Atomic<bool>,
    received_galaxy_settings: Atomic<bool>,
    compiled_with_max_players_supported: Atomic<u8>,
    compiled_with_symbol: ArcSwap<String>,

    teams: UniversalArcHolder<TeamId, Team>,
    clusters: UniversalArcHolder<ClusterId, Cluster>,
    players: UniversalArcHolder<PlayerId, Player>,
    controllables: UniversalArcHolder<ControllableId, Controllable>,

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

    pub const SPECTATORS_TEAM_ID: TeamId = TeamId(12);
    pub const TEAM_CAPACITY: usize = 13;
    pub const CLUSTER_CAPACITY: usize = 24;

    #[inline]
    pub async fn connect(
        galaxy: u16,
        auth: impl Into<Option<&str>>,
        team: impl Into<Option<&str>>,
        runtime_disclosure: Option<RuntimeDisclosure>,
        build_disclosure: Option<BuildDisclosure>,
    ) -> Result<Arc<Self>, ConnectError> {
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
                runtime_disclosure,
                build_disclosure,
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
                runtime_disclosure,
                build_disclosure,
            )
            .await
        }
    }

    #[instrument(level = "trace", skip(auth, team))]
    pub async fn connect_to(
        uri: &str,
        auth: impl Into<Option<&str>>,
        team: impl Into<Option<&str>>,
        runtime_disclosure: Option<RuntimeDisclosure>,
        build_disclosure: Option<BuildDisclosure>,
    ) -> Result<Arc<Self>, ConnectError> {
        let mut session = None;
        let this = crate::network::connect(
            uri,
            auth.into().unwrap_or(Self::AUTH_ANONYMOUS),
            team.into(),
            runtime_disclosure,
            build_disclosure,
            |handle, event_receiver| {
                session = Some(
                    handle
                        .sessions
                        .get()
                        .expect("Failed to get initial session"),
                );
                Arc::new(Self {
                    name: ArcSwap::default(),
                    game_mode: Atomic::from(GameMode::Mission),
                    description: ArcSwap::default(),
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
                    requires_self_disclosure: Atomic::from(false),
                    active: Atomic::from(true),
                    received_compiled_with: Atomic::from(false),
                    received_galaxy_settings: Atomic::from(false),
                    compiled_with_max_players_supported: Atomic::default(),
                    compiled_with_symbol: ArcSwap::default(),
                    teams: UniversalArcHolder::with_capacity(Self::TEAM_CAPACITY),
                    clusters: UniversalArcHolder::with_capacity(Self::CLUSTER_CAPACITY),
                    players: UniversalArcHolder::with_capacity(193),
                    controllables: UniversalArcHolder::with_capacity(192),
                    connection: handle,
                    events: event_receiver,
                    player: Atomic::from(PlayerId(0)),
                })
                .also(|galaxy| {
                    galaxy.teams.populate(Team::new(
                        Arc::downgrade(&galaxy),
                        Self::SPECTATORS_TEAM_ID,
                        "Spectators",
                        128,
                        128,
                        128,
                    ));
                })
            },
        )
        .await?;

        session
            .expect("Failed to get initial session")
            .response()
            .await
            .map_err(|e| ConnectError::GameError(e.into()))?
            .read(|reader| {
                this.setup_self(reader.read_byte());
            });

        Ok(this)
    }

    #[instrument(level = "trace", skip(self))]
    fn setup_self(&self, id: u8) {
        debug_assert!(id < 193, "Id out of bounds.");
        let id = PlayerId(id);
        debug_assert!(
            self.players.has(id),
            "Failed to set {id:?} as self, id unknown."
        );
        self.player.store(id);
    }

    /// Sends a chat message to all players in this [`Galaxy`].
    #[inline]
    pub async fn chat(&self, message: impl AsRef<str>) -> Result<(), GameError> {
        self.connection.chat_galaxy(message).await
    }

    /// Create a classic style ship.
    #[inline]
    pub async fn create_classic_ship(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Arc<Controllable>, GameError> {
        let id = self.connection.create_classic_style_ship(name).await?;
        Ok(self.get_controllable(id))
    }

    /// Configures galaxy metadata, teams and clusters from an XML document.
    /// Missing attributes keep old values for the referenced element.
    /// Team/Cluster elements define the final set: missing ids are removed.
    /// Unknown attributes and unknown child nodes are rejected by the server.
    ///
    /// ```xml
    /// <Galaxy Name="New Name">
    ///   <Team Id="0" />
    ///   <Team Id="1" Name="Green" ColorR="64" ColorG="255" ColorB="64" />
    ///   <Cluster Id="0" Name="Playground" Start="true" Respawn="false" />
    /// </Galaxy>
    /// ```
    ///
    /// Team id 12 (Spectators) must not be included.
    /// Team names must be unique.
    /// Removing a team fails if any remaining cluster still has regions referencing that team.
    /// Galaxy/Team/Cluster names must be non-empty and at most 32 characters.
    /// Description must be at most 4096 characters.
    /// At least one cluster must end up with `Start="true"`.
    #[inline]
    pub async fn configure(&self, xml: impl AsRef<str>) -> Result<(), GameError> {
        self.connection.configure_galaxy(xml).await
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn ping_pong(
        &self,
        events: &mut EventSink,
        challenge: u16,
    ) -> Result<(), GameError> {
        debug!("Responding to ping with challenge={challenge:#04x}");
        self.connection.respond_to_ping(challenge)?;
        events.push(FlattiverseEventKind::RespondedToPingMeasurement { challenge }.into());
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn update_galaxy(
        self: &Arc<Self>,
        events: &mut EventSink,
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
        maintenance: u8,
        requires_self_disclosure: u8,
    ) -> Result<(), GameError> {
        debug!("Updating galaxy");
        let before = if self.received_galaxy_settings.load() {
            Some(GalaxySettingsSnapshot::from(&**self))
        } else {
            None
        };

        self.game_mode.store(game_mode);
        self.name.store(Arc::new(name));
        self.description.store(Arc::new(description));
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
        self.maintenance.store(maintenance != 0);
        self.requires_self_disclosure
            .store(requires_self_disclosure != 0);

        self.received_galaxy_settings.store(true);

        event!(
            events,
            GalaxySettingsUpdated {
                galaxy: Arc::clone(self),
                before,
            }
        );

        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn update_team(
        self: &Arc<Self>,
        events: &mut EventSink,
        id: TeamId,
        red: u8,
        green: u8,
        blue: u8,
        name: String,
    ) -> Result<(), GameError> {
        debug!("Updating team with {id:?}");
        debug_assert!(id.0 < Self::SPECTATORS_TEAM_ID.0, "Invalid {id:?}");
        match self.teams.get_opt(id) {
            Some(team) => {
                let before = TeamSnapshot::from(&*team);
                team.update(name, red, green, blue);
                event!(events, TeamUpdated { team, before });
            }
            None => {
                event!(
                    events,
                    TeamCreated {
                        team: self.teams.populate(Team::new(
                            Arc::downgrade(self),
                            id,
                            name,
                            red,
                            green,
                            blue,
                        ))
                    }
                );
            }
        }
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn update_team_score(
        &self,
        events: &mut EventSink,
        id: TeamId,
        player_kills: u32,
        player_deaths: u32,
        friendly_kills: u32,
        friendly_deaths: u32,
        npc_kills: u32,
        npc_deaths: u32,
        neutral_deaths: u32,
        mission: i32,
    ) -> Result<(), GameError> {
        debug!("Updating Score for Team with {id:?}");
        debug_assert!(id.0 < Self::SPECTATORS_TEAM_ID.0, "Invalid {id:?}");
        debug_assert!(self.teams.has(id), "{id:?} does not exist.");
        let team = self.teams.get(id);
        let before = team.score().clone();
        team.score().update(
            player_kills,
            player_deaths,
            friendly_kills,
            friendly_deaths,
            npc_kills,
            npc_deaths,
            neutral_deaths,
            mission,
        );
        event!(events, TeamScoreUpdated { team, before });
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn deactivate_team(
        &self,
        events: &mut EventSink,
        id: TeamId,
    ) -> Result<(), GameError> {
        debug!("Deactivating team with {id:?}");
        debug_assert!(id.0 < Self::SPECTATORS_TEAM_ID.0, "Invalid {id:?}");
        event!(
            events,
            TeamRemoved {
                team: {
                    self.teams.get(id).deactivate();
                    self.teams.remove(id)
                },
            }
        );
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn update_cluster(
        self: &Arc<Galaxy>,
        events: &mut EventSink,
        id: ClusterId,
        name: String,
        flags: u8,
    ) -> Result<(), GameError> {
        debug!("Updating cluster with {id:?}");
        debug_assert!(usize::from(id.0) < Self::CLUSTER_CAPACITY, "Invalid {id:?}");
        let start = (flags & 0x01) != 0;
        let respawn = (flags & 0x02) != 0;
        match self.clusters.get_opt(id) {
            Some(cluster) => {
                let before = ClusterSnapshot::from(&*cluster);
                cluster.update(name, start, respawn);
                event!(events, ClusterUpdated { cluster, before });
            }
            None => {
                event!(
                    events,
                    ClusterCreated {
                        cluster: self.clusters.populate(Cluster::new(
                            Arc::downgrade(self),
                            id,
                            name,
                            start,
                            respawn,
                        ))
                    }
                );
            }
        }
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn deactivate_cluster(
        &self,
        events: &mut EventSink,
        id: ClusterId,
    ) -> Result<(), GameError> {
        debug!("Deactivating cluster with {id:?}");
        debug_assert!(usize::from(id.0) < Self::CLUSTER_CAPACITY, "Invalid {id:?}");
        event!(
            events,
            ClusterRemoved {
                cluster: {
                    self.clusters.get(id).deactivate();
                    self.clusters.remove(id)
                },
            }
        );
        Ok(())
    }

    #[instrument(level = "trace", skip(self, reader))]
    pub(crate) fn create_player(
        self: &Arc<Self>,
        events: &mut EventSink,
        id: PlayerId,
        kind: PlayerKind,
        team: TeamId,
        name: String,
        ping: f32,
        admin: bool,
        state_flags: u8,
        rank: i32,
        player_kills: i64,
        player_deaths: i64,
        friendly_kills: i64,
        friendly_deaths: i64,
        npc_kills: i64,
        npc_deaths: i64,
        neutral_deaths: i64,
        has_avatar: bool,
        reader: &mut dyn PacketReader,
    ) -> Result<(), GameError> {
        debug!("Creating player with {id:?}");
        debug_assert!(id.0 < 193, "Invalid {id:?}");
        debug_assert!(self.players.has_not(id), "{id:?} does already exist.");
        debug_assert!(self.teams.has(team), "{team:?} does not exist.");

        let disclosure_flags = reader.read_byte();
        let runtime_disclosure = if (disclosure_flags & 0x01) != 0 {
            RuntimeDisclosure::try_read(reader)
        } else {
            None
        };

        let build_disclosure = if (disclosure_flags & 0x02) != 0 {
            BuildDisclosure::try_read(reader)
        } else {
            None
        };

        let disconnected = state_flags & 0x01 != 0;
        let player = self.players.populate(Player::new(
            Arc::downgrade(self),
            id,
            kind,
            Arc::downgrade(&self.teams.get(team)),
            name,
            ping,
            admin,
            disconnected,
            rank,
            player_kills,
            player_deaths,
            friendly_kills,
            friendly_deaths,
            npc_kills,
            npc_deaths,
            neutral_deaths,
            has_avatar,
            runtime_disclosure,
            build_disclosure,
        ));

        event!(
            events,
            PlayerJoined {
                player: player.clone()
            }
        );

        if player.disconnected() {
            event!(events, PlayerDisconnected { player });
        }

        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn update_player(
        &self,
        events: &mut EventSink,
        id: PlayerId,
        ping: f32,
        admin: bool,
        state_flag: u8,
        rank: i32,
        player_kills: i64,
        player_deaths: i64,
        friendly_kills: i64,
        friendly_deaths: i64,
        npc_kills: i64,
        npc_deaths: i64,
        neutral_deaths: i64,
    ) -> Result<(), GameError> {
        debug!("Updating player with {id:?}");
        debug_assert!(id.0 < 193, "Invalid {id:?}");
        debug_assert!(self.players.has(id), "{id:?} does not exist.");

        let disconnected = state_flag & 0x01 != 0;
        let player = self.players.get(id).also(|player| {
            player.update(
                ping,
                admin,
                disconnected,
                rank,
                player_kills,
                player_deaths,
                friendly_kills,
                friendly_deaths,
                npc_kills,
                npc_deaths,
                neutral_deaths,
            );
        });

        event!(
            events,
            PlayerUpdated {
                player: player.clone()
            }
        );

        if player.disconnected() {
            event!(events, PlayerDisconnected { player });
        }

        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn update_player_score(
        &self,
        events: &mut EventSink,
        id: PlayerId,
        player_kills: u32,
        player_deaths: u32,
        friendly_kills: u32,
        friendly_deaths: u32,
        npc_kills: u32,
        npc_deaths: u32,
        neutral_deaths: u32,
        mission: i32,
    ) -> Result<(), GameError> {
        debug!("Updating Score for player with {id:?}");
        debug_assert!(id.0 < 193, "Invalid {id:?}");
        debug_assert!(self.players.has(id), "{id:?} does not exist.");
        let player = self.players.get(id);
        let before = player.score().clone();
        player.score().update(
            player_kills,
            player_deaths,
            friendly_kills,
            friendly_deaths,
            npc_kills,
            npc_deaths,
            neutral_deaths,
            mission,
        );
        event!(events, PlayerScoreUpdated { player, before });
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn deactivate_player(
        &self,
        events: &mut EventSink,
        id: PlayerId,
    ) -> Result<(), GameError> {
        debug!("Deactivating player with {id:?}");
        debug_assert!(id.0 < 193, "Invalid {id:?}");
        debug_assert!(self.players.has(id), "{id:?} does not exist.");
        event!(
            events,
            PlayerParted {
                player: {
                    self.players.get(id).deactivate();
                    self.players.remove(id)
                }
            }
        );
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn controllable_info_new(
        self: &Arc<Self>,
        events: &mut EventSink,
        player: PlayerId,
        kind: UnitKind,
        id: ControllableInfoId,
        name: String,
        alive: bool,
    ) -> Result<(), GameError> {
        debug!("New ControllableInfo for {player:?} with {id:?}");
        debug_assert!(self.players.has(player), "{player:?} does not exist.");
        let player = self.players.get(player);
        let controllable = player
            .controllable_infos
            .populate(ControllableInfo::from_packet(
                kind,
                Arc::downgrade(&self),
                Arc::downgrade(&player),
                id,
                name,
                alive,
            )?);
        event!(
            events,
            ControllableInfoRegistered {
                player,
                controllable,
            }
        );
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn controllable_info_alive(
        self: &Arc<Self>,
        events: &mut EventSink,
        player: PlayerId,
        id: ControllableInfoId,
    ) -> Result<(), GameError> {
        debug!("Updating ControllableInfo for {player:?} with {id:?}");
        debug_assert!(self.players.has(player), "{player:?} does not exist.");
        let player = self.players.get(player);
        let controllable = player.get_controllable_info(id);
        controllable.set_alive();
        event!(
            events,
            ControllableInfoContinued {
                player,
                controllable,
            }
        );
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn controllable_info_dead_by_reason(
        self: &Arc<Self>,
        events: &mut EventSink,
        player: PlayerId,
        id: ControllableInfoId,
        reason: PlayerUnitDestroyedReason,
    ) -> Result<(), GameError> {
        debug!("Death of ControllableInfo for {player:?} with {id:?}");
        debug_assert!(self.players.has(player), "{player:?} does not exist.");
        let player = self.players.get(player);
        let controllable = player.get_controllable_info(id);
        controllable.set_dead();
        event!(
            events,
            ControllableInfoDestroyed {
                player,
                controllable,
                reason,
            }
        );
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn controllable_info_dead_by_neutral_collision(
        self: &Arc<Self>,
        events: &mut EventSink,
        player: PlayerId,
        id: ControllableInfoId,
        colliders_kind: UnitKind,
        colliders_name: String,
    ) -> Result<(), GameError> {
        debug!("Death of ControllableInfo for {player:?} with {id:?} (neutral collision)");
        debug_assert!(self.players.has(player), "{player:?} does not exist.");
        let player = self.players.get(player);
        let controllable = player.get_controllable_info(id);
        controllable.set_dead();
        event!(
            events,
            ControllableInfoDestroyedByNeutralCollision {
                player,
                controllable,
                reason: PlayerUnitDestroyedReason::CollidedWithNeutralUnit,
                colliders_kind,
                colliders_name
            }
        );
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn controllable_info_dead_by_player_unit(
        self: &Arc<Self>,
        events: &mut EventSink,
        player: PlayerId,
        id: ControllableInfoId,
        reason: PlayerUnitDestroyedReason,
        causer: PlayerId,
        causer_controllable_info: ControllableInfoId,
    ) -> Result<(), GameError> {
        debug!("Death of ControllableInfo for {player:?} with {id:?} (player collision)");
        debug_assert!(self.players.has(player), "{player:?} does not exist.");
        let player = self.players.get(player);
        let controllable = player.get_controllable_info(id);
        controllable.set_dead();
        debug_assert!(self.players.has(causer), "{causer:?} does not exist");
        let destroyer = self.players.get(causer);
        let destroyer_unit = destroyer.get_controllable_info(causer_controllable_info);
        event!(
            events,
            ControllableInfoDestroyedByPlayerUnit {
                player,
                controllable,
                reason,
                destroyer_player: destroyer,
                destroyed_unit: destroyer_unit,
            }
        );
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn controllable_info_score_updated(
        self: &Arc<Self>,
        events: &mut EventSink,
        player: PlayerId,
        id: ControllableInfoId,
        player_kills: u32,
        player_deaths: u32,
        friendly_kills: u32,
        friendly_deaths: u32,
        npc_kills: u32,
        npc_deaths: u32,
        neutral_deaths: u32,
        mission: i32,
    ) -> Result<(), GameError> {
        debug!("Updating Score for ControllableId with {id:?} of {player:?}.");
        debug_assert!(self.players.has(player), "{player:?} does not exist.");
        let player = self.players.get(player);
        let controllable = player.get_controllable_info(id);
        let before = controllable.score().clone();
        controllable.score().update(
            player_kills,
            player_deaths,
            friendly_kills,
            friendly_deaths,
            npc_kills,
            npc_deaths,
            neutral_deaths,
            mission,
        );
        event!(
            events,
            ControllableInfoScoreUpdated {
                player,
                controllable,
                before,
            }
        );
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn controllable_info_removed(
        self: &Arc<Self>,
        events: &mut EventSink,
        player: PlayerId,
        id: ControllableInfoId,
    ) -> Result<(), GameError> {
        debug!("Removing ControllableInfo for {player:?} with {id:?}");
        debug_assert!(self.players.has(player), "{player:?} does not exist.");
        let player = self.players.get(player);
        match player.controllable_infos.remove_opt(id) {
            None => {
                error!("Failed to remove ControllableInfo. {id:?} does not exist for {player:?}.");
            }
            Some(controllable) => {
                event!(
                    events,
                    ControllableInfoClosed {
                        player,
                        controllable,
                    }
                );
            }
        }
        Ok(())
    }

    #[instrument(level = "trace", skip(self, reader))]
    pub(crate) fn controllable_new(
        self: &Arc<Self>,
        events: &mut EventSink,
        kind: UnitKind,
        id: ControllableId,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<(), GameError> {
        let _ = events;
        debug!("New Controllable with {id:?} and name {name:?}");
        self.controllables.populate(Controllable::from_packet(
            kind,
            Arc::downgrade(&self.clusters.get(ClusterId(0))), // TODO
            id,
            name,
            reader,
        )?);

        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn controllable_deceased(
        self: &Arc<Self>,
        events: &mut EventSink,
        id: ControllableId,
    ) -> Result<(), GameError> {
        let _ = events;
        debug!("{id:?} deceased");
        self.controllables.get(id).deceased();
        Ok(())
    }

    #[instrument(level = "trace", skip(self, reader))]
    pub(crate) fn controllable_updated(
        self: &Arc<Self>,
        events: &mut EventSink,
        id: ControllableId,
        reader: &mut dyn PacketReader,
    ) -> Result<(), GameError> {
        let _ = events;
        debug!("Updating Controllable with {id:?}");
        if let Some(controllable) = self.controllables.get_opt(id) {
            controllable.update(reader);
        } else {
            error!("There is no Controllable for {id:?}");
        }
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn controllable_removed(
        self: &Arc<Self>,
        events: &mut EventSink,
        id: ControllableId,
    ) -> Result<(), GameError> {
        let _ = events;
        debug!("{id:?} removed");
        self.controllables.remove(id).deactivate();
        Ok(())
    }

    #[instrument(level = "trace", skip(self, reader))]
    pub(crate) fn unit_new(
        &self,
        events: &mut EventSink,
        cluster: ClusterId,
        name: String,
        kind: UnitKind,
        reader: &mut dyn PacketReader,
    ) -> Result<(), GameError> {
        debug!("Adding unit {name:?}");
        debug_assert!(self.clusters.has(cluster), "{cluster:?} does not exist.");

        let cluster = self.clusters.get(cluster);
        let unit = match Unit::try_read(kind, Arc::downgrade(&cluster), name, reader) {
            None => {
                error!("Unable to read Unit for UnitKind::{kind:?}");
                return Ok(());
            }
            Some(unit) => Arc::new(unit),
        };

        cluster.add_unit(Arc::clone(&unit));
        event!(events, UnitAdded { unit });

        Ok(())
    }

    #[instrument(level = "trace", skip(self, reader))]
    pub(crate) fn unit_updated_movement(
        &self,
        events: &mut EventSink,
        cluster: ClusterId,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<(), GameError> {
        debug!("Updating unit {name:?}");
        debug_assert!(self.clusters.has(cluster), "{cluster:?} does not exist.");

        let cluster = self.clusters.get(cluster);
        if let Some(unit) = cluster.get_unit(&name) {
            unit.update_movement(reader);
            event!(events, UnitUpdated { unit });
        } else {
            error!("Failed to find unit with name {name:?}");
        }

        Ok(())
    }

    #[instrument(level = "trace", skip(self, reader))]
    pub(crate) fn unit_updated_state(
        &self,
        events: &mut EventSink,
        cluster: ClusterId,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<(), GameError> {
        debug!("Updating state of unit {name:?}");
        debug_assert!(self.clusters.has(cluster), "{cluster:?} does not exist.");

        let cluster = self.clusters.get(cluster);
        if let Some(unit) = cluster.get_unit(&name) {
            unit.update_state(reader);
            event!(events, UnitUpdated { unit });
        } else {
            error!("Failed to find unit with name {name:?}");
        }

        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn unit_updated_by_admin(
        &self,
        events: &mut EventSink,
        cluster: ClusterId,
        name: String,
    ) -> Result<(), GameError> {
        debug!("Admin has updated the unit {name:?}");
        event!(events, UnitAlteredByAdmin { cluster, name });
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn unit_removed(
        &self,
        events: &mut EventSink,
        cluster: ClusterId,
        name: String,
    ) -> Result<(), GameError> {
        debug!("Removing unit {name:?}");
        debug_assert!(self.clusters.has(cluster), "{cluster:?} does not exist.");

        let cluster = self.clusters.get(cluster);
        if let Some(unit) = cluster.remove_unit_(&name) {
            event!(events, UnitRemoved { unit });
        } else {
            error!("Failed to remove unit with name {name:?}");
        }

        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn compiled_with(
        &self,
        events: &mut EventSink,
        max_players_supported: u8,
        symbol: String,
    ) -> Result<(), GameError> {
        debug!("Compiled with message with max_players_supported={max_players_supported:?}, symbol={symbol:?}");

        if self.received_compiled_with.load() {
            warn!("Received compiled-with flags again, ignoring max_players_supported={max_players_supported:?}, symbol={symbol:?}");
        } else {
            self.compiled_with_max_players_supported
                .store(max_players_supported);
            self.compiled_with_symbol.store(Arc::new(symbol));
            self.received_compiled_with.store(true);

            let symbol = self.compiled_with_symbol.load_full();
            event!(events, CompiledWithMessage {
                message: format!("The server has been compiled with support for up to {max_players_supported} players ({symbol})"),
                max_players_supported,
                symbol,
            });
        }

        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn universe_tick(
        &self,
        events: &mut EventSink,
        number: u32,
    ) -> Result<(), GameError> {
        debug!("Universe tick with #{number}");
        event!(events, GalaxyTick { tick: number });
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn flag_scored_chat(
        &self,
        events: &mut EventSink,
        player: PlayerId,
        controllable: ControllableInfoId,
        flag_team: TeamId,
        flag_name: String,
    ) -> Result<(), GameError> {
        debug!("Received flag scored chat message: {player:?}, {controllable:?}, {flag_team:?}, flag_name={flag_name:?}");
        debug_assert!(self.players.has(player), "{player:?} does not exist.");
        debug_assert!(self.teams.has(flag_team), "{flag_team:?} does not exist.");
        let player = self.players.get(player);
        let controllable_info = player.get_controllable_info(controllable);
        event!(
            events,
            FlagScoredChat {
                player,
                controllable_info,
                flag_team: self.teams.get(flag_team),
                flag_name,
            }
        );
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn domination_point_scored_chat(
        &self,
        events: &mut EventSink,
        team: TeamId,
        domination_point_name: String,
    ) -> Result<(), GameError> {
        debug!("Received flag scored chat message: {team:?}, domination_point_name={domination_point_name:?}");
        debug_assert!(self.teams.has(team), "{team:?} does not exist.");
        event!(
            events,
            DominationPointScoredChat {
                team: self.teams.get(team),
                domination_point_name
            }
        );
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn own_flag_hit(
        &self,
        events: &mut EventSink,
        player: PlayerId,
        controllable: ControllableInfoId,
        flag_team: TeamId,
        flag_name: String,
    ) -> Result<(), GameError> {
        debug!("Received own flag hit chat message: {player:?}, {controllable:?}, {flag_team:?}, flag_name={flag_name:?}");
        debug_assert!(self.players.has(player), "{player:?} does not exist.");
        debug_assert!(self.teams.has(flag_team), "{flag_team:?} does not exist.");
        let player = self.players.get(player);
        let controllable_info = player.get_controllable_info(controllable);
        event!(
            events,
            OwnFlagHitChat {
                player,
                controllable_info,
                flag_team: self.teams.get(flag_team),
                flag_name,
            }
        );
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn chat_galaxy(
        self: &Arc<Self>,
        events: &mut EventSink,
        player: PlayerId,
        message: String,
    ) -> Result<(), GameError> {
        debug!("Received galaxy chat message: {message:?}");
        debug_assert!(self.players.has(player), "{player:?} does not exist.");
        event!(
            events,
            GalaxyChat {
                player: self.players.get(player),
                destination: Arc::clone(self),
                message
            }
        );
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn chat_team(
        self: &Arc<Self>,
        events: &mut EventSink,
        player: PlayerId,
        message: String,
    ) -> Result<(), GameError> {
        debug!("Received team chat message: {message:?}");
        debug_assert!(self.players.has(player), "{player:?} does not exist.");
        event!(
            events,
            TeamChat {
                player: self.players.get(player),
                destination: self.player(),
                message
            }
        );
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn chat_player(
        self: &Arc<Self>,
        events: &mut EventSink,
        player: PlayerId,
        message: String,
    ) -> Result<(), GameError> {
        debug!("Received player chat message: {message:?}");
        debug_assert!(self.players.has(player), "{player:?} does not exist.");
        event!(
            events,
            PlayerChat {
                player: self.players.get(player),
                destination: self.player(),
                message
            }
        );
        Ok(())
    }

    #[instrument(level = "trace", skip(self, events))]
    pub(crate) fn flag_reactivated_chat(
        &self,
        events: &mut EventSink,
        flag_team: TeamId,
        flag_name: String,
    ) -> Result<(), GameError> {
        debug!("Received flag reactivated chat message: {flag_team:?}, flag_name={flag_name:?}");
        debug_assert!(self.teams.has(flag_team), "{flag_team:?} does not exist.");
        event!(
            events,
            FlagReactivatedChat {
                flag_team: self.teams.get(flag_team),
                flag_name,
            }
        );
        Ok(())
    }

    /// Yourself.
    pub fn player(&self) -> Arc<Player> {
        self.players.get(self.player.load())
    }

    /// The name of the galaxy.
    #[inline]
    pub fn name(&self) -> impl Deref<Target = str> {
        GuardedArcStringDeref(self.name.load())
    }

    /// The description of the galaxy.
    #[inline]
    pub fn description(&self) -> impl Deref<Target = str> {
        GuardedArcStringDeref(self.description.load())
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

    /// Whether this galaxy requires self-disclosure for regular player logins.
    pub fn requires_self_disclosure(&self) -> bool {
        self.requires_self_disclosure.load()
    }

    /// The maximum amount of players the server binary has been compiled to support.
    pub fn compiled_with_may_players_supported(&self) -> u8 {
        self.compiled_with_max_players_supported.load()
    }

    /// The compile symbol that selected the server binary's player-capacity profile.
    pub fn compiled_with_symbol(&self) -> Arc<String> {
        self.compiled_with_symbol.load_full()
    }

    /// Awaits the next [`FlattiverseEvent`]
    pub async fn next_event(&self) -> Result<FlattiverseEvent, GameError> {
        self.events.recv().await.map_err(|_| {
            GameErrorKind::ConnectionTerminated {
                reason: Some(Arc::from("Event-Receiver gone")),
            }
            .into()
        })
    }

    /// Returns the next [`FlattiverseEvent`], if available.
    pub fn poll_next_event(&self) -> Result<Option<FlattiverseEvent>, GameError> {
        match self.events.try_recv() {
            Ok(event) => Ok(Some(event)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Closed) => Err(GameErrorKind::ConnectionTerminated {
                reason: Some(Arc::from("Event-Receiver gone")),
            }
            .into()),
        }
    }

    /// Returns the underlying [`ConnectionHandle`] to the server.
    #[inline]
    pub(crate) fn connection(&self) -> &ConnectionHandle {
        &self.connection
    }

    #[inline]
    pub fn iter_teams(&self) -> impl Iterator<Item = Arc<Team>> + '_ {
        self.teams.iter()
    }

    #[inline]
    pub fn get_team(&self, id: TeamId) -> Arc<Team> {
        self.teams.get(id)
    }

    #[inline]
    pub fn get_team_opt(&self, id: TeamId) -> Option<Arc<Team>> {
        self.teams.get_opt(id)
    }
    #[inline]
    pub fn iter_clusters(&self) -> impl Iterator<Item = Arc<Cluster>> + '_ {
        self.clusters.iter()
    }

    #[inline]
    pub fn get_cluster(&self, id: ClusterId) -> Arc<Cluster> {
        self.clusters.get(id)
    }

    #[inline]
    pub fn get_cluster_opt(&self, id: ClusterId) -> Option<Arc<Cluster>> {
        self.clusters.get_opt(id)
    }

    #[inline]
    pub fn iter_players(&self) -> impl Iterator<Item = Arc<Player>> + '_ {
        self.players.iter()
    }

    #[inline]
    pub fn get_player(&self, id: PlayerId) -> Arc<Player> {
        self.players.get(id)
    }

    #[inline]
    pub fn get_player_opt(&self, id: PlayerId) -> Option<Arc<Player>> {
        self.players.get_opt(id)
    }

    #[inline]
    pub fn iter_controllables(&self) -> impl Iterator<Item = Arc<Controllable>> + '_ {
        self.controllables.iter()
    }

    #[inline]
    pub fn get_controllable(&self, id: ControllableId) -> Arc<Controllable> {
        self.controllables.get(id)
    }

    #[inline]
    pub fn get_controllable_opt(&self, id: ControllableId) -> Option<Arc<Controllable>> {
        self.controllables.get_opt(id)
    }
}
