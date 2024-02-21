use crate::network::PacketReader;
use crate::player_kind::PlayerKind;
use crate::{Indexer, NamedUnit, TeamId};
use std::fmt::{Display, Formatter};
use rustc_hash::FxHashMap;
use crate::hierarchy::ControllableInfo;

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
    controllables: FxHashMap<String, ControllableInfo>,
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
            controllables: FxHashMap::default(),
        }
    }

    pub(crate) fn deactivate(&mut self) {
        self.active = false;
    }

    pub(crate) fn add_controllable_info(&mut self, info: ControllableInfo) {
        let name = info.name().to_string();
        self.controllables.insert(name, info);
    }

    pub(crate) fn remove_controllable_info(&mut self, name: &str) -> Option<ControllableInfo> {
        if let Some(mut controllable) = self.controllables.remove(name) {
            controllable.deactivate();
            Some(controllable)
        } else {
            warn!("Did not find ControllableInfo for name={name}");
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
