use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractMobileNpcUnit, MobileNpcUnit, MobileNpcUnitInternal, MobileUnit, MobileUnitInternal,
    Unit, UnitCastTable, UnitHierarchy, UnitInternal, UnitKind,
};
use crate::utils::Atomic;
use crate::GameError;
use std::sync::{Arc, Weak};

/// Mobile NPC freighter with visible loot values.
#[derive(Debug)]
pub struct AiFreighter {
    parent: AbstractMobileNpcUnit,
    metal: Atomic<f32>,
    carbon: Atomic<f32>,
    hydrogen: Atomic<f32>,
    silicon: Atomic<f32>,
}

impl AiFreighter {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractMobileNpcUnit::new(cluster, name, reader)?,
            metal: Atomic::from(0.0),
            carbon: Atomic::from(0.0),
            hydrogen: Atomic::from(0.0),
            silicon: Atomic::from(0.0),
        }))
    }

    /// Metal loot value.
    #[inline]
    pub fn metal(&self) -> f32 {
        self.metal.load()
    }

    /// Carbon loot value.
    #[inline]
    pub fn carbon(&self) -> f32 {
        self.carbon.load()
    }

    /// Hydrogen loot value.
    #[inline]
    pub fn hydrogen(&self) -> f32 {
        self.hydrogen.load()
    }

    /// Silicon loot value.
    #[inline]
    pub fn silicon(&self) -> f32 {
        self.silicon.load()
    }
}

impl UnitInternal for AiFreighter {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.metal.read(reader);
        self.carbon.read(reader);
        self.hydrogen.read(reader);
        self.silicon.read(reader);
    }
}

impl UnitCastTable for AiFreighter {
    cast_fn!(mobile_unit_cast_fn, AbstractMobileNpcUnit, dyn MobileUnit);
    cast_fn!(
        mobile_npc_unit_cast_fn,
        AbstractMobileNpcUnit,
        dyn MobileNpcUnit
    );
}

impl UnitHierarchy for AiFreighter {
    #[inline]
    fn as_mobile_npc_unit(&self) -> Option<&dyn MobileNpcUnit> {
        Some(self)
    }

    #[inline]
    fn as_ai_freighter(&self) -> Option<&AiFreighter> {
        Some(self)
    }
}

impl Unit for AiFreighter {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::AiFreighter
    }
}

impl MobileUnitInternal for AiFreighter {
    #[inline]
    fn parent(&self) -> &dyn MobileUnit {
        &self.parent
    }
}

impl MobileUnit for AiFreighter {}

impl MobileNpcUnitInternal for AiFreighter {
    #[inline]
    fn parent(&self) -> &dyn MobileNpcUnit {
        &self.parent
    }
}

impl MobileNpcUnit for AiFreighter {}
