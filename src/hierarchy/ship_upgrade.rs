use crate::hierarchy::{GalaxyId, ShipDesignId, ShipUpgradeConfig};
use crate::network::{ConnectionHandle, PacketReader};
use crate::{GameError, Indexer, NamedUnit};
use std::future::Future;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ShipUpgradeId(pub(crate) u8);

impl Indexer for ShipUpgradeId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct ShipUpgrade {
    galaxy: GalaxyId,
    ship_design: ShipDesignId,
    id: ShipUpgradeId,
    config: ShipUpgradeConfig,
    connection: ConnectionHandle,
}

impl ShipUpgrade {
    pub fn new(
        id: impl Into<ShipUpgradeId>,
        galaxy: GalaxyId,
        ship: ShipDesignId,
        connection: ConnectionHandle,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            id: id.into(),
            galaxy,
            ship_design: ship,
            config: ShipUpgradeConfig::from(reader),
            connection,
        }
    }

    /// Sets the given values for this [`ShipUpgrade`].
    /// See also [`ConnectionHandle::configure_upgrade`].
    #[inline]
    pub async fn configure(
        &self,
        config: &ShipUpgradeConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection
            .configure_upgrade_split(self.id, config)
            .await
    }

    /// Removes this [`ShipUpgrade`].
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
    pub fn ship_design(&self) -> ShipDesignId {
        self.ship_design
    }

    #[inline]
    pub fn id(&self) -> ShipUpgradeId {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.config.name
    }

    #[inline]
    pub fn config(&self) -> &ShipUpgradeConfig {
        &self.config
    }
}

impl NamedUnit for ShipUpgrade {
    #[inline]
    fn name(&self) -> &str {
        ShipUpgrade::name(self)
    }
}
