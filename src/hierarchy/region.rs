use crate::hierarchy::GlaxyId;
use crate::hierarchy::{ClusterId, RegionConfig};
use crate::network::{ConnectionHandle, PacketReader};
use crate::{GameError, Indexer, NamedUnit};
use std::future::Future;

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
    galaxy: GlaxyId,
    cluster: ClusterId,
    id: RegionId,
    config: RegionConfig,
    connection: ConnectionHandle,
}

impl Region {
    pub fn new(
        galaxy: GlaxyId,
        cluster: ClusterId,
        id: RegionId,
        connection: ConnectionHandle,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            galaxy,
            cluster,
            id,
            config: RegionConfig::from(reader),
            connection,
        }
    }

    /// Sets the given values for this [`Region`].
    /// See also [`ConnectionHandle::configure_region`].
    #[inline]
    pub async fn configure(
        &self,
        config: &RegionConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection
            .configure_region_split(self.id, config)
            .await
    }

    /// Removes this [`Region`].
    /// See also [`ConnectionHandle::remove_region`].
    #[inline]
    pub async fn remove(&self) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.remove_region_split(self.id).await
    }

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
        &self.config.name
    }

    #[inline]
    pub fn config(&self) -> &RegionConfig {
        &self.config
    }
}

impl NamedUnit for Region {
    #[inline]
    fn name(&self) -> &str {
        Region::name(self)
    }
}
