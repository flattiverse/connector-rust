use crate::hierarchy::{ConnectionProvider, Galaxy, ShipDesign, ShipUpgradeConfig};
use crate::network::PacketReader;
use crate::{GameError, Identifiable, Indexer, NamedUnit};
use std::sync::{Arc, Weak};

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
    galaxy: Weak<Galaxy>,
    ship_design: Arc<ShipDesign>,
    id: ShipUpgradeId,
    config: ShipUpgradeConfig,
}

impl ShipUpgrade {
    pub fn new(
        galaxy: Weak<Galaxy>,
        ship: Arc<ShipDesign>,
        id: impl Into<ShipUpgradeId>,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            galaxy,
            ship_design: ship,
            id: id.into(),
            config: ShipUpgradeConfig::from(reader),
        }
    }

    /// Sets the given values for this [`ShipUpgrade`].
    /// See also [`ConnectionHandle::configure_upgrade`].
    #[inline]
    pub async fn configure(&self, config: &ShipUpgradeConfig) -> Result<(), GameError> {
        self.galaxy
            .connection()?
            .configure_upgrade(self.id, self.ship_design.id(), config)
            .await
    }

    /// Removes this [`ShipUpgrade`].
    /// See also [`ConnectionHandle::remove_upgrade`].
    #[inline]
    pub async fn remove(&self) -> Result<(), GameError> {
        self.galaxy
            .connection()?
            .remove_upgrade(self.id, self.ship_design.id())
            .await
    }

    #[inline]
    pub fn galaxy(&self) -> &Weak<Galaxy> {
        &self.galaxy
    }

    #[inline]
    pub fn ship_design(&self) -> &Arc<ShipDesign> {
        &self.ship_design
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

impl Identifiable<ShipUpgradeId> for ShipUpgrade {
    #[inline]
    fn id(&self) -> ShipUpgradeId {
        self.id
    }
}

impl NamedUnit for ShipUpgrade {
    #[inline]
    fn name(&self) -> &str {
        ShipUpgrade::name(self)
    }
}
