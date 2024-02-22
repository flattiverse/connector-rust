use crate::hierarchy::{ControllableInfo, ControllableInfoId};
use crate::network::PacketReader;
use crate::player_kind::PlayerKind;
use crate::{Indexer, NamedUnit, TeamId, UniversalHolder};
use std::fmt::{Display, Formatter};

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

    pub(crate) fn add_controllable_info(&mut self, info: ControllableInfo) {
        self.controllables.set(info.id(), info);
    }

    pub(crate) fn remove_controllable_info(
        &mut self,
        id: ControllableInfoId,
    ) -> Option<ControllableInfo> {
        if let Some(mut controllable) = self.controllables.remove(id) {
            controllable.deactivate();
            Some(controllable)
        } else {
            warn!("Did not find ControllableInfo for {id:?}");
            None
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

    #[inline]
    pub fn active(&self) -> bool {
        self.active
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
