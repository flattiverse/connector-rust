use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractPowerUp, PowerUp, PowerUpInternal, SteadyUnit, SteadyUnitInternal, Unit, UnitHierarchy,
    UnitInternal, UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// A visible shot charge power-up.
#[derive(Debug, Clone)]
pub struct ShotChargePowerUp {
    parent: AbstractPowerUp,
}

impl ShotChargePowerUp {
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

impl UnitInternal for ShotChargePowerUp {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitHierarchy for ShotChargePowerUp {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_power_up(&self) -> Option<&dyn PowerUp> {
        Some(self)
    }

    #[inline]
    fn as_shot_charge_power_up(&self) -> Option<&ShotChargePowerUp> {
        Some(self)
    }
}

impl Unit for ShotChargePowerUp {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::ShotChargePowerUp
    }
}

impl SteadyUnitInternal for ShotChargePowerUp {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for ShotChargePowerUp {}

impl PowerUpInternal for ShotChargePowerUp {
    #[inline]
    fn parent(&self) -> &dyn PowerUp {
        &self.parent
    }
}

impl PowerUp for ShotChargePowerUp {}
