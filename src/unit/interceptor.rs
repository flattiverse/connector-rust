use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractProjectile, MobileUnit, MobileUnitInternal, Projectile, ProjectileInternal, Unit,
    UnitCastTable, UnitHierarchy, UnitInternal, UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// Represents a Interceptor.
#[derive(Debug, Clone)]
pub struct Interceptor {
    parent: AbstractProjectile,
}

impl Interceptor {
    pub const FIXED_SPEED_LIMIT: f32 = 10.0;

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

impl UnitInternal for Interceptor {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitCastTable for Interceptor {
    cast_fn!(mobile_unit_cast_fn, Interceptor, dyn MobileUnit);
    cast_fn!(projectile_cast_fn, Interceptor, dyn Projectile);
}

impl UnitHierarchy for Interceptor {
    #[inline]
    fn as_mobile_unit(&self) -> Option<&dyn MobileUnit> {
        Some(self)
    }

    #[inline]
    fn as_projectile(&self) -> Option<&dyn Projectile> {
        Some(self)
    }

    #[inline]
    fn as_interceptor(&self) -> Option<&Interceptor> {
        Some(self)
    }
}

impl Unit for Interceptor {
    #[inline]
    fn speed_limit(&self) -> f32 {
        Self::FIXED_SPEED_LIMIT
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Interceptor
    }
}

impl MobileUnitInternal for Interceptor {
    #[inline]
    fn parent(&self) -> &dyn MobileUnit {
        &self.parent
    }
}

impl MobileUnit for Interceptor {}

impl ProjectileInternal for Interceptor {
    #[inline]
    fn parent(&self) -> &dyn Projectile {
        &self.parent
    }
}

impl Projectile for Interceptor {}
