use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractPowerUp, PowerUp, PowerUpInternal, SteadyUnit, SteadyUnitInternal, Unit, UnitCastTable,
    UnitHierarchy, UnitInternal, UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// A visible silicon cargo power-up.
#[derive(Debug, Clone)]
pub struct SiliconCargoPowerUp {
    parent: AbstractPowerUp,
}

impl SiliconCargoPowerUp {
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

impl UnitInternal for SiliconCargoPowerUp {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitCastTable for SiliconCargoPowerUp {
    cast_fn!(steady_unit_cast_fn, SiliconCargoPowerUp, dyn SteadyUnit);
    cast_fn!(power_up_cast_fn, SiliconCargoPowerUp, dyn PowerUp);
}

impl UnitHierarchy for SiliconCargoPowerUp {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_power_up(&self) -> Option<&dyn PowerUp> {
        Some(self)
    }

    #[inline]
    fn as_silicon_cargo_power_up(&self) -> Option<&SiliconCargoPowerUp> {
        Some(self)
    }
}

impl Unit for SiliconCargoPowerUp {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::SiliconCargoPowerUp
    }
}

impl SteadyUnitInternal for SiliconCargoPowerUp {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for SiliconCargoPowerUp {}

impl PowerUpInternal for SiliconCargoPowerUp {
    #[inline]
    fn parent(&self) -> &dyn PowerUp {
        &self.parent
    }
}

impl PowerUp for SiliconCargoPowerUp {}
