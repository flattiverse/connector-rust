use crate::hierarchy::GlaxyId;
use crate::hierarchy::{ClusterId, RegionConfig};
use crate::network::{ConnectionHandle, PacketReader};
use crate::{GameError, Indexer, NamedUnit};
use std::future::Future;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, derive_more::From)]
pub struct RegionId(pub(crate) u8);

impl Indexer for RegionId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

pub struct Region {
    galaxy: GlaxyId,
    cluster: ClusterId,
    id: RegionId,
    name: String,
    start_probability: f64,
    respawn_probability: f64,
    protected: bool,
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
            name: reader.read_string(),
            start_probability: reader.read_2u(100.0),
            respawn_probability: reader.read_2u(100.0),
            protected: reader.read_boolean(),
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
        &self.name
    }

    #[inline]
    pub fn start_probability(&self) -> f64 {
        self.start_probability
    }

    #[inline]
    pub fn respawn_probability(&self) -> f64 {
        self.respawn_probability
    }

    #[inline]
    pub fn protected(&self) -> bool {
        self.protected
    }
}

impl NamedUnit for Region {
    #[inline]
    fn name(&self) -> &str {
        Region::name(self)
    }
}
