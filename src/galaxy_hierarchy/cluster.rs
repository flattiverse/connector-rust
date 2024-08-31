use crate::galaxy_hierarchy::{Identifiable, Indexer, NamedUnit};
use crate::runtime::Atomic;
use std::ops::Deref;
use std::sync::RwLock;

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
    /// The id within the galaxy of the cluster.
    pub id: ClusterId,
    name: RwLock<String>,
    active: Atomic<bool>,
}

impl Cluster {
    pub fn new(id: ClusterId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: RwLock::new(name.into()),
            active: Atomic::from(true),
        }
    }

    pub(crate) fn update(&self, name: String) {
        *self.name.write().unwrap() = name;
    }

    pub(crate) fn deactivate(&self) {
        self.active.store(false);
    }

    /// If false, you have been disconnected or the cluster has been removed and therefore disabled.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }
}

impl Identifiable<ClusterId> for Cluster {
    #[inline]
    fn id(&self) -> ClusterId {
        self.id
    }
}

impl NamedUnit for Cluster {
    fn name<'a>(&'a self) -> impl Deref<Target = str> + 'a {
        self.name.read().unwrap().clone()
    }
}
