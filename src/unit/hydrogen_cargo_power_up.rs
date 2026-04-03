use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractPowerUp, PowerUp, PowerUpInternal, SteadyUnit, SteadyUnitInternal, Unit, UnitCastTable,
    UnitHierarchy, UnitInternal, UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// A visible hydrogen cargo power-up.
#[derive(Debug, Clone)]
pub struct HydrogenCargoPowerUp {
    parent: AbstractPowerUp,
}

impl HydrogenCargoPowerUp {
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

impl UnitInternal for HydrogenCargoPowerUp {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitCastTable for HydrogenCargoPowerUp {
    cast_fn!(steady_unit_cast_fn, HydrogenCargoPowerUp, dyn SteadyUnit);
    cast_fn!(power_up_cast_fn, HydrogenCargoPowerUp, dyn PowerUp);
}

impl UnitHierarchy for HydrogenCargoPowerUp {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_power_up(&self) -> Option<&dyn PowerUp> {
        Some(self)
    }

    #[inline]
    fn as_hydrogen_cargo_power_up(&self) -> Option<&HydrogenCargoPowerUp> {
        Some(self)
    }
}

impl Unit for HydrogenCargoPowerUp {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::HydrogenCargoPowerUp
    }
}

impl SteadyUnitInternal for HydrogenCargoPowerUp {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for HydrogenCargoPowerUp {}

impl PowerUpInternal for HydrogenCargoPowerUp {
    #[inline]
    fn parent(&self) -> &dyn PowerUp {
        &self.parent
    }
}

impl PowerUp for HydrogenCargoPowerUp {}
