use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractExplosion, Explosion, ExplosionInternal, Unit, UnitCastTable, UnitHierarchy,
    UnitInternal, UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// Represents an interceptor explosion.
#[derive(Debug, Clone)]
pub struct InterceptorExplosion {
    parent: AbstractExplosion,
}

impl InterceptorExplosion {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractExplosion::new_inner(cluster, name, reader)?,
        }))
    }
}

impl UnitInternal for InterceptorExplosion {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitCastTable for InterceptorExplosion {
    cast_fn!(explosion_cast_fn, InterceptorExplosion, dyn Explosion);
}

impl UnitHierarchy for InterceptorExplosion {
    #[inline]
    fn as_explosion(&self) -> Option<&dyn Explosion> {
        Some(self)
    }

    #[inline]
    fn as_interceptor_explosion(&self) -> Option<&InterceptorExplosion> {
        Some(self)
    }
}

impl Unit for InterceptorExplosion {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::InterceptorExplosion
    }
}

impl ExplosionInternal for InterceptorExplosion {
    fn parent(&self) -> &dyn Explosion {
        &self.parent
    }
}

impl Explosion for InterceptorExplosion {}
