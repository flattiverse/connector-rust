use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractStormWhirl, MobileUnit, MobileUnitInternal, StormWhirl, StormWhirlInternal, Unit,
    UnitCastTable, UnitHierarchy, UnitInternal, UnitKind,
};
use crate::utils::Atomic;
use crate::GameError;
use std::sync::{Arc, Weak};

/// A storm whirl that is active, masking, and damaging.
#[derive(Debug, Clone)]
pub struct StormActiveWhirl {
    parent: AbstractStormWhirl,
    damage: Atomic<f32>,
}

impl StormActiveWhirl {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractStormWhirl::new(cluster, name, reader)?,
            damage: Atomic::from(0.0),
        }))
    }

    /// Damage applied by each successful hit of this active whirl.
    #[inline]
    pub fn damage(&self) -> f32 {
        self.damage.load()
    }
}

impl UnitInternal for StormActiveWhirl {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);
        self.parent.read_remaining_ticks(reader);
        self.damage.read(reader);
    }
}

impl UnitCastTable for StormActiveWhirl {
    cast_fn!(mobile_unit_cast_fn, StormActiveWhirl, dyn MobileUnit);
    cast_fn!(storm_whirl_cast_fn, StormActiveWhirl, dyn StormWhirl);
}

impl UnitHierarchy for StormActiveWhirl {
    #[inline]
    fn as_mobile_unit(&self) -> Option<&dyn MobileUnit> {
        Some(self)
    }

    #[inline]
    fn as_storm_whirl(&self) -> Option<&dyn StormWhirl> {
        Some(self)
    }

    #[inline]
    fn as_storm_active_whirl(&self) -> Option<&StormActiveWhirl> {
        Some(self)
    }
}

impl Unit for StormActiveWhirl {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::StormActiveWhirl
    }
}

impl MobileUnitInternal for StormActiveWhirl {}

impl MobileUnit for StormActiveWhirl {}

impl StormWhirlInternal for StormActiveWhirl {}

impl StormWhirl for StormActiveWhirl {}
