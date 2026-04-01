use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractSteadyUnit, SteadyUnit, SteadyUnitInternal, Unit, UnitHierarchy, UnitInternal, UnitKind,
};
use crate::utils::Atomic;
use crate::GameError;
use std::sync::{Arc, Weak};

/// A black hole.
#[derive(Debug, Clone)]
pub struct BlackHole {
    parent: AbstractSteadyUnit,
    gravity_well_radius: Atomic<f32>,
    gravity_well_force: Atomic<f32>,
}

impl BlackHole {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractSteadyUnit::new(cluster, name, reader)?,
            gravity_well_radius: Atomic::default(),
            gravity_well_force: Atomic::default(),
        }))
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

impl UnitInternal for BlackHole {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.gravity_well_radius.read(reader);
        self.gravity_well_force.read(reader);
    }
}

impl UnitHierarchy for BlackHole {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_black_hole(&self) -> Option<&BlackHole> {
        Some(self)
    }
}

impl Unit for BlackHole {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::BlackHole
    }
}

impl SteadyUnitInternal for BlackHole {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for BlackHole {}
