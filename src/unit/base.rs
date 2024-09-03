use crate::galaxy_hierarchy::Cluster;
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub struct UnitBase {
    name: String,
    cluster: Weak<Cluster>,
}

impl UnitBase {
    pub(crate) fn new(cluster: Weak<Cluster>, name: String) -> Self {
        Self { cluster, name }
    }

    /// The name of the unit. A unit can't change her name after it has been set up.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The cluster the unit is in.
    #[inline]
    pub fn cluster(&self) -> Arc<Cluster> {
        self.cluster.upgrade().unwrap()
    }
}
