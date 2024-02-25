use crate::hierarchy::{GalaxyId, ShipDesignConfig, ShipUpgrade, ShipUpgradeConfig, ShipUpgradeId};
use crate::network::{ConnectionHandle, PacketReader};
use crate::{GameError, Indexer, NamedUnit, UniversalHolder};
use std::future::Future;
use std::ops::{Index, IndexMut};

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
    galaxy: GalaxyId,
    id: ShipDesignId,
    upgrades: UniversalHolder<ShipUpgradeId, ShipUpgrade>,
    config: ShipDesignConfig,
    connection: ConnectionHandle,
}

impl ShipDesign {
    pub fn new(
        id: impl Into<ShipDesignId>,
        galaxy: GalaxyId,
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

    /// Creates an [`ShipUpgrade`] with the given values for this [`ShipDesign`].
    /// See also [`ConnectionHandle::create_upgrade`]
    #[inline]
    pub async fn create_upgrade(
        &self,
        config: &ShipUpgradeConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.create_upgrade_split(self.id, config).await
    }

    #[inline]
    pub fn id(&self) -> ShipDesignId {
        self.id
    }

    #[inline]
    pub fn galaxy(&self) -> GalaxyId {
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
    pub fn upgrades(&self) -> &UniversalHolder<ShipUpgradeId, ShipUpgrade> {
        &self.upgrades
    }

    #[inline]
    pub fn upgrades_mut(&mut self) -> &mut UniversalHolder<ShipUpgradeId, ShipUpgrade> {
        &mut self.upgrades
    }
}

impl Index<ShipUpgradeId> for ShipDesign {
    type Output = ShipUpgrade;

    #[inline]
    fn index(&self, index: ShipUpgradeId) -> &Self::Output {
        &self.upgrades[index]
    }
}

impl IndexMut<ShipUpgradeId> for ShipDesign {
    #[inline]
    fn index_mut(&mut self, index: ShipUpgradeId) -> &mut Self::Output {
        &mut self.upgrades[index]
    }
}

impl NamedUnit for ShipDesign {
    #[inline]
    fn name(&self) -> &str {
        ShipDesign::name(self)
    }
}
