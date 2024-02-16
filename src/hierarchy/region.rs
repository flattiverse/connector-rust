use crate::hierarchy::ClusterId;
use crate::hierarchy::GlaxyId;
use crate::{Indexer, NamedUnit};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, derive_more::From)]
pub struct RegionId(u8);

impl Indexer for RegionId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct Region {
    pub(crate) galaxy: GlaxyId,
    pub(crate) cluster: ClusterId,
    pub(crate) id: RegionId,
    pub(crate) name: String,
}

impl Region {
    #[inline]
    pub fn galaxy(&self) -> GlaxyId {
        self.galaxy
    }

    #[inline]
    pub fn cluster(&self) -> ClusterId {
        self.cluster
    }

    #[inline]
    pub fn id(&self) -> RegionId {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl NamedUnit for Region {
    #[inline]
    fn name(&self) -> &str {
        Region::name(self)
    }
}
