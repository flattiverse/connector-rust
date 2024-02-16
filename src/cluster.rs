use crate::network::PacketReader;
use crate::{GlaxyId, Indexer, NamedUnit};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, derive_more::From)]
pub struct ClusterId(u8);

impl Indexer for ClusterId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct Cluster {
    id: ClusterId,
    galaxy: GlaxyId,
    name: String,
}

impl Cluster {
    #[inline]
    pub fn new(id: impl Into<ClusterId>, galaxy: GlaxyId, reader: &mut dyn PacketReader) -> Self {
        Self {
            id: id.into(),
            galaxy,
            name: reader.read_string(),
        }
    }

    #[inline]
    pub fn id(&self) -> ClusterId {
        self.id
    }

    #[inline]
    pub fn galaxy(&self) -> GlaxyId {
        self.galaxy
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl NamedUnit for Cluster {
    #[inline]
    fn name(&self) -> &str {
        Cluster::name(self)
    }
}
