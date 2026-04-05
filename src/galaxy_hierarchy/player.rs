use crate::galaxy_hierarchy::{
    BuildDisclosure, ControllableInfo, ControllableInfoId, Galaxy, Identifiable, Indexer,
    RuntimeDisclosure, Score, Team, UniversalArcHolder,
};
use crate::utils::Atomic;
use crate::{GameError, GameErrorKind, ProgressState};
use std::sync::{Arc, Weak};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct PlayerId(pub(crate) u8);

impl Indexer for PlayerId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

/// Represents one player account inside the connected galaxy session.
#[derive(Debug)]
pub struct Player {
    galaxy: Weak<Galaxy>,
    id: PlayerId,
    kind: PlayerKind,
    team: Weak<Team>,
    name: String,
    ping: Atomic<f32>,
    score: Score,
    active: Atomic<bool>,
    admin: Atomic<bool>,
    disconnected: Atomic<bool>,
    rank: Atomic<i32>,
    player_kills: Atomic<i64>,
    player_deaths: Atomic<i64>,
    friendly_kills: Atomic<i64>,
    friendly_deaths: Atomic<i64>,
    npc_kills: Atomic<i64>,
    npc_deaths: Atomic<i64>,
    neutral_deaths: Atomic<i64>,
    has_avatar: bool,
    runtime_disclosure: Option<RuntimeDisclosure>,
    build_disclosure: Option<BuildDisclosure>,
    pub(crate) controllable_infos: UniversalArcHolder<ControllableInfoId, ControllableInfo>,
}

impl Player {
    pub fn new(
        galaxy: Weak<Galaxy>,
        id: PlayerId,
        kind: PlayerKind,
        team: Weak<Team>,
        name: String,
        ping: f32,
        admin: bool,
        disconnected: bool,
        rank: i32,
        player_kills: i64,
        player_deaths: i64,
        friendly_kills: i64,
        friendly_deaths: i64,
        npc_kills: i64,
        npc_deaths: i64,
        neutral_deaths: i64,
        has_avatar: bool,
        runtime_disclosure: Option<RuntimeDisclosure>,
        build_disclosure: Option<BuildDisclosure>,
    ) -> Self {
        Self {
            galaxy,
            id,
            kind,
            team,
            name,
            ping: Atomic::from(ping),
            score: Score::default(),
            active: Atomic::from(true),
            admin: Atomic::from(admin),
            disconnected: Atomic::from(disconnected),
            rank: Atomic::from(rank),
            player_kills: Atomic::from(player_kills),
            player_deaths: Atomic::from(player_deaths),
            friendly_kills: Atomic::from(friendly_kills),
            friendly_deaths: Atomic::from(friendly_deaths),
            npc_kills: Atomic::from(npc_kills),
            npc_deaths: Atomic::from(npc_deaths),
            neutral_deaths: Atomic::from(neutral_deaths),
            has_avatar,
            runtime_disclosure,
            build_disclosure,
            controllable_infos: UniversalArcHolder::with_capacity(256),
        }
    }

    /// Protocol id of the player.
    #[inline]
    pub fn id(&self) -> PlayerId {
        self.id
    }

    /// Login kind of the player, for example normal player, spectator, or admin.
    #[inline]
    pub fn kind(&self) -> PlayerKind {
        self.kind
    }

    /// The team the player belongs to.
    #[inline]
    pub fn team(&self) -> Arc<Team> {
        self.team.upgrade().unwrap()
    }

    /// The team the player belongs to but weak.
    #[inline]
    pub(crate) fn team_weak(&self) -> Weak<Team> {
        self.team.clone()
    }

    /// Sends a chat message to this [`Player`].
    #[inline]
    pub async fn chat(&self, message: impl AsRef<str>) -> Result<(), GameError> {
        self.galaxy
            .upgrade()
            .unwrap()
            .connection()
            .chat_player(self.id, message)
            .await
    }

    /// Downloads the player's cached small avatar image bytes.
    #[inline]
    pub async fn download_small_avatar(
        &self,
        progress_state: impl Into<Option<Arc<ProgressState>>>,
    ) -> Result<Vec<u8>, GameError> {
        if !self.has_avatar {
            Err(GameErrorKind::AvatarNotAvailable.into())
        } else {
            self.galaxy
                .upgrade()
                .unwrap()
                .connection()
                .player_download_small_avatar(self.id, progress_state.into())
                .await
        }
    }

    /// Downloads the player's cached big avatar image bytes.
    #[inline]
    pub async fn download_big_avatar(
        &self,
        progress_state: impl Into<Option<Arc<ProgressState>>>,
    ) -> Result<Vec<u8>, GameError> {
        if !self.has_avatar {
            Err(GameErrorKind::AvatarNotAvailable.into())
        } else {
            self.galaxy
                .upgrade()
                .unwrap()
                .connection()
                .player_download_big_avatar(self.id, progress_state.into())
                .await
        }
    }

    /// Account display name inside this galaxy session.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// True while this player is still represented in the current galaxy session.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    /// True when the player's connection has already dropped and only session cleanup remains.
    #[inline]
    pub fn disconnected(&self) -> bool {
        self.disconnected.load()
    }

    /// Latest ping in milliseconds reported by the server.
    #[inline]
    pub fn ping(&self) -> f32 {
        self.ping.load()
    }

    /// Whether the account has administrator privileges.
    #[inline]
    pub fn admin(&self) -> bool {
        self.admin.load()
    }

    /// Global account rank.
    #[inline]
    pub fn rank(&self) -> i32 {
        self.rank.load()
    }

    /// Total kills of other players.
    #[inline]
    pub fn player_kills(&self) -> i64 {
        self.player_kills.load()
    }

    /// Total deaths caused by other players.
    #[inline]
    pub fn player_deaths(&self) -> i64 {
        self.player_deaths.load()
    }

    /// Total kills of teammates.
    #[inline]
    pub fn friendly_kills(&self) -> i64 {
        self.friendly_kills.load()
    }

    /// Total deaths caused by the same team, including self-inflicted deaths.
    #[inline]
    pub fn friendly_deaths(&self) -> i64 {
        self.friendly_deaths.load()
    }

    /// Total kills of NPC enemies.
    #[inline]
    pub fn npc_kills(&self) -> i64 {
        self.npc_kills.load()
    }

    /// Total deaths caused by NPC enemies.
    #[inline]
    pub fn npc_deaths(&self) -> i64 {
        self.npc_deaths.load()
    }

    /// Total deaths caused by neutral units or the environment.
    #[inline]
    pub fn neutral_deaths(&self) -> i64 {
        self.neutral_deaths.load()
    }

    /// Whether this player currently has a cached avatar available on the server.
    #[inline]
    pub fn has_avatar(&self) -> bool {
        self.has_avatar
    }

    /// Session-level runtime self-disclosure, if provided by the player.
    #[inline]
    pub fn runtime_disclosure(&self) -> Option<&RuntimeDisclosure> {
        self.runtime_disclosure.as_ref()
    }

    /// Session-level build-assistance self-disclosure, if provided by the player.
    #[inline]
    pub fn build_disclosure(&self) -> Option<&BuildDisclosure> {
        self.build_disclosure.as_ref()
    }

    /// Current live player score.
    #[inline]
    pub fn score(&self) -> &Score {
        &self.score
    }

    pub(crate) fn update(
        &self,
        ping: f32,
        admin: bool,
        disconnected: bool,
        rank: i32,
        player_kills: i64,
        player_deaths: i64,
        friendly_kills: i64,
        friendly_deaths: i64,
        npc_kills: i64,
        npc_deaths: i64,
        neutral_deaths: i64,
    ) {
        self.ping.store(ping);
        self.admin.store(admin);
        self.disconnected.store(disconnected);
        self.rank.store(rank);
        self.player_kills.store(player_kills);
        self.player_deaths.store(player_deaths);
        self.friendly_kills.store(friendly_kills);
        self.friendly_deaths.store(friendly_deaths);
        self.npc_kills.store(npc_kills);
        self.npc_deaths.store(npc_deaths);
        self.neutral_deaths.store(neutral_deaths);
    }

    pub(crate) fn deactivate(&self) {
        self.ping.store(-1.0);
        self.active.store(false);
        self.disconnected.store(true);

        for controllable in self.controllable_infos.iter() {
            controllable.deactivate();
        }
    }

    #[inline]
    pub fn get_controllable_info(&self, id: ControllableInfoId) -> Arc<ControllableInfo> {
        self.controllable_infos.get(id)
    }

    #[inline]
    pub fn get_controllable_info_opt(
        &self,
        id: ControllableInfoId,
    ) -> Option<Arc<ControllableInfo>> {
        self.controllable_infos.get_opt(id)
    }

    #[inline]
    pub fn iter_controllable_infos(&self) -> impl Iterator<Item = Arc<ControllableInfo>> + '_ {
        self.controllable_infos.iter()
    }

    /// The connected galaxy session this player belongs to.
    #[inline]
    pub fn galaxy(&self) -> Arc<Galaxy> {
        self.galaxy.upgrade().unwrap()
    }
}

impl Identifiable<PlayerId> for Player {
    #[inline]
    fn id(&self) -> PlayerId {
        self.id
    }
}

/// Specifies the kind of the client connected to the server.
#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    num_enum::FromPrimitive,
    num_enum::IntoPrimitive,
    strum::EnumIter,
    strum::AsRefStr,
)]
pub enum PlayerKind {
    /// It's a regular player which can register ships, etc.
    Player = 0x01,
    /// It's a spectator.
    Spectator = 0x02,
    /// It's an admin.
    Admin = 0x04,
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl PlayerKind {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}
