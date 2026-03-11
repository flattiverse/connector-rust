use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{SteadyUnit, UnitBase, UnitExt, UnitExtSealed, UnitKind};
use crate::utils::Readable;
use std::sync::Weak;

/// A black hole.
#[derive(Debug, Clone)]
pub struct BlackHole {
    base: UnitBase,
    steady: SteadyUnit,
    gravity_well_radius: f32,
    gravity_well_force: f32,
}

impl BlackHole {
    pub(crate) fn read(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            base: UnitBase::new(cluster, name),
            steady: SteadyUnit::read(reader),
            gravity_well_radius: reader.read_f32(),
            gravity_well_force: reader.read_f32(),
        }
    }

    /// Radius of the intensified gravity well.
    #[inline]
    pub fn gravity_well_radius(&self) -> f32 {
        self.gravity_well_radius
    }

    /// Additional attraction force inside the gravity well.
    #[inline]
    pub fn gravity_well_force(&self) -> f32 {
        self.gravity_well_force
    }
}

impl AsRef<UnitBase> for BlackHole {
    #[inline]
    fn as_ref(&self) -> &UnitBase {
        &self.base
    }
}

impl AsRef<SteadyUnit> for BlackHole {
    #[inline]
    fn as_ref(&self) -> &SteadyUnit {
        &self.steady
    }
}

impl<'a> UnitExtSealed<'a> for &'a BlackHole {
    type Parent = (&'a UnitBase, &'a SteadyUnit);

    fn parent(self) -> Self::Parent {
        (&self.base, &self.steady)
    }
}

impl<'a> UnitExt<'a> for &'a BlackHole {
    #[inline]
    fn kind(self) -> UnitKind {
        UnitKind::BlackHole
    }
}
