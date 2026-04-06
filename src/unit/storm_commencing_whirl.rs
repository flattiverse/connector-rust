use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractStormWhirl, MobileUnit, MobileUnitInternal, StormWhirl, StormWhirlInternal, Unit,
    UnitCastTable, UnitHierarchy, UnitInternal, UnitKind,
};
use crate::GameError;
use std::sync::{Arc, Weak};

/// A storm whirl that is still announcing itself and does not deal damage yet.
#[derive(Debug, Clone)]
pub struct StormCommencingWhirl {
    parent: AbstractStormWhirl,
}

impl StormCommencingWhirl {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractStormWhirl::new(cluster, name, reader)?,
        }))
    }
}

impl UnitInternal for StormCommencingWhirl {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);
        self.parent.read_remaining_ticks(reader);
    }
}

impl UnitCastTable for StormCommencingWhirl {
    cast_fn!(mobile_unit_cast_fn, StormCommencingWhirl, dyn MobileUnit);
    cast_fn!(storm_whirl_cast_fn, StormCommencingWhirl, dyn StormWhirl);
}

impl UnitHierarchy for StormCommencingWhirl {
    #[inline]
    fn as_mobile_unit(&self) -> Option<&dyn MobileUnit> {
        Some(self)
    }

    #[inline]
    fn as_storm_whirl(&self) -> Option<&dyn StormWhirl> {
        Some(self)
    }

    #[inline]
    fn as_storm_commencing_whirl(&self) -> Option<&StormCommencingWhirl> {
        Some(self)
    }
}

impl Unit for StormCommencingWhirl {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::StormCommencingWhirl
    }

    #[inline]
    fn is_masking(&self) -> bool {
        false
    }
}

impl MobileUnitInternal for StormCommencingWhirl {
    #[inline]
    fn parent(&self) -> &dyn MobileUnit {
        &self.parent
    }
}

impl MobileUnit for StormCommencingWhirl {}

impl StormWhirlInternal for StormCommencingWhirl {}

impl StormWhirl for StormCommencingWhirl {}
