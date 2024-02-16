use crate::hierarchy::{GlaxyId, Region, RegionId};
use crate::network::PacketReader;
use crate::{Indexer, NamedUnit, UniversalHolder};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, derive_more::From)]
pub struct ClusterId(u8);

impl Indexer for ClusterId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

pub struct Cluster {
    id: ClusterId,
    galaxy: GlaxyId,
    name: String,
    regions: UniversalHolder<RegionId, Region>,
}

impl Cluster {
    #[inline]
    pub fn new(id: impl Into<ClusterId>, galaxy: GlaxyId, reader: &mut dyn PacketReader) -> Self {
        Self {
            id: id.into(),
            galaxy,
            name: reader.read_string(),
            regions: UniversalHolder::with_capacity(256),
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

    #[inline]
    pub fn regions(&self) -> &UniversalHolder<RegionId, Region> {
        &self.regions
    }
}

impl NamedUnit for Cluster {
    #[inline]
    fn name(&self) -> &str {
        Cluster::name(self)
    }
}
