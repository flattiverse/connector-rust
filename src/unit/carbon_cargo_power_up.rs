use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractPowerUp, PowerUp, PowerUpInternal, SteadyUnit, SteadyUnitInternal, Unit, UnitHierarchy,
    UnitInternal, UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// A visible carbon cargo power-up.
#[derive(Debug, Clone)]
pub struct CarbonCargoPowerUp {
    parent: AbstractPowerUp,
}

impl CarbonCargoPowerUp {
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

impl UnitInternal for CarbonCargoPowerUp {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitHierarchy for CarbonCargoPowerUp {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_power_up(&self) -> Option<&dyn PowerUp> {
        Some(self)
    }

    #[inline]
    fn as_carbon_cargo_power_up(&self) -> Option<&CarbonCargoPowerUp> {
        Some(self)
    }
}

impl Unit for CarbonCargoPowerUp {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::CarbonCargoPowerUp
    }
}

impl SteadyUnitInternal for CarbonCargoPowerUp {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for CarbonCargoPowerUp {}

impl PowerUpInternal for CarbonCargoPowerUp {
    #[inline]
    fn parent(&self) -> &dyn PowerUp {
        &self.parent
    }
}

impl PowerUp for CarbonCargoPowerUp {}
