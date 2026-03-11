use crate::galaxy_hierarchy::{Cluster, ClusterId};

/// Snapshot of a cluster state relevant for events.
#[derive(Debug, Clone)]
pub struct ClusterSnapshot {
    /// Cluster id.
    pub id: ClusterId,
    /// Cluster name.
    pub name: String,
    /// Cluster activity flag.
    pub active: bool,
}

impl From<&Cluster> for ClusterSnapshot {
    fn from(cluster: &Cluster) -> Self {
        Self {
            id: cluster.id(),
            name: cluster.name().to_string(),
            active: cluster.active(),
        }
    }
}
