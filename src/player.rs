use crate::atomics::Atomic;
use crate::hierarchy::{ConnectionProvider, ControllableInfo, ControllableInfoId, Galaxy};
use crate::network::PacketReader;
use crate::player_kind::PlayerKind;
use crate::{GameError, GameErrorKind, Identifiable, Indexer, NamedUnit, Team, UniversalArcHolder};
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Weak};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct PlayerId(pub(crate) u8);

impl Indexer for PlayerId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct Player {
    galaxy: Weak<Galaxy>,
    id: PlayerId,
    name: String,
    kind: PlayerKind,
    team: Arc<Team>,
    active: Atomic<bool>,
    controllables: UniversalArcHolder<ControllableInfoId, ControllableInfo>,
}

impl Player {
    #[inline]
    pub fn new(
        galaxy: Weak<Galaxy>,
        id: impl Into<PlayerId>,
        kind: PlayerKind,
        team: Arc<Team>,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            galaxy,
            id: id.into(),
            name: {
                let name = reader.read_string();
                let _ping = reader.read_int32();
                name
            },
            kind,
            team,
            active: Atomic::from(true),
            controllables: UniversalArcHolder::with_capacity(256),
        }
    }

    /// Sends a chat message with a maximum of 512 characters to this [`Player`].
    #[inline]
    pub async fn chat(&mut self, message: impl AsRef<str>) -> Result<(), GameError> {
        if !self.active() {
            Err(GameErrorKind::UnitIsBeingDeactivated.into())
        } else {
            self.galaxy
                .connection()?
                .chat_player(self.id, message)
                .await
        }
    }

    pub(crate) fn deactivate(&self) {
        self.active.store(false);
    }

    #[inline]
    pub fn id(&self) -> PlayerId {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn kind(&self) -> PlayerKind {
        self.kind
    }

    #[inline]
    pub fn team(&self) -> &Arc<Team> {
        &self.team
    }

    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    #[inline]
    pub fn controllable_info(&self) -> &UniversalArcHolder<ControllableInfoId, ControllableInfo> {
        &self.controllables
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
    fn name(&self) -> &str {
        Player::name(self)
    }
}

impl Display for Player {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player [{}] {}({:?})", self.id.0, self.name, self.kind)
    }
}
