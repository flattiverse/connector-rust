use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractProjectile, MobileUnit, MobileUnitInternal, Projectile, ProjectileInternal, Unit,
    UnitHierarchy, UnitInternal, UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// Represents a rail projectile.
#[derive(Debug, Clone)]
pub struct Rail {
    parent: AbstractProjectile,
}

impl Rail {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractProjectile::new(cluster, name, reader)?,
        }))
    }
}

impl UnitInternal for Rail {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitHierarchy for Rail {
    #[inline]
    fn as_mobile_unit(&self) -> Option<&dyn MobileUnit> {
        Some(self)
    }

    #[inline]
    fn as_projectile(&self) -> Option<&dyn Projectile> {
        Some(self)
    }

    #[inline]
    fn as_rail(&self) -> Option<&Rail> {
        Some(self)
    }
}

impl Unit for Rail {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Rail
    }
}

impl MobileUnitInternal for Rail {}

impl MobileUnit for Rail {}

impl ProjectileInternal for Rail {
    #[inline]
    fn parent(&self) -> &dyn Projectile {
        &self.parent
    }
}

impl Projectile for Rail {}
