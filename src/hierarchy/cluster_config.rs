use crate::hierarchy::Cluster;
use crate::network::PacketWriter;

#[derive(Debug, Clone, Default)]
pub struct ClusterConfig {
    pub name: String,
}

impl From<&Cluster> for ClusterConfig {
    fn from(cluster: &Cluster) -> Self {
        Self {
            name: cluster.name().to_string(),
        }
    }
}

impl ClusterConfig {
    #[inline]
    pub(crate) fn write_to(&self, writer: &mut dyn PacketWriter) {
        writer.write_string(&self.name);
    }
}