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
pub struct AiProbe {
    parent: AbstractMobileNpcUnit,
}

impl AiProbe {
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

impl UnitInternal for AiProbe {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitCastTable for AiProbe {
    cast_fn!(mobile_unit_cast_fn, AbstractMobileNpcUnit, dyn MobileUnit);
    cast_fn!(
        mobile_npc_unit_cast_fn,
        AbstractMobileNpcUnit,
        dyn MobileNpcUnit
    );
}

impl UnitHierarchy for AiProbe {
    #[inline]
    fn as_mobile_npc_unit(&self) -> Option<&dyn MobileNpcUnit> {
        Some(self)
    }

    #[inline]
    fn as_ai_probe(&self) -> Option<&AiProbe> {
        Some(self)
    }
}

impl Unit for AiProbe {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::AiProbe
    }
}

impl MobileUnitInternal for AiProbe {
    #[inline]
    fn parent(&self) -> &dyn MobileUnit {
        &self.parent
    }
}

impl MobileUnit for AiProbe {}

impl MobileNpcUnitInternal for AiProbe {
    #[inline]
    fn parent(&self) -> &dyn MobileNpcUnit {
        &self.parent
    }
}

impl MobileNpcUnit for AiProbe {}
