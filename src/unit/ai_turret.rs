use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractNpcUnit, NpcUnit, NpcUnitInternal, Unit, UnitCastTable, UnitHierarchy, UnitInternal,
    UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// Stationary NPC turret.
#[derive(Debug)]
pub struct AiTurret {
    parent: AbstractNpcUnit,
}

impl AiTurret {
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

impl UnitInternal for AiTurret {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitCastTable for AiTurret {
    cast_fn!(npc_unit_cast_fn, AbstractNpcUnit, dyn NpcUnit);
}

impl UnitHierarchy for AiTurret {
    #[inline]
    fn as_npc_unit(&self) -> Option<&dyn NpcUnit> {
        Some(self)
    }

    #[inline]
    fn as_ai_turret(&self) -> Option<&AiTurret> {
        Some(self)
    }
}

impl Unit for AiTurret {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::AiTurret
    }
}

impl NpcUnitInternal for AiTurret {
    #[inline]
    fn parent(&self) -> &dyn NpcUnit {
        &self.parent
    }
}

impl NpcUnit for AiTurret {}
