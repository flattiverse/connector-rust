use crate::hierarchy::{GlaxyId, ShipConfig, UpgradeConfig};
use crate::network::{ConnectionHandle, PacketReader};
use crate::{GameError, Indexer, NamedUnit, UniversalHolder, Upgrade, UpgradeId};
use std::future::Future;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ShipId(pub(crate) u8);

impl Indexer for ShipId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct Ship {
    galaxy: GlaxyId,
    id: ShipId,
    upgrades: UniversalHolder<UpgradeId, Upgrade>,
    config: ShipConfig,
    connection: ConnectionHandle,
}

impl Ship {
    pub fn new(
        id: impl Into<ShipId>,
        galaxy: GlaxyId,
        connection: ConnectionHandle,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            id: id.into(),
            galaxy,
            upgrades: UniversalHolder::with_capacity(256),
            config: ShipConfig::from(reader),
            connection,
        }
    }

    pub(crate) fn read_upgrade(&mut self, id: UpgradeId, reader: &mut dyn PacketReader) {
        self.upgrades.set(
            id,
            Upgrade::new(id, self.galaxy, self.id, self.connection.clone(), reader),
        );
    }

    /// Sets the given values for this [`Ship`].
    /// See also [`ConnectionHandle::configure_ship`].
    #[inline]
    pub async fn configure(
        &self,
        config: &ShipConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.configure_ship_split(self.id, config).await
    }

    /// Removes this [`Ship`].
    /// See also [`ConnectionHandle::remove_ship`].
    #[inline]
    pub async fn remove(&self) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.remove_ship_split(self.id).await
    }

    /// Creates an [`Upgrade`] with the given values for this [`Ship`].
    /// See also [`ConnectionHandle::create_upgrade`]
    #[inline]
    pub async fn create_upgrade(
        &self,
        config: &UpgradeConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.create_upgrade_split(self.id, config).await
    }

    #[inline]
    pub fn id(&self) -> ShipId {
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
    pub fn config(&self) -> &ShipConfig {
        &self.config
    }

    #[inline]
    pub fn get_upgrade(&self, id: UpgradeId) -> Option<&Upgrade> {
        self.upgrades.get(id)
    }
}

impl NamedUnit for Ship {
    #[inline]
    fn name(&self) -> &str {
        Ship::name(self)
    }
}
