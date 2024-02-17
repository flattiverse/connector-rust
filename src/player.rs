use crate::network::PacketReader;
use crate::player_kind::PlayerKind;
use crate::{Indexer, NamedUnit, TeamId};

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
    id: PlayerId,
    name: String,
    kind: PlayerKind,
    team: TeamId,
}

impl Player {
    #[inline]
    pub fn new(
        id: impl Into<PlayerId>,
        kind: PlayerKind,
        team: TeamId,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            team,
            name: reader.read_string(),
        }
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
    pub fn team(&self) -> TeamId {
        self.team
    }
}

impl NamedUnit for Player {
    #[inline]
    fn name(&self) -> &str {
        Player::name(self)
    }
}
