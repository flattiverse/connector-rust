use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractPowerUp, PowerUp, PowerUpInternal, SteadyUnit, SteadyUnitInternal, Unit, UnitHierarchy,
    UnitInternal, UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// A visible shield charge power-up.
#[derive(Debug, Clone)]
pub struct ShieldChargePowerUp {
    parent: AbstractPowerUp,
}

impl ShieldChargePowerUp {
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

impl UnitInternal for ShieldChargePowerUp {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitHierarchy for ShieldChargePowerUp {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_power_up(&self) -> Option<&dyn PowerUp> {
        Some(self)
    }

    #[inline]
    fn as_shield_charge_power_up(&self) -> Option<&ShieldChargePowerUp> {
        Some(self)
    }
}

impl Unit for ShieldChargePowerUp {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::ShieldChargePowerUp
    }
}

impl SteadyUnitInternal for ShieldChargePowerUp {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for ShieldChargePowerUp {}

impl PowerUpInternal for ShieldChargePowerUp {
    #[inline]
    fn parent(&self) -> &dyn PowerUp {
        &self.parent
    }
}

impl PowerUp for ShieldChargePowerUp {}
