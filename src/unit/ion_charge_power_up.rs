use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractPowerUp, PowerUp, PowerUpInternal, SteadyUnit, SteadyUnitInternal, Unit, UnitCastTable,
    UnitHierarchy, UnitInternal, UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// A visible ion charge power-up.
#[derive(Debug, Clone)]
pub struct IonChargePowerUp {
    parent: AbstractPowerUp,
}

impl IonChargePowerUp {
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

impl UnitInternal for IonChargePowerUp {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitCastTable for IonChargePowerUp {
    cast_fn!(steady_unit_cast_fn, IonChargePowerUp, dyn SteadyUnit);
    cast_fn!(power_up_cast_fn, IonChargePowerUp, dyn PowerUp);
}

impl UnitHierarchy for IonChargePowerUp {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_power_up(&self) -> Option<&dyn PowerUp> {
        Some(self)
    }

    #[inline]
    fn as_ion_charge_power_up(&self) -> Option<&IonChargePowerUp> {
        Some(self)
    }
}

impl Unit for IonChargePowerUp {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::IonChargePowerUp
    }
}

impl SteadyUnitInternal for IonChargePowerUp {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for IonChargePowerUp {}

impl PowerUpInternal for IonChargePowerUp {
    #[inline]
    fn parent(&self) -> &dyn PowerUp {
        &self.parent
    }
}

impl PowerUp for IonChargePowerUp {}
