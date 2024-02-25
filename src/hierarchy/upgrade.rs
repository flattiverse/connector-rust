use crate::hierarchy::{GalaxyId, ShipDesignId, UpgradeConfig};
use crate::network::{ConnectionHandle, PacketReader};
use crate::{GameError, Indexer, NamedUnit};
use std::future::Future;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct UpgradeId(pub(crate) u8);

impl Indexer for UpgradeId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct Upgrade {
    galaxy: GalaxyId,
    ship: ShipDesignId,
    id: UpgradeId,
    config: UpgradeConfig,
    connection: ConnectionHandle,
}

impl Upgrade {
    pub fn new(
        id: impl Into<UpgradeId>,
        galaxy: GalaxyId,
        ship: ShipDesignId,
        connection: ConnectionHandle,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            id: id.into(),
            galaxy,
            ship,
            config: UpgradeConfig::from(reader),
            connection,
        }
    }

    /// Sets the given values for this [`Upgrade`].
    /// See also [`ConnectionHandle::configure_upgrade`].
    #[inline]
    pub async fn configure(
        &self,
        config: &UpgradeConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection
            .configure_upgrade_split(self.id, config)
            .await
    }

    /// Removes this [`Upgrade`].
    /// See also [`ConnectionHandle::remove_upgrade`].
    #[inline]
    pub async fn remove(&self) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.remove_upgrade_split(self.id).await
    }

    #[inline]
    pub fn galaxy(&self) -> GalaxyId {
        self.galaxy
    }

    #[inline]
    pub fn ship(&self) -> ShipDesignId {
        self.ship
    }

    #[inline]
    pub fn id(&self) -> UpgradeId {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.config.name
    }

    #[inline]
    pub fn config(&self) -> &UpgradeConfig {
        &self.config
    }
}

impl NamedUnit for Upgrade {
    #[inline]
    fn name(&self) -> &str {
        Upgrade::name(self)
    }
}
