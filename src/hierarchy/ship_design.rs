use crate::hierarchy::{GlaxyId, ShipDesignConfig, Upgrade, UpgradeConfig, UpgradeId};
use crate::network::{ConnectionHandle, PacketReader};
use crate::{GameError, Indexer, NamedUnit, UniversalHolder};
use std::future::Future;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ShipDesignId(pub(crate) u8);

impl Indexer for ShipDesignId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct ShipDesign {
    galaxy: GlaxyId,
    id: ShipDesignId,
    upgrades: UniversalHolder<UpgradeId, Upgrade>,
    config: ShipDesignConfig,
    connection: ConnectionHandle,
}

impl ShipDesign {
    pub fn new(
        id: impl Into<ShipDesignId>,
        galaxy: GlaxyId,
        connection: ConnectionHandle,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            id: id.into(),
            galaxy,
            config: ShipDesignConfig::from(reader),
            upgrades: UniversalHolder::with_capacity(256),
            connection,
        }
    }

    pub(crate) fn read_upgrade(&mut self, id: UpgradeId, reader: &mut dyn PacketReader) {
        self.upgrades.set(
            id,
            Upgrade::new(id, self.galaxy, self.id, self.connection.clone(), reader),
        );
    }

    /// Sets the given values for this [`ShipDesign`].
    /// See also [`ConnectionHandle::configure_ship`].
    #[inline]
    pub async fn configure(
        &self,
        config: &ShipDesignConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.configure_ship_split(self.id, config).await
    }

    /// Removes this [`ShipDesign`].
    /// See also [`ConnectionHandle::remove_ship`].
    #[inline]
    pub async fn remove(&self) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.remove_ship_split(self.id).await
    }

    /// Creates an [`Upgrade`] with the given values for this [`ShipDesign`].
    /// See also [`ConnectionHandle::create_upgrade`]
    #[inline]
    pub async fn create_upgrade(
        &self,
        config: &UpgradeConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.create_upgrade_split(self.id, config).await
    }

    #[inline]
    pub fn id(&self) -> ShipDesignId {
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
    pub fn config(&self) -> &ShipDesignConfig {
        &self.config
    }

    #[inline]
    pub fn get_upgrade(&self, id: UpgradeId) -> Option<&Upgrade> {
        self.upgrades.get(id)
    }
}

impl NamedUnit for ShipDesign {
    #[inline]
    fn name(&self) -> &str {
        ShipDesign::name(self)
    }
}
