use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::unit::{AbstractUnit, Unit, UnitInternal};
use crate::unit::UnitHierarchy;
use crate::utils::Atomic;
use crate::{GameError, Vector};
use std::sync::Weak;

pub(crate) trait SteadyUnitInternal {}

/// Map units such as suns or planets that remain present in a cluster.
#[allow(private_bounds)]
pub trait SteadyUnit: SteadyUnitInternal + Unit {}

#[derive(Debug, Clone)]
pub(crate) struct AbstractSteadyUnit {
    parent: AbstractUnit,
    gravity: Atomic<f32>,
    radius: Atomic<f32>,
    position: Atomic<Vector>,
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
        })
    }
}

impl UnitInternal for AbstractSteadyUnit {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
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
    fn gravity(&self) -> f32 {
        self.gravity.load()
    }
}

#[forbid(clippy::missing_trait_methods)]
impl SteadyUnitInternal for AbstractSteadyUnit {}

#[forbid(clippy::missing_trait_methods)]
impl SteadyUnit for AbstractSteadyUnit {}
