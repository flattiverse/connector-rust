use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractPowerUp, PowerUp, PowerUpInternal, SteadyUnit, SteadyUnitInternal, Unit, UnitHierarchy,
    UnitInternal, UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// A visible neutrino charge power-up.
#[derive(Debug, Clone)]
pub struct NeutrinoChargePowerUp {
    parent: AbstractPowerUp,
}

impl NeutrinoChargePowerUp {
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

impl UnitInternal for NeutrinoChargePowerUp {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitHierarchy for NeutrinoChargePowerUp {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_power_up(&self) -> Option<&dyn PowerUp> {
        Some(self)
    }

    #[inline]
    fn as_neutrino_charge_power_up(&self) -> Option<&NeutrinoChargePowerUp> {
        Some(self)
    }
}

impl Unit for NeutrinoChargePowerUp {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::NeutrinoChargePowerUp
    }
}

impl SteadyUnitInternal for NeutrinoChargePowerUp {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for NeutrinoChargePowerUp {}

impl PowerUpInternal for NeutrinoChargePowerUp {
    #[inline]
    fn parent(&self) -> &dyn PowerUp {
        &self.parent
    }
}

impl PowerUp for NeutrinoChargePowerUp {}
