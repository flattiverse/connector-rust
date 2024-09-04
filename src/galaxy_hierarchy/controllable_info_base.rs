use crate::galaxy_hierarchy::{Galaxy, Indexer, Player};
use crate::runtime::Atomic;
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
pub struct ControllableInfoBase {
    galaxy: Weak<Galaxy>,
    player: Weak<Player>,
    id: ControllableInfoId,
    name: String,
    alive: Atomic<bool>,
    active: Atomic<bool>,
}

impl ControllableInfoBase {
    pub(crate) fn new(
        galaxy: Weak<Galaxy>,
        player: Weak<Player>,
        id: ControllableInfoId,
        name: String,
        alive: bool,
    ) -> Self {
        Self {
            galaxy,
            player,
            id,
            name,
            alive: Atomic::from(alive),
            active: Atomic::from(true),
        }
    }

    /// The id of this ControllableInfo.
    #[inline]
    pub fn id(&self) -> ControllableInfoId {
        self.id
    }

    /// The name of this controllable.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

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

    pub(crate) fn deactivate(&self) {
        self.active.store(false);
        self.alive.store(false);
    }

    pub(crate) fn set_alive(&self) {
        self.alive.store(true);
    }

    pub(crate) fn set_dead(&self) {
        self.alive.store(false);
    }
}
