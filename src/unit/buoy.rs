use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{SteadyUnit, UnitBase};
use crate::utils::Readable;
use std::sync::Weak;

/// A buoy.
#[derive(Debug, Clone)]
pub struct Buoy {
    base: UnitBase,
    steady: SteadyUnit,
}

impl Buoy {
    pub(crate) fn read(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            base: UnitBase::new(cluster, name),
            steady: SteadyUnit::read(reader),
        }
    }
}

impl AsRef<UnitBase> for Buoy {
    #[inline]
    fn as_ref(&self) -> &UnitBase {
        &self.base
    }
}

impl AsRef<SteadyUnit> for Buoy {
    #[inline]
    fn as_ref(&self) -> &SteadyUnit {
        &self.steady
    }
}
