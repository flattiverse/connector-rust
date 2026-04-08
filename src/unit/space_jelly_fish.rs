use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractMobileUnit, MobileUnit, MobileUnitInternal, Unit, UnitCastTable, UnitHierarchy,
    UnitInternal, UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

#[derive(Debug, Clone)]
pub struct SpaceJellyFish {
    parent: AbstractMobileUnit,
}

impl SpaceJellyFish {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        _reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractMobileUnit::new(cluster, name),
        }))
    }
}

impl UnitInternal for SpaceJellyFish {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitCastTable for SpaceJellyFish {
    cast_fn!(mobile_unit_cast_fn, SpaceJellyFish, dyn MobileUnit);
}

impl UnitHierarchy for SpaceJellyFish {
    #[inline]
    fn as_mobile_unit(&self) -> Option<&dyn MobileUnit> {
        Some(self)
    }

    #[inline]
    fn as_space_jelly_fish(&self) -> Option<&SpaceJellyFish> {
        Some(self)
    }
}

impl Unit for SpaceJellyFish {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::SpaceJellyFish
    }
}

impl MobileUnitInternal for SpaceJellyFish {
    #[inline]
    fn parent(&self) -> &dyn MobileUnit {
        &self.parent
    }
}

impl MobileUnit for SpaceJellyFish {}
