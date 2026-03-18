use crate::galaxy_hierarchy::{Galaxy, Identifiable, Indexer, NamedUnit};
use crate::unit::Unit;
use crate::utils::Atomic;
use crate::utils::GuardedArcStringDeref;
use arc_swap::ArcSwap;
use crossbeam_skiplist::SkipMap;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::{Arc, Weak};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ClusterId(pub(crate) u8);

impl Indexer for ClusterId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

/// This is a subset of the galaxy. Each cluster is a separate map.
pub struct Cluster {
    id: ClusterId,
    galaxy: Weak<Galaxy>,
    name: ArcSwap<String>,
    start: Atomic<bool>,
    respawn: Atomic<bool>,
    active: Atomic<bool>,
    units: SkipMap<String, Arc<Unit>>,
}

impl Debug for Cluster {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cluster")
            .field("id", &self.id)
            .field("galaxy", &self.galaxy)
            .field("name", &self.name)
            .field("active", &self.active)
            .finish_non_exhaustive()
    }
}

impl Cluster {
    pub fn new(
        galaxy: Weak<Galaxy>,
        id: ClusterId,
        name: impl Into<String>,
        start: bool,
        respawn: bool,
    ) -> Self {
        Self {
            galaxy,
            id,
            name: ArcSwap::new(Arc::new(name.into())),
            start: Atomic::from(start),
            respawn: Atomic::from(respawn),
            active: Atomic::from(true),
            units: SkipMap::new(),
        }
    }

    pub(crate) fn update(&self, name: String, start: bool, respawn: bool) {
        self.name.store(Arc::new(name));
        self.start.store(start);
        self.respawn.store(respawn);
        self.active.store(true);
    }

    pub(crate) fn deactivate(&self) {
        self.active.store(false);
    }

    pub(crate) fn add_unit(&self, unit: Arc<Unit>) {
        let name = NamedUnit::name(&*unit).to_string();
        self.units.insert(name, unit);
    }

    pub(crate) fn remove_unit(&self, name: &str) -> Option<Arc<Unit>> {
        self.units.remove(name).map(|e| Arc::clone(e.value()))
    }

    #[inline]
    pub fn get_unit(&self, unit: &str) -> Option<Arc<Unit>> {
        self.units.get(unit).map(|e| Arc::clone(e.value()))
    }

    #[inline]
    pub fn iter_units(&self) -> impl Iterator<Item = Arc<Unit>> + '_ {
        self.units.iter().map(|e| Arc::clone(e.value()))
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

    /// If true, freshly registered ships spawn in this cluster.
    #[inline]
    pub fn start(&self) -> bool {
        self.start.load()
    }

    /// If true, [`Controllable::r#continue`] spawns in this cluster.
    ///
    /// [`Controllable::r#continue`]: crate::galaxy_hierarchy::Controllable::r#continue
    #[inline]
    pub fn respawn(&self) -> bool {
        self.respawn.load()
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
    #[inline]
    fn name(&self) -> impl Deref<Target = str> + '_ {
        GuardedArcStringDeref(self.name.load())
    }
}

impl Hash for Unit {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        NamedUnit::name(self).hash(state)
    }
}

impl Eq for Unit {}

impl PartialEq<Self> for Unit {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        *NamedUnit::name(self) == *NamedUnit::name(other)
    }
}

unsafe impl Sync for Cluster {}
