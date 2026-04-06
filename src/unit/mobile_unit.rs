use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{AbstractUnit, Mobility, Unit, UnitCastTable, UnitHierarchy, UnitInternal};
use crate::utils::Atomic;
use crate::Vector;
use std::sync::Arc;
use std::sync::Weak;

pub(crate) trait MobileUnitInternal {
    fn parent(&self) -> &dyn MobileUnit;
}

/// Base type for mobile visible units.
#[allow(private_bounds)]
pub trait MobileUnit: MobileUnitInternal + Unit {
    #[inline]
    fn angular_velocity(&self) -> f32 {
        MobileUnitInternal::parent(self).angular_velocity()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AbstractMobileUnit {
    parent: AbstractUnit,
    pub(crate) position: Atomic<Vector>,
    pub(crate) movement: Atomic<Vector>,
    pub(crate) angle: Atomic<f32>,
    pub(crate) angular_velocity: Atomic<f32>,
}

impl AbstractMobileUnit {
    pub(crate) fn new(cluster: Weak<Cluster>, name: String) -> Self {
        Self {
            parent: AbstractUnit::new(cluster, name),
            position: Atomic::default(),
            movement: Atomic::default(),
            angle: Atomic::default(),
            angular_velocity: Atomic::default(),
        }
    }

    pub(crate) fn read_position_and_movement(&self, reader: &mut dyn PacketReader) {
        self.position.read(reader);
        self.movement.read(reader);
        self.angle.read(reader);
        self.angular_velocity.read(reader);
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

impl UnitCastTable for AbstractMobileUnit {
    cast_fn!(mobile_unit_cast_fn, AbstractMobileUnit, dyn MobileUnit);
}

impl UnitHierarchy for AbstractMobileUnit {
    #[inline]
    fn as_mobile_unit(&self) -> Option<&dyn MobileUnit> {
        Some(self)
    }
}

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
        self.angle.load()
    }

    #[inline]
    fn mobility(&self) -> Mobility {
        Mobility::Mobile
    }
}

#[forbid(clippy::missing_trait_methods)]
impl MobileUnitInternal for AbstractMobileUnit {
    fn parent(&self) -> &dyn MobileUnit {
        unreachable!()
    }
}

#[forbid(clippy::missing_trait_methods)]
impl MobileUnit for AbstractMobileUnit {
    #[inline]
    fn angular_velocity(&self) -> f32 {
        self.angular_velocity.load()
    }
}
