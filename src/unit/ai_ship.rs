use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractMobileNpcUnit, MobileNpcUnit, MobileNpcUnitInternal, MobileUnit, MobileUnitInternal,
    Unit, UnitCastTable, UnitHierarchy, UnitInternal, UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// Mobile NPC ship.
#[derive(Debug)]
pub struct AiShip {
    parent: AbstractMobileNpcUnit,
}

impl AiShip {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractMobileNpcUnit::new(cluster, name, reader)?,
        }))
    }
}

impl UnitInternal for AiShip {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitCastTable for AiShip {
    cast_fn!(mobile_unit_cast_fn, AbstractMobileNpcUnit, dyn MobileUnit);
    cast_fn!(
        mobile_npc_unit_cast_fn,
        AbstractMobileNpcUnit,
        dyn MobileNpcUnit
    );
}

impl UnitHierarchy for AiShip {
    #[inline]
    fn as_mobile_npc_unit(&self) -> Option<&dyn MobileNpcUnit> {
        Some(self)
    }

    #[inline]
    fn as_ai_ship(&self) -> Option<&AiShip> {
        Some(self)
    }
}

impl Unit for AiShip {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::AiShip
    }
}

impl MobileUnitInternal for AiShip {
    #[inline]
    fn parent(&self) -> &dyn MobileUnit {
        &self.parent
    }
}

impl MobileUnit for AiShip {}

impl MobileNpcUnitInternal for AiShip {
    #[inline]
    fn parent(&self) -> &dyn MobileNpcUnit {
        &self.parent
    }
}

impl MobileNpcUnit for AiShip {}
