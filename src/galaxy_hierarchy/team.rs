use crate::galaxy_hierarchy::{Identifiable, Indexer, NamedUnit};
use crate::runtime::Atomic;
use std::ops::Deref;
use std::sync::RwLock;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct TeamId(pub(crate) u8);

impl Indexer for crate::galaxy_hierarchy::TeamId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

/// Represents a team.
#[derive(Debug)]
pub struct Team {
    /// The id of the team
    pub id: TeamId,
    /// The name of the team.
    name: RwLock<String>,
    red: Atomic<u8>,
    green: Atomic<u8>,
    blue: Atomic<u8>,
    active: Atomic<bool>,
}

impl Team {
    pub fn new(id: TeamId, name: impl Into<String>, red: u8, green: u8, blue: u8) -> Team {
        Self {
            id,
            name: RwLock::new(name.into()),
            red: Atomic::from(red),
            green: Atomic::from(green),
            blue: Atomic::from(blue),
            active: Atomic::from(true),
        }
    }

    pub fn update(&self, name: String, red: u8, green: u8, blue: u8) {
        *self.name.write().unwrap() = name;
        self.red.store(red);
        self.green.store(green);
        self.blue.store(blue);
    }

    pub fn deactivate(&self) {
        self.active.store(false);
    }

    /// The red part of the team color.
    #[inline]
    pub fn red(&self) -> u8 {
        self.red.load()
    }

    /// The green part of the team color.
    #[inline]
    pub fn green(&self) -> u8 {
        self.green.load()
    }

    /// The blue part of the team color.
    #[inline]
    pub fn blue(&self) -> u8 {
        self.blue.load()
    }

    /// True as long as the team is active.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }
}

impl Identifiable<TeamId> for Team {
    #[inline]
    fn id(&self) -> TeamId {
        self.id
    }
}

impl NamedUnit for Team {
    fn name(&self) -> impl Deref<Target = str> {
        self.name.read().unwrap().clone()
    }
}
