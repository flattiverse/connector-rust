use crate::hierarchy::{
    ConnectionProvider, Galaxy, ShipDesignConfig, ShipUpgrade, ShipUpgradeConfig, ShipUpgradeId,
};
use crate::network::PacketReader;
use crate::{GameError, Identifiable, Indexer, NamedUnit, UniversalArcHolder};
use std::sync::{Arc, Weak};

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
    galaxy: Weak<Galaxy>,
    upgrades: UniversalArcHolder<ShipUpgradeId, ShipUpgrade>,
    id: ShipDesignId,
    config: ShipDesignConfig,
}

impl ShipDesign {
    pub fn new(
        galaxy: Weak<Galaxy>,
        id: impl Into<ShipDesignId>,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            galaxy,
            id: id.into(),
            config: ShipDesignConfig::from(reader),
            upgrades: UniversalArcHolder::with_capacity(256),
        }
    }

    /// Sets the given values for this [`ShipDesign`].
    /// See also [`ConnectionHandle::configure_ship`].
    #[inline]
    pub async fn configure(&self, config: &ShipDesignConfig) -> Result<(), GameError> {
        self.galaxy
            .connection()?
            .configure_ship(self.id, config)
            .await
    }

    /// Removes this [`ShipDesign`].
    /// See also [`ConnectionHandle::remove_ship`].
    #[inline]
    pub async fn remove(&self) -> Result<(), GameError> {
        self.galaxy.connection()?.remove_ship(self.id).await
    }

    /// Creates an [`ShipUpgrade`] with the given values for this [`ShipDesign`].
    /// See also [`ConnectionHandle::create_upgrade`]
    #[inline]
    pub async fn create_upgrade(
        &self,
        config: &ShipUpgradeConfig,
    ) -> Result<Arc<ShipUpgrade>, GameError> {
        Ok(self.upgrades.get(
            self.galaxy
                .connection()?
                .create_upgrade(self.id, config)
                .await?,
        ))
    }

    #[inline]
    pub fn id(&self) -> ShipDesignId {
        self.id
    }

    #[inline]
    pub fn galaxy(&self) -> &Weak<Galaxy> {
        &self.galaxy
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
    pub fn upgrades(&self) -> &UniversalArcHolder<ShipUpgradeId, ShipUpgrade> {
        &self.upgrades
    }
}

impl Identifiable<ShipDesignId> for ShipDesign {
    #[inline]
    fn id(&self) -> ShipDesignId {
        self.id
    }
}

impl NamedUnit for ShipDesign {
    #[inline]
    fn name(&self) -> &str {
        ShipDesign::name(self)
    }
}
