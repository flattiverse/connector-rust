use crate::hierarchy::{ControllableInfo, ControllableInfoId};
use crate::network::PacketReader;
use crate::player_kind::PlayerKind;
use crate::{Indexer, NamedUnit, TeamId, UniversalHolder};
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

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
    active: bool,
    controllables: UniversalHolder<ControllableInfoId, ControllableInfo>,
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
            active: true,
            id: id.into(),
            kind,
            team,
            name: reader.read_string(),
            controllables: UniversalHolder::with_capacity(256),
        }
    }

    pub(crate) fn deactivate(&mut self) {
        self.active = false;
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

    #[inline]
    pub fn active(&self) -> bool {
        self.active
    }

    #[inline]
    pub fn controllables_info(&self) -> &UniversalHolder<ControllableInfoId, ControllableInfo> {
        &self.controllables
    }

    #[inline]
    pub fn controllables_info_mut(
        &mut self,
    ) -> &mut UniversalHolder<ControllableInfoId, ControllableInfo> {
        &mut self.controllables
    }

    #[inline]
    pub fn iter_controllables_info(&self) -> impl Iterator<Item = &ControllableInfo> {
        self.controllables.iter()
    }
}

impl Index<ControllableInfoId> for Player {
    type Output = ControllableInfo;

    #[inline]
    fn index(&self, index: ControllableInfoId) -> &Self::Output {
        &self.controllables[index]
    }
}

impl IndexMut<ControllableInfoId> for Player {
    #[inline]
    fn index_mut(&mut self, index: ControllableInfoId) -> &mut Self::Output {
        &mut self.controllables[index]
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
