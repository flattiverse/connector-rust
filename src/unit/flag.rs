use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractTargetUnit, SteadyUnit, SteadyUnitInternal, TargetUnit, TargetUnitInternal, Unit,
    UnitHierarchy, UnitInternal, UnitKind,
};
use crate::utils::{Also, Atomic};
use crate::GameError;
use std::sync::{Arc, Weak};

/// A flag target.
#[derive(Debug, Clone)]
pub struct Flag {
    pub(crate) parent: AbstractTargetUnit,
    grace_ticks: Atomic<i32>,
    active: Atomic<bool>,
}

impl Flag {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(
            Self {
                parent: AbstractTargetUnit::new(cluster, name, reader)?,
                grace_ticks: Atomic::default(),
                active: Atomic::default(),
            }
            .also(|flag| flag.mark_full_state_known()),
        ))
    }

    /// Configured flag grace time in ticks.
    #[inline]
    pub fn grace_ticks(&self) -> i32 {
        self.grace_ticks.load()
    }

    /// True while the flag can currently be scored.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }
}

impl UnitInternal for Flag {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.grace_ticks.read(reader);
        self.active.store(reader.read_byte() != 0);
    }
}

impl UnitHierarchy for Flag {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_target_unit(&self) -> Option<&dyn TargetUnit> {
        Some(self)
    }

    #[inline]
    fn as_flag(&self) -> Option<&Flag> {
        Some(self)
    }
}

impl Unit for Flag {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Flag
    }
}

impl SteadyUnitInternal for Flag {}

impl SteadyUnit for Flag {}

impl TargetUnitInternal for Flag {}

impl TargetUnit for Flag {}
