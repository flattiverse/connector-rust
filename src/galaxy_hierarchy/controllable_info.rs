use crate::galaxy_hierarchy::{Galaxy, Identifiable, Indexer, NamedUnit, Player};
use crate::runtime::Atomic;
use crate::unit::UnitKind;
use std::ops::Deref;
use std::sync::{Arc, Weak};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ControllableInfoId(pub(crate) u8);

impl Indexer for ControllableInfoId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct ControllableInfo {
    galaxy: Weak<Galaxy>,
    player: Weak<Player>,
    /// The id of this ControllableInfo.
    pub id: ControllableInfoId,
    /// The name of this controllable.
    pub name: String,
    alive: Atomic<bool>,
    active: Atomic<bool>,
}

impl ControllableInfo {
    /// The galaxy instance this ControllableInfo belongs to.
    #[inline]
    pub fn galaxy(&self) -> Arc<Galaxy> {
        self.galaxy.upgrade().unwrap()
    }

    /// The player this ControllableInfo belongs to.
    #[inline]
    pub fn player(&self) -> Arc<Player> {
        self.player.upgrade().unwrap()
    }

    /// true, if the corresponding PlayerUnit is alive.
    #[inline]
    pub fn alive(&self) -> bool {
        self.alive.load()
    }

    /// true, if the corresponding PlayerUnit is still in use.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    /// Specifies the kind of the PlayerUnit.
    pub fn kind(&self) -> UnitKind {
        todo!()
    }
}

impl Identifiable<ControllableInfoId> for ControllableInfo {
    #[inline]
    fn id(&self) -> ControllableInfoId {
        self.id
    }
}

impl NamedUnit for ControllableInfo {
    #[inline]
    fn name(&self) -> impl Deref<Target = str> + '_ {
        self.name.as_str()
    }
}
