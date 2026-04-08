use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractNpcUnit, NpcUnit, NpcUnitInternal, Unit, UnitCastTable, UnitHierarchy, UnitInternal,
    UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// Stationary NPC base.
#[derive(Debug)]
pub struct AiBase {
    parent: AbstractNpcUnit,
}

impl AiBase {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractNpcUnit::new(cluster, name, reader)?,
        }))
    }
}

impl UnitInternal for AiBase {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitCastTable for AiBase {
    cast_fn!(npc_unit_cast_fn, AbstractNpcUnit, dyn NpcUnit);
}

impl UnitHierarchy for AiBase {
    #[inline]
    fn as_npc_unit(&self) -> Option<&dyn NpcUnit> {
        Some(self)
    }

    #[inline]
    fn as_ai_base(&self) -> Option<&AiBase> {
        Some(self)
    }
}

impl Unit for AiBase {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::AiBase
    }
}

impl NpcUnitInternal for AiBase {
    #[inline]
    fn parent(&self) -> &dyn NpcUnit {
        &self.parent
    }
}

impl NpcUnit for AiBase {}
