use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractProjectile, MobileUnit, MobileUnitInternal, Projectile, ProjectileInternal, Unit,
    UnitCastTable, UnitHierarchy, UnitInternal, UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// Represents a shot.
#[derive(Debug, Clone)]
pub struct Shot {
    parent: AbstractProjectile,
}

impl Shot {
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

impl UnitInternal for Shot {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitCastTable for Shot {
    cast_fn!(mobile_unit_cast_fn, Shot, dyn MobileUnit);
    cast_fn!(projectile_cast_fn, Shot, dyn Projectile);
}

impl UnitHierarchy for Shot {
    #[inline]
    fn as_mobile_unit(&self) -> Option<&dyn MobileUnit> {
        Some(self)
    }

    #[inline]
    fn as_projectile(&self) -> Option<&dyn Projectile> {
        Some(self)
    }

    #[inline]
    fn as_shot(&self) -> Option<&Shot> {
        Some(self)
    }
}

impl Unit for Shot {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Shot
    }
}

impl MobileUnitInternal for Shot {
    #[inline]
    fn parent(&self) -> &dyn MobileUnit {
        &self.parent
    }
}

impl MobileUnit for Shot {}

impl ProjectileInternal for Shot {
    #[inline]
    fn parent(&self) -> &dyn Projectile {
        &self.parent
    }
}

impl Projectile for Shot {}
