use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::unit::{AbstractUnit, Unit, UnitInternal};
use crate::unit::{Mobility, Orbit, UnitHierarchy};
use crate::utils::Atomic;
use crate::{GameError, Vector};
use arc_swap::ArcSwap;
use std::sync::{Arc, Weak};

pub(crate) trait SteadyUnitInternal {
    fn parent(&self) -> &dyn SteadyUnit;
}

/// Map units such as suns or planets that remain present in a cluster.
#[allow(private_bounds)]
pub trait SteadyUnit: SteadyUnitInternal + Unit {
    /// Configured map-editor position of the unit.
    /// For orbiting units this is the center of the first orbit segment, not necessarily the
    /// current live position.
    #[inline]
    fn configured_position(&self) -> Vector {
        SteadyUnitInternal::parent(self).configured_position()
    }

    /// Orbit chain received once the unit becomes fully visible.
    /// Each entry is applied relative to the configured center or to the previous orbit segment.
    #[inline]
    fn orbiting_list(&self) -> Arc<Vec<Orbit>> {
        SteadyUnitInternal::parent(self).orbiting_list()
    }
}

#[derive(Debug)]
pub(crate) struct AbstractSteadyUnit {
    parent: AbstractUnit,
    gravity: Atomic<f32>,
    radius: Atomic<f32>,
    position: Atomic<Vector>,
    movement: Atomic<Vector>,
    configured_position: Atomic<Vector>,
    orbiting_list: ArcSwap<Vec<Orbit>>,
}

impl Clone for AbstractSteadyUnit {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            gravity: self.gravity.clone(),
            radius: self.radius.clone(),
            position: self.position.clone(),
            movement: self.movement.clone(),
            configured_position: self.configured_position.clone(),
            orbiting_list: ArcSwap::new(self.orbiting_list.load_full()),
        }
    }
}

impl AbstractSteadyUnit {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Self, GameError> {
        Ok(Self {
            parent: AbstractUnit::new(cluster, name),
            position: Atomic::from_reader(reader),
            radius: Atomic::from_reader(reader),
            gravity: Atomic::from_reader(reader),
            movement: Atomic::default(),
            configured_position: Atomic::default(),
            orbiting_list: ArcSwap::default(),
        })
    }
}

impl UnitInternal for AbstractSteadyUnit {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_movement(&self, reader: &mut dyn PacketReader) {
        self.position.read(reader);
        self.movement.read(reader);
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.configured_position.read(reader);

        let orbit_count = reader.read_byte();
        let mut orbits = Vec::with_capacity(usize::from(orbit_count));
        for _ in 0..orbit_count {
            orbits.push(Orbit::new(
                reader.read_f32(),
                reader.read_f32(),
                reader.read_int32(),
            ));
        }
        self.orbiting_list.store(Arc::new(orbits));
    }
}

impl UnitHierarchy for AbstractSteadyUnit {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }
}

impl Unit for AbstractSteadyUnit {
    #[inline]
    fn radius(&self) -> f32 {
        self.radius.load()
    }

    #[inline]
    fn position(&self) -> Vector {
        self.position.load()
    }

    #[inline]
    fn movement(&self) -> Vector {
        self.movement.load()
    }

    #[inline]
    fn gravity(&self) -> f32 {
        self.gravity.load()
    }

    fn mobility(&self) -> Mobility {
        if !self.orbiting_list().is_empty() || self.movement().length_squared() != 0.0 {
            Mobility::Steady
        } else {
            Mobility::Still
        }
    }
}

#[forbid(clippy::missing_trait_methods)]
impl SteadyUnitInternal for AbstractSteadyUnit {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        unreachable!()
    }
}

#[forbid(clippy::missing_trait_methods)]
impl SteadyUnit for AbstractSteadyUnit {
    #[inline]
    fn configured_position(&self) -> Vector {
        self.configured_position.load()
    }

    #[inline]
    fn orbiting_list(&self) -> Arc<Vec<Orbit>> {
        self.orbiting_list.load_full()
    }
}
