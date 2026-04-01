use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{AbstractUnit, Mobility, Unit, UnitHierarchy, UnitInternal};
use crate::utils::Atomic;
use crate::{GameError, Vector};
use std::sync::Weak;

pub(crate) trait MobileUnitInternal {}

/// Base type for mobile visible units.
#[allow(private_bounds)]
pub trait MobileUnit: MobileUnitInternal + Unit {}

#[derive(Debug, Clone)]
pub(crate) struct AbstractMobileUnit {
    parent: AbstractUnit,
    position: Atomic<Vector>,
    movement: Atomic<Vector>,
}

impl AbstractMobileUnit {
    pub(crate) fn new(cluster: Weak<Cluster>, name: String) -> Result<Self, GameError> {
        Ok(Self {
            parent: AbstractUnit::new(cluster, name),
            position: Atomic::default(),
            movement: Atomic::default(),
        })
    }

    fn read_position_and_movement(&self, reader: &mut dyn PacketReader) {
        self.position.read(reader);
        self.movement.read(reader);
    }
}

impl UnitInternal for AbstractMobileUnit {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_movement(&self, reader: &mut dyn PacketReader) {
        self.parent.update_movement(reader);
        self.read_position_and_movement(reader);
    }
}

impl UnitHierarchy for AbstractMobileUnit {}

impl Unit for AbstractMobileUnit {
    #[inline]
    fn position(&self) -> Vector {
        self.position.load()
    }

    #[inline]
    fn movement(&self) -> Vector {
        self.movement.load()
    }

    #[inline]
    fn angle(&self) -> f32 {
        self.movement.load().angle()
    }

    #[inline]
    fn mobility(&self) -> Mobility {
        Mobility::Mobile
    }
}

#[forbid(clippy::missing_trait_methods)]
impl MobileUnitInternal for AbstractMobileUnit {}

#[forbid(clippy::missing_trait_methods)]
impl MobileUnit for AbstractMobileUnit {}
