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

mod explosion;
pub use explosion::*;

mod shot;
pub use shot::*;

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
    Shot(Shot),
    Explosion(Explosion),
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
            UnitKind::Shot => Some(Unit::Shot(Shot::read(cluster, name, reader))),
            UnitKind::ClassicShipPlayerUnit => {
                let galaxy = cluster.upgrade().unwrap().galaxy();
                Some(Unit::ClassicShipPlayerUnit {
                    base: UnitBase::new(cluster, name),
                    player_unit: PlayerUnit::read(&*galaxy, reader),
                })
            }
            UnitKind::NewShipPlayerUnit => None,
            UnitKind::Explosion => Some(Unit::Explosion(Explosion::read(cluster, name, reader))),
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
            Unit::Shot(shot) => shot.radius(),
            Unit::Explosion(explosion) => explosion.radius(),
        }
    }

    /// The position of the unit.
    pub fn position(&self) -> Vector {
        match self {
            Unit::Planet { steady, .. } => steady.position(),
            Unit::ClassicShipPlayerUnit { player_unit, .. } => player_unit.position(),
            Unit::Shot(shot) => shot.position(),
            Unit::Explosion(explosion) => explosion.position(),
        }
    }

    /// The movement of the unit.
    pub fn movement(&self) -> Vector {
        match self {
            Unit::Planet { .. } => Vector::default(),
            Unit::ClassicShipPlayerUnit { player_unit, .. } => player_unit.movement(),
            Unit::Shot(shot) => shot.movement(),
            Unit::Explosion(_) => Vector::default(),
        }
    }

    /// The direction the unit is looking into.
    pub fn angle(&self) -> f32 {
        match self {
            Unit::ClassicShipPlayerUnit { player_unit, .. } => player_unit.angle(),
            Unit::Shot(shot) => shot.angle(),
            _ => f32::default(),
        }
    }

    /// If true, other unis can hide behind this unit.
    pub fn is_masking(&self) -> bool {
        match self {
            Unit::Explosion(e) => e.is_masking(),
            _ => true,
        }
    }

    /// If true, a crash with this unit is lethal.
    pub fn is_solid(&self) -> bool {
        match self {
            Unit::Explosion(e) => e.is_solid(),
            _ => true,
        }
    }

    /// If true, the unit can be edited via map editor calls.

    pub fn can_be_edited(&self) -> bool {
        match self {
            _ => false,
        }
    }

    /// The gravity of this unit. This is how much this unit pulls others towards it.
    pub fn gravity(&self) -> f32 {
        match self {
            Unit::Planet { steady, .. } => steady.gravity(),
            Unit::ClassicShipPlayerUnit { .. } => 0.0012f32,
            Unit::Explosion(e) => e.gravity(),
            _ => 0.0,
        }
    }

    /// The mobility of this unit.
    #[inline]
    pub fn mobility(&self) -> Mobility {
        match self {
            Unit::ClassicShipPlayerUnit { player_unit, .. } => player_unit.mobility(),
            Unit::Shot(shot) => shot.mobility(),
            _ => Mobility::Still,
        }
    }

    /// The kind of the unit for a better match() experience.
    #[inline]
    pub fn kind(&self) -> UnitKind {
        match self {
            Unit::Planet { .. } => UnitKind::Planet,
            Unit::ClassicShipPlayerUnit { .. } => UnitKind::ClassicShipPlayerUnit,
            Unit::Shot(_) => UnitKind::Shot,
            Unit::Explosion(_) => UnitKind::Explosion,
        }
    }

    /// The cluster the unit is in.
    #[inline]
    pub fn cluster(&self) -> Arc<Cluster> {
        self.base().cluster()
    }

    /// The team of the unit.
    pub fn team(&self) -> Weak<Team> {
        match self {
            Unit::Planet { .. } => Weak::default(),
            Unit::ClassicShipPlayerUnit { player_unit, .. } => {
                Arc::downgrade(&player_unit.player().team())
            }
            Unit::Shot(shot) => shot.team(),
            Unit::Explosion(explosion) => explosion.team(),
        }
    }

    pub(crate) fn update_movement(&self, reader: &mut dyn PacketReader) {
        match self {
            Unit::Planet { .. } => unreachable!(),
            Unit::ClassicShipPlayerUnit { player_unit, .. } => player_unit.update_movement(reader),
            Unit::Shot(shot) => shot.update_movement(reader),
            Unit::Explosion(explosion) => explosion.update_movement(reader),
        }
    }

    pub fn base(&self) -> &UnitBase {
        match self {
            Unit::Planet { base, .. } => base,
            Unit::ClassicShipPlayerUnit { base, .. } => &base,
            Unit::Shot(shot) => shot.base(),
            Unit::Explosion(explosion) => explosion.base(),
        }
    }

    pub fn as_steady_unit(&self) -> Option<&SteadyUnit> {
        match self {
            Unit::Planet { steady, .. } => Some(steady),
            _ => None,
        }
    }

    pub fn as_player_unit(&self) -> Option<&PlayerUnit> {
        match self {
            Unit::ClassicShipPlayerUnit { player_unit, .. } => Some(player_unit),
            _ => None,
        }
    }

    pub fn as_shot(&self) -> Option<&Shot> {
        if let Self::Shot(shot) = self {
            Some(shot)
        } else {
            None
        }
    }

    pub fn as_explosion(&self) -> Option<&Explosion> {
        if let Self::Explosion(explosion) = self {
            Some(explosion)
        } else {
            None
        }
    }
}

impl NamedUnit for Unit {
    #[inline]
    fn name(&self) -> impl Deref<Target = str> + '_ {
        self.base().name()
    }
}
