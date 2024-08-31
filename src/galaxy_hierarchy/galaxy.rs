use crate::galaxy_hierarchy::{GameMode, Player, PlayerId, Team, TeamId, UniversalHolder};
use crate::network::{ConnectError, ConnectionHandle, Packet};
use crate::runtime::Atomic;
use crate::{FlattiverseEvent, GameError, GameErrorKind};
use async_channel::Receiver;
use std::sync::Arc;

pub struct Galaxy {
    name: String,

    game_mode: Atomic<GameMode>,
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

    player_max_total_ships: u16,
    player_max_classic_ships: u16,
    player_max_new_ships: u16,
    player_max_bases: u16,

    maintenance: bool,
    active: bool,

    teams: UniversalHolder<TeamId, Team>,
    // clusters: UniversalHolder<ClusterId, Cluster>,
    players: UniversalHolder<PlayerId, Player>,

    connection: ConnectionHandle,
    events: Receiver<FlattiverseEvent>,

    player: Atomic<PlayerId>,
}

impl Galaxy {
    pub const AUTH_ANONYMOUS: &'static str =
        "0000000000000000000000000000000000000000000000000000000000000000";
    pub const URI_BASE: &'static str = "www.flattiverse.com";

    #[cfg(not(feature = "dev-environment"))]
    pub const URI_GALAXY_DEFAULT: &'static str = "https://www.flattiverse.com/api/galaxies/all";

    #[cfg(feature = "dev-environment")]
    pub const URI_GALAXY_DEFAULT: &'static str = "http://localhost:8080/api/galaxies/all";

    #[inline]
    pub async fn connect(
        auth: impl Into<Option<&str>>,
        team: impl Into<Option<&str>>,
    ) -> Result<Arc<Self>, GameError> {
        Self::connect_to(Self::URI_GALAXY_DEFAULT, auth, team).await
    }

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
                Arc::new(Self {
                    name: String::default(),
                    game_mode: Default::default(),
                    description: String::default(),
                    max_players: 0,
                    max_spectators: 0,
                    galaxy_max_total_ships: 0,
                    galaxy_max_classic_ships: 0,
                    galaxy_max_new_ships: 0,
                    galaxy_max_bases: 0,
                    team_max_total_ships: 0,
                    team_max_classic_ships: 0,
                    team_max_new_ships: 0,
                    team_max_bases: 0,
                    player_max_total_ships: 0,
                    player_max_classic_ships: 0,
                    player_max_new_ships: 0,
                    player_max_bases: 0,
                    maintenance: false,
                    active: true,
                    teams: {
                        let mut teams = UniversalHolder::with_capacity(33);
                        teams[TeamId(32)] = Team::new(TeamId(33), "Spectators", 128, 128, 128);
                        teams
                    },
                    // clusters: UniversalHolder::with_capacity(64),
                    players: UniversalHolder::with_capacity(193),
                    connection: handle,
                    events: event_receiver,
                    player: Atomic::from(PlayerId(0)),
                })
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
        debug_assert!(self.players.get(id).is_none(), "{id:?} not setup.");
        self.player.store(id);
    }

    pub(crate) fn on_packet(
        self: &Arc<Self>,
        mut packet: Packet,
    ) -> Result<Option<FlattiverseEvent>, GameError> {
        #[cfg(feature = "dev-environment")]
        {
            debug!(
                "Processing packet with command=0x{:02x}",
                packet.header().command()
            );
        }

        todo!()
    }
}
