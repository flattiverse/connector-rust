use crate::galaxy_hierarchy::{Galaxy, Identifiable, Indexer, NamedUnit};
use crate::unit::Unit;
use crate::utils::Atomic;
use crate::utils::GuardedArcStringDeref;
use arc_swap::ArcSwap;
use flashmap::{ReadHandle, WriteHandle};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::{Arc, Mutex, Weak};

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
    active: Atomic<bool>,
    units: (
        Mutex<WriteHandle<String, Arc<Unit>>>,
        ReadHandle<String, Arc<Unit>>,
    ),
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
    pub fn new(galaxy: Weak<Galaxy>, id: ClusterId, name: impl Into<String>) -> Self {
        Self {
            galaxy,
            id,
            name: ArcSwap::new(Arc::new(name.into())),
            active: Atomic::from(true),
            units: {
                let (write, read) = flashmap::new();
                (Mutex::new(write), read)
            },
        }
    }

    pub(crate) fn update(&self, name: String) {
        self.name.store(Arc::new(name));
    }

    pub(crate) fn deactivate(&self) {
        self.active.store(false);
    }

    pub(crate) fn add_unit(&self, unit: Arc<Unit>) {
        let mut lock = self.units.0.lock().unwrap();
        let mut guard = lock.guard();
        guard.insert(unit.name().to_string(), unit);
        guard.publish();
    }

    pub(crate) fn remove_unit(&self, name: String) -> Option<Arc<Unit>> {
        let mut lock = self.units.0.lock().unwrap();
        let mut guard = lock.guard();
        let removed = guard.remove(name).map(|e| Arc::clone(e.deref()));
        guard.publish();
        removed
    }

    #[inline]
    pub fn get_unit(&self, unit: &str) -> Arc<Unit> {
        self.get_unit_opt(unit).unwrap()
    }

    #[inline]
    pub fn get_unit_opt(&self, unit: &str) -> Option<Arc<Unit>> {
        self.units.1.guard().get(unit).map(Arc::clone)
    }

    #[inline]
    pub fn get_units(&self) -> Vec<Arc<Unit>> {
        self.units.1.guard().values().cloned().collect()
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
    #[inline]
    fn name(&self) -> impl Deref<Target = str> + '_ {
        GuardedArcStringDeref(self.name.load())
    }
}
