use crate::galaxy_hierarchy::{Galaxy, Identifiable, Indexer, NamedUnit, Team};
use crate::runtime::Atomic;
use crate::GameError;
use std::ops::Deref;
use std::sync::{Arc, Weak};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct PlayerId(pub(crate) u8);

impl Indexer for PlayerId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

/// Represents a player in the galaxy.
#[derive(Debug)]
pub struct Player {
    galaxy: Weak<Galaxy>,
    /// The id of the player
    pub id: PlayerId,
    /// The kind of the player.
    pub kind: PlayerKind,
    /// The team the player belongs to.
    pub team: Arc<Team>,
    /// The account name.
    pub name: String,
    ping: Atomic<f32>,
    active: Atomic<bool>,
}

impl Player {
    pub fn new(
        galaxy: Weak<Galaxy>,
        id: PlayerId,
        kind: PlayerKind,
        team: Arc<Team>,
        name: String,
        ping: f32,
    ) -> Self {
        Self {
            galaxy,
            id,
            kind,
            team,
            name,
            ping: Atomic::from(ping),
            active: Atomic::from(true),
        }
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

    /// The ping in ms of the player.
    #[inline]
    pub fn ping(&self) -> f32 {
        self.ping.load()
    }

    pub(crate) fn update(&self, ping: f32) {
        self.ping.store(ping);
    }

    pub(crate) fn deactivate(&self) {
        self.ping.store(-1.0);
        self.active.store(false);
    }
}

impl Identifiable<PlayerId> for Player {
    #[inline]
    fn id(&self) -> PlayerId {
        self.id
    }
}

impl NamedUnit for Player {
    #[inline]
    fn name(&self) -> impl Deref<Target = str> {
        self.name.as_str()
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
