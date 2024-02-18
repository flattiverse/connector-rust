use crate::hierarchy::{ClusterConfig, GlaxyId, Region, RegionConfig, RegionId};
use crate::network::{ConnectionHandle, PacketReader};
use crate::unit::configurations::SunConfiguration;
use crate::{GameError, Indexer, NamedUnit, UniversalHolder};
use std::future::Future;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ClusterId(pub(crate) u8);

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
    config: ClusterConfig,
    regions: UniversalHolder<RegionId, Region>,
    connection: ConnectionHandle,
}

impl Cluster {
    #[inline]
    pub fn new(
        id: impl Into<ClusterId>,
        galaxy: GlaxyId,
        connection: ConnectionHandle,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            id: id.into(),
            galaxy,
            connection,
            config: ClusterConfig::from(reader),
            regions: UniversalHolder::with_capacity(256),
        }
    }

    #[inline]
    pub(crate) fn read_region(&mut self, id: RegionId, reader: &mut dyn PacketReader) {
        self.regions.set(
            id,
            Region::new(self.galaxy, self.id, id, self.connection.clone(), reader),
        );
    }

    /// Sets the given values for this [`Cluster`].
    /// See also [`ConnectionHandle::configure_cluster`].
    pub async fn configure(
        &self,
        config: &ClusterConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection
            .configure_cluster_split(self.id, config)
            .await
    }

    /// Removes this [`Cluster`].
    /// See also [`ConnectionHandle::remove_cluster`].
    pub async fn remove(&self) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.remove_cluster_split(self.id).await
    }

    /// Creates a [`Region`] with the given values in this [`Cluster`].
    /// See also [`ConnectionHandle::create_region`].
    pub async fn create_region(
        &self,
        config: &RegionConfig,
    ) -> Result<impl Future<Output = Result<RegionId, GameError>>, GameError> {
        self.connection.create_region_split(self.id, config).await
    }

    /// Creates a [`Sun`] with the given values in this [`Cluster`].
    /// See also [`ConnectionHandle::create_sun`].
    pub async fn create_sun(
        &self,
        config: &SunConfiguration,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.create_sun_split(self.id, config).await
    }

    // TODO pub async fn create_sun
    // TODO pub async fn create_blackhole
    // TODO pub async fn create_planet
    // TODO pub async fn create_moon
    // TODO pub async fn create_meteoroid
    // TODO pub async fn create_buoy

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
        &self.config.name
    }

    #[inline]
    pub fn config(&self) -> &ClusterConfig {
        &self.config
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
