use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{SteadyUnit, TargetUnit, UnitBase, UnitExt, UnitExtSealed, UnitKind};
use crate::utils::{Also, Atomic, Readable};
use std::sync::Weak;

/// A flag target.
#[derive(Debug, Clone)]
pub struct Flag {
    base: UnitBase,
    steady: SteadyUnit,
    target: TargetUnit,
    grace_ticks: Atomic<i32>,
    active: Atomic<bool>,
}

impl Flag {
    pub(crate) fn read(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            base: UnitBase::new(cluster.clone(), name),
            steady: SteadyUnit::read(reader),
            target: TargetUnit::read(&cluster, reader),
            grace_ticks: Atomic::default(),
            active: Atomic::default(),
        }
        .also(|flag| flag.base.mark_full_state_known())
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

impl AsRef<UnitBase> for Flag {
    #[inline]
    fn as_ref(&self) -> &UnitBase {
        &self.base
    }
}

impl AsRef<SteadyUnit> for Flag {
    #[inline]
    fn as_ref(&self) -> &SteadyUnit {
        &self.steady
    }
}
impl AsRef<TargetUnit> for Flag {
    #[inline]
    fn as_ref(&self) -> &TargetUnit {
        &self.target
    }
}

impl<'a> UnitExtSealed<'a> for &'a Flag {
    type Parent = (&'a UnitBase, &'a SteadyUnit, &'a TargetUnit);

    #[inline]
    fn parent(self) -> Self::Parent {
        (&self.base, &self.steady, &self.target)
    }

    fn update_state(self, reader: &mut dyn PacketReader) {
        self.parent().update_state(reader);

        self.grace_ticks.read(reader);
        self.active.store(reader.read_byte() != 0);
    }
}

impl<'a> UnitExt<'a> for &'a Flag {
    #[inline]
    fn kind(self) -> UnitKind {
        UnitKind::Flag
    }
}
