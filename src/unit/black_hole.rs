use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{SteadyUnit, UnitBase, UnitExt, UnitExtSealed, UnitKind};
use crate::utils::{Atomic, Readable};
use std::sync::Weak;

/// A black hole.
#[derive(Debug, Clone)]
pub struct BlackHole {
    base: UnitBase,
    steady: SteadyUnit,
    gravity_well_radius: Atomic<f32>,
    gravity_well_force: Atomic<f32>,
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
            gravity_well_radius: Atomic::default(),
            gravity_well_force: Atomic::default(),
        }
    }

    /// Radius of the intensified gravity well.
    #[inline]
    pub fn gravity_well_radius(&self) -> f32 {
        self.gravity_well_radius.load()
    }

    /// Additional attraction force inside the gravity well.
    #[inline]
    pub fn gravity_well_force(&self) -> f32 {
        self.gravity_well_force.load()
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

    #[inline]
    fn parent(self) -> Self::Parent {
        (&self.base, &self.steady)
    }

    fn update_state(self, reader: &mut dyn PacketReader) {
        self.parent().update_state(reader);

        self.gravity_well_radius.read(reader);
        self.gravity_well_force.read(reader);
    }
}

impl<'a> UnitExt<'a> for &'a BlackHole {
    #[inline]
    fn kind(self) -> UnitKind {
        UnitKind::BlackHole
    }
}
