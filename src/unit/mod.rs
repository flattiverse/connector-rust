mod kind;
pub use kind::*;

mod base;
pub use base::*;

mod steady;
pub use steady::*;

mod mobility;
pub use mobility::*;

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
    Planet { base: UnitBase, steady: SteadyUnit },
}

impl Unit {
    pub(crate) fn try_read(
        kind: UnitKind,
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Option<Self> {
        let base = UnitBase::from_packet(cluster, name, reader);
        match kind {
            UnitKind::Sun => None,
            UnitKind::BlackHole => None,
            UnitKind::Planet => Some(Unit::Planet {
                base,
                steady: SteadyUnit::read(reader),
            }),
            UnitKind::Moon => None,
            UnitKind::Meteoroid => None,
            UnitKind::ClassicalShipPlayerUnit => None,
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
    #[inline]
    pub fn radius(&self) -> f32 {
        self.base().radius()
    }

    /// The position of the unit.
    #[inline]
    pub fn position(&self) -> Vector {
        self.base().position()
    }

    /// The movement of the unit.
    #[inline]
    pub fn movement(&self) -> Vector {
        dbg!(Vector::default())
    }

    /// The direction the unit is looking into.
    #[inline]
    pub fn angle(&self) -> f32 {
        dbg!(f32::default())
    }

    /// If true, other unis can hide behind this unit.
    #[inline]
    pub fn is_masking(&self) -> bool {
        dbg!(true)
    }

    /// If true, a crash with this unit is lethal.
    #[inline]
    pub fn is_solid(&self) -> bool {
        dbg!(true)
    }

    /// If true, the unit can be edited via map editor calls.
    #[inline]
    pub fn can_be_edited(&self) -> bool {
        dbg!(false)
    }

    /// The gravity of this unit. This is how much this unit pulls others towards it.
    #[inline]
    pub fn gravity(&self) -> f32 {
        self.steady().map(SteadyUnit::gravity).unwrap_or_default()
    }

    /// The mobility of this unit.
    #[inline]
    pub fn mobility(&self) -> Mobility {
        match self {
            Unit::Planet { .. } => Mobility::Still,
        }
    }

    /// The kind of the unit for a better match() experience.
    #[inline]
    pub fn kind(&self) -> UnitKind {
        match self {
            Unit::Planet { .. } => UnitKind::Planet,
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
        }
    }

    pub fn base(&self) -> &UnitBase {
        match self {
            Unit::Planet { base, .. } => &base,
        }
    }

    pub fn steady(&self) -> Option<&SteadyUnit> {
        match self {
            Unit::Planet { steady, .. } => Some(steady),
        }
    }
}

impl NamedUnit for Unit {
    #[inline]
    fn name(&self) -> impl Deref<Target = str> + '_ {
        self.base().name.as_str()
    }
}
