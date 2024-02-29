use crate::atomics::Atomic;
use crate::hierarchy::{Cluster, Galaxy};
use crate::hierarchy::{ConnectionProvider, RegionConfig};
use crate::network::PacketReader;
use crate::{GameError, Identifiable, Indexer};
use arc_swap::ArcSwap;
use std::ops::Deref;
use std::sync::{Arc, Weak};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct RegionId(pub(crate) u8);

impl Indexer for RegionId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct Region {
    active: Atomic<bool>,
    galaxy: Weak<Galaxy>,
    cluster: Arc<Cluster>,
    id: RegionId,
    config: ArcSwap<RegionConfig>,
}

impl Region {
    pub fn new(
        galaxy: Weak<Galaxy>,
        cluster: Arc<Cluster>,
        id: RegionId,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            active: Atomic::from(true),
            galaxy,
            cluster,
            id,
            config: ArcSwap::new(Arc::new(RegionConfig::from(reader))),
        }
    }

    pub(crate) fn update(&self, reader: &mut dyn PacketReader) {
        self.config.store(Arc::new(RegionConfig::from(reader)));
    }

    pub(crate) fn deactivate(&self) {
        self.active.store(false);
    }

    /// Sets the given values for this [`Region`].
    /// See also [`ConnectionHandle::configure_region`].
    #[inline]
    pub async fn configure(&self, config: &RegionConfig) -> Result<(), GameError> {
        self.galaxy
            .connection()?
            .configure_region(self.id, config)
            .await
    }

    /// Removes this [`Region`].
    /// See also [`ConnectionHandle::remove_region`].
    #[inline]
    pub async fn remove(&self) -> Result<(), GameError> {
        self.galaxy.connection()?.remove_region(self.id).await
    }

    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    #[inline]
    pub fn galaxy(&self) -> &Weak<Galaxy> {
        &self.galaxy
    }

    #[inline]
    pub fn cluster(&self) -> &Arc<Cluster> {
        &self.cluster
    }

    #[inline]
    pub fn id(&self) -> RegionId {
        self.id
    }

    #[inline]
    pub fn config(&self) -> impl Deref<Target = Arc<RegionConfig>> {
        self.config.load()
    }
}

impl Identifiable<RegionId> for Region {
    #[inline]
    fn id(&self) -> RegionId {
        self.id
    }
}
