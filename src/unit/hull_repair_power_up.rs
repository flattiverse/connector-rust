use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractPowerUp, PowerUp, PowerUpInternal, SteadyUnit, SteadyUnitInternal, Unit, UnitHierarchy,
    UnitInternal, UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// A visible hull repair power-up.
#[derive(Debug, Clone)]
pub struct HullRepairPowerUp {
    parent: AbstractPowerUp,
}

impl HullRepairPowerUp {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractPowerUp::new(cluster, name.clone(), reader)?,
        }))
    }
}

impl UnitInternal for HullRepairPowerUp {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitHierarchy for HullRepairPowerUp {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_power_up(&self) -> Option<&dyn PowerUp> {
        Some(self)
    }

    #[inline]
    fn as_hull_repair_power_up(&self) -> Option<&HullRepairPowerUp> {
        Some(self)
    }
}

impl Unit for HullRepairPowerUp {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::HullRepairPowerUp
    }
}

impl SteadyUnitInternal for HullRepairPowerUp {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for HullRepairPowerUp {}

impl PowerUpInternal for HullRepairPowerUp {
    #[inline]
    fn parent(&self) -> &dyn PowerUp {
        &self.parent
    }
}

impl PowerUp for HullRepairPowerUp {}
