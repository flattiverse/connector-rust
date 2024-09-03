use crate::galaxy_hierarchy::{Galaxy, Identifiable, Indexer, NamedUnit, UniversalArcHolder};
use crate::runtime::Atomic;
use crate::unit::Unit;
use std::ops::Deref;
use std::sync::{Arc, RwLock, Weak};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ClusterId(pub(crate) u8);

impl Indexer for ClusterId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

/// This is a subset of the galaxy. Each cluster is a separate map.
#[derive(Debug)]
pub struct Cluster {
    id: ClusterId,
    galaxy: Weak<Galaxy>,
    name: RwLock<String>,
    active: Atomic<bool>,
    units: UniversalArcHolder<(), Unit>,
}

impl Cluster {
    pub fn new(galaxy: Weak<Galaxy>, id: ClusterId, name: impl Into<String>) -> Self {
        Self {
            galaxy,
            id,
            name: RwLock::new(name.into()),
            active: Atomic::from(true),
            units: UniversalArcHolder::with_capacity(1024 * 1024),
        }
    }

    pub(crate) fn update(&self, name: String) {
        *self.name.write().unwrap() = name;
    }

    pub(crate) fn deactivate(&self) {
        self.active.store(false);
    }

    pub(crate) fn add_unit(&self, unit: Arc<Unit>) {
        self.units.push(unit);
    }

    pub(crate) fn remove_unit(&self, name: &str) -> Arc<Unit> {
        self.units.remove_by_name(name)
    }

    pub fn units(&self) -> impl Iterator<Item = Arc<Unit>> + '_ {
        self.units.iter()
    }

    /// The id within the galaxy of the cluster.
    #[inline]
    pub fn id(&self) -> ClusterId {
        self.id
    }

    /// The name of the cluster.
    #[inline]
    pub fn name(&self) -> impl Deref<Target = str> + '_ {
        NamedUnit::name(self)
    }

    /// If false, you have been disconnected or the cluster has been removed and therefore disabled.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    #[inline]
    pub fn galaxy(&self) -> Arc<Galaxy> {
        self.galaxy.upgrade().unwrap()
    }
}

impl Identifiable<ClusterId> for Cluster {
    #[inline]
    fn id(&self) -> ClusterId {
        self.id
    }
}

impl NamedUnit for Cluster {
    fn name(&self) -> impl Deref<Target = str> + '_ {
        self.name.read().unwrap().clone()
    }
}
