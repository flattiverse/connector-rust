use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractPowerUp, PowerUp, PowerUpInternal, SteadyUnit, SteadyUnitInternal, Unit, UnitCastTable,
    UnitHierarchy, UnitInternal, UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// A visible metal cargo power-up.
#[derive(Debug, Clone)]
pub struct MetalCargoPowerUp {
    parent: AbstractPowerUp,
}

impl MetalCargoPowerUp {
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

impl UnitInternal for MetalCargoPowerUp {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitCastTable for MetalCargoPowerUp {
    cast_fn!(steady_unit_cast_fn, MetalCargoPowerUp, dyn SteadyUnit);
    cast_fn!(power_up_cast_fn, MetalCargoPowerUp, dyn PowerUp);
}

impl UnitHierarchy for MetalCargoPowerUp {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_power_up(&self) -> Option<&dyn PowerUp> {
        Some(self)
    }

    #[inline]
    fn as_metal_cargo_power_up(&self) -> Option<&MetalCargoPowerUp> {
        Some(self)
    }
}

impl Unit for MetalCargoPowerUp {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::MetalCargoPowerUp
    }
}

impl SteadyUnitInternal for MetalCargoPowerUp {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for MetalCargoPowerUp {}

impl PowerUpInternal for MetalCargoPowerUp {
    #[inline]
    fn parent(&self) -> &dyn PowerUp {
        &self.parent
    }
}

impl PowerUp for MetalCargoPowerUp {}
