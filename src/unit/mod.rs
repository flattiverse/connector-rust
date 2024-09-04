mod kind;
pub use kind::*;

mod base;
pub use base::*;

mod steady;
pub use steady::*;

mod mobility;
pub use mobility::*;

mod player_unit;
pub use player_unit::*;

use crate::galaxy_hierarchy::{Cluster, NamedUnit, Team};
use crate::network::PacketReader;
use crate::runtime::Readable;
use crate::Vector;
use std::ops::Deref;
use std::sync::{Arc, Weak};

/// Represents a unit in Flattiverse. Each unit in a Cluster derives from this class. This enum
/// has properties and methods which most units have in common. Specialized properties and methods
/// are store in the relevant variant or added via interfaces.
#[derive(Debug)]
pub enum Unit {
    Planet {
        base: UnitBase,
        steady: SteadyUnit,
    },
    ClassicShipPlayerUnit {
        base: UnitBase,
        player_unit: PlayerUnit,
    },
}

impl Unit {
    pub(crate) fn try_read(
        kind: UnitKind,
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Option<Self> {
        match kind {
            UnitKind::Sun => None,
            UnitKind::BlackHole => None,
            UnitKind::Planet => Some(Unit::Planet {
                base: UnitBase::new(cluster, name),
                steady: SteadyUnit::read(reader),
            }),
            UnitKind::Moon => None,
            UnitKind::Meteoroid => None,
            UnitKind::ClassicShipPlayerUnit => {
                let galaxy = cluster.upgrade().unwrap().galaxy();
                Some(Unit::ClassicShipPlayerUnit {
                    base: UnitBase::new(cluster, name),
                    player_unit: PlayerUnit::read(&*galaxy, reader),
                })
            }
            UnitKind::NewShipPlayerUnit => None,
            UnitKind::Unknown(_) => None,
        }
    }

    /// This is the name of the unit. A unit can't change her name after it has been set up.
    #[inline]
    pub fn name(&self) -> &str {
        self.base().name()
    }

    /// The radius of the unit.
    pub fn radius(&self) -> f32 {
        match self {
            Unit::Planet { steady, .. } => steady.radius(),
            Unit::ClassicShipPlayerUnit { .. } => 14f32,
        }
    }

    /// The position of the unit.
    pub fn position(&self) -> Vector {
        match self {
            Unit::Planet { steady, .. } => steady.position(),
            Unit::ClassicShipPlayerUnit { player_unit, .. } => player_unit.position(),
        }
    }

    /// The movement of the unit.
    pub fn movement(&self) -> Vector {
        match self {
            Unit::Planet { .. } => Vector::default(),
            Unit::ClassicShipPlayerUnit { player_unit, .. } => player_unit.movement(),
        }
    }

    /// The direction the unit is looking into.
    #[inline]
    pub fn angle(&self) -> f32 {
        f32::default()
    }

    /// If true, other unis can hide behind this unit.
    #[inline]
    pub fn is_masking(&self) -> bool {
        true
    }

    /// If true, a crash with this unit is lethal.
    #[inline]
    pub fn is_solid(&self) -> bool {
        true
    }

    /// If true, the unit can be edited via map editor calls.
    #[inline]
    pub fn can_be_edited(&self) -> bool {
        false
    }

    /// The gravity of this unit. This is how much this unit pulls others towards it.
    #[inline]
    pub fn gravity(&self) -> f32 {
        match self {
            Unit::Planet { steady, .. } => steady.gravity(),
            Unit::ClassicShipPlayerUnit { .. } => 0.0012f32,
        }
    }

    /// The mobility of this unit.
    #[inline]
    pub fn mobility(&self) -> Mobility {
        match self {
            Unit::Planet { .. } => Mobility::Still,
            Unit::ClassicShipPlayerUnit { .. } => Mobility::Mobile,
        }
    }

    /// The kind of the unit for a better match() experience.
    #[inline]
    pub fn kind(&self) -> UnitKind {
        match self {
            Unit::Planet { .. } => UnitKind::Planet,
            Unit::ClassicShipPlayerUnit { .. } => UnitKind::ClassicShipPlayerUnit,
        }
    }

    /// The cluster the unit is in.
    #[inline]
    pub fn cluster(&self) -> Arc<Cluster> {
        self.base().cluster()
    }

    /// The team of the unit.
    #[inline]
    pub fn team(&self) -> Option<Arc<Team>> {
        match self {
            Unit::Planet { .. } => None,
            Unit::ClassicShipPlayerUnit { player_unit, .. } => Some(player_unit.player().team()),
        }
    }

    pub(crate) fn update_movement(&self, reader: &mut dyn PacketReader) {
        match self {
            Unit::Planet { .. } => unreachable!(),
            Unit::ClassicShipPlayerUnit { player_unit, .. } => player_unit.update_movement(reader),
        }
    }

    pub fn base(&self) -> &UnitBase {
        match self {
            Unit::Planet { base, .. } => base,
            Unit::ClassicShipPlayerUnit { base, .. } => &base,
        }
    }

    pub fn steady(&self) -> Option<&SteadyUnit> {
        match self {
            Unit::Planet { steady, .. } => Some(steady),
            Unit::ClassicShipPlayerUnit { .. } => None,
        }
    }

    pub fn player_unit(&self) -> Option<&PlayerUnit> {
        match self {
            Unit::Planet { .. } => None,
            Unit::ClassicShipPlayerUnit { player_unit, .. } => Some(player_unit),
        }
    }
}

impl NamedUnit for Unit {
    #[inline]
    fn name(&self) -> impl Deref<Target = str> + '_ {
        self.base().name()
    }
}
