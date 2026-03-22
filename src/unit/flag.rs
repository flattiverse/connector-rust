use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{SteadyUnit, TargetUnit, UnitBase, UnitExt, UnitExtSealed, UnitKind};
use crate::utils::{Also, Readable};
use std::sync::Weak;

/// A flag target.
#[derive(Debug, Clone)]
pub struct Flag {
    base: UnitBase,
    steady: SteadyUnit,
    target: TargetUnit,
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
        }
        .also(|flag| flag.base.mark_full_state_known())
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
}

impl<'a> UnitExt<'a> for &'a Flag {
    #[inline]
    fn kind(self) -> UnitKind {
        UnitKind::Flag
    }
}
