mod kind;
pub use kind::*;

mod base;
pub use base::*;

mod steady;
pub use steady::*;

mod mobility;
pub use mobility::*;

mod sun;
pub use sun::*;

mod black_hole;
pub use black_hole::*;

mod moon;
pub use moon::*;

mod meteoroid;
pub use meteoroid::*;

mod buoy;
pub use buoy::*;

mod planet;
pub use planet::*;

mod player_unit;
pub use player_unit::*;

mod classic_ship_player_unit;
pub use classic_ship_player_unit::*;

mod explosion;
pub use explosion::*;

mod shot;
pub use shot::*;

use crate::galaxy_hierarchy::{AsPlayerUnit, AsSteadyUnit, AsUnitBase, Cluster, Team};
use crate::network::PacketReader;
use crate::Vector;
use std::sync::{Arc, Weak};

/// Represents a unit in Flattiverse. Each unit in a Cluster derives from this class. This enum
/// has properties and methods which most units have in common. Specialized properties and methods
/// are stored in the relevant variant or added via interfaces.
#[derive(Debug, Clone)]
pub enum Unit {
    Sun(Sun),
    BlackHole(BlackHole),
    Moon(Moon),
    Meteoroid(Meteoroid),
    Buoy(Buoy),
    Planet(Planet),
    ClassicShipPlayerUnit(ClassicShipPlayerUnit),
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
            UnitKind::Sun => Some(Unit::Sun(Sun::read(cluster, name, reader))),
            UnitKind::BlackHole => Some(Unit::BlackHole(BlackHole::read(cluster, name, reader))),
            UnitKind::Moon => Some(Unit::Moon(Moon::read(cluster, name, reader))),
            UnitKind::Meteoroid => Some(Unit::Meteoroid(Meteoroid::read(cluster, name, reader))),
            UnitKind::Buoy => Some(Unit::Buoy(Buoy::read(cluster, name, reader))),
            UnitKind::Planet => Some(Unit::Planet(Planet::read(cluster, name, reader))),
            UnitKind::Shot => Some(Unit::Shot(Shot::read(cluster, name, reader))),
            UnitKind::ClassicShipPlayerUnit => Some(Unit::ClassicShipPlayerUnit(
                ClassicShipPlayerUnit::read(cluster, name, reader),
            )),
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
            Unit::Sun(sun) => sun.as_steady_unit().radius(),
            Unit::BlackHole(bh) => bh.as_steady_unit().radius(),
            Unit::Moon(moon) => moon.as_steady_unit().radius(),
            Unit::Meteoroid(meteoroid) => meteoroid.as_steady_unit().radius(),
            Unit::Buoy(buoy) => buoy.as_steady_unit().radius(),
            Unit::Planet(planet) => planet.as_steady_unit().radius(),
            Unit::ClassicShipPlayerUnit(cs) => cs.radius(),
            Unit::Shot(shot) => shot.radius(),
            Unit::Explosion(explosion) => explosion.radius(),
        }
    }

    /// The position of the unit.
    pub fn position(&self) -> Vector {
        match self {
            Unit::Sun(sun) => sun.as_steady_unit().position(),
            Unit::BlackHole(bh) => bh.as_steady_unit().position(),
            Unit::Moon(moon) => moon.as_steady_unit().position(),
            Unit::Meteoroid(meteoroid) => meteoroid.as_steady_unit().position(),
            Unit::Buoy(buoy) => buoy.as_steady_unit().position(),
            Unit::Planet(planet) => planet.as_steady_unit().position(),
            Unit::ClassicShipPlayerUnit(cs) => cs.as_player_unit().position(),
            Unit::Shot(shot) => shot.position(),
            Unit::Explosion(explosion) => explosion.position(),
        }
    }

    /// The movement of the unit.
    pub fn movement(&self) -> Vector {
        match self {
            Unit::ClassicShipPlayerUnit(cs) => cs.as_player_unit().movement(),
            Unit::Shot(shot) => shot.movement(),
            _ => Vector::default(),
        }
    }

    /// The direction the unit is looking into.
    pub fn angle(&self) -> f32 {
        match self {
            Unit::ClassicShipPlayerUnit(cs) => cs.as_player_unit().angle(),
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
            Unit::ClassicShipPlayerUnit(cs) => cs.gravity(),
            Unit::Explosion(e) => e.gravity(),
            _ => {
                if let Some(steady) = self.as_steady_unit() {
                    steady.gravity()
                } else {
                    0.0
                }
            }
        }
    }

    /// The mobility of this unit.
    #[inline]
    pub fn mobility(&self) -> Mobility {
        match self {
            Unit::ClassicShipPlayerUnit(cs) => cs.as_player_unit().mobility(),
            Unit::Shot(shot) => shot.mobility(),
            _ => Mobility::Still,
        }
    }

    /// The kind of the unit for a better match() experience.
    #[inline]
    pub fn kind(&self) -> UnitKind {
        match self {
            Unit::Sun(_) => UnitKind::Sun,
            Unit::BlackHole(_) => UnitKind::BlackHole,
            Unit::Moon(_) => UnitKind::Moon,
            Unit::Meteoroid(_) => UnitKind::Meteoroid,
            Unit::Buoy(_) => UnitKind::Buoy,
            Unit::Planet(_) => UnitKind::Planet,
            Unit::ClassicShipPlayerUnit(_) => UnitKind::ClassicShipPlayerUnit,
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
            Unit::Sun(_) => Weak::default(),
            Unit::BlackHole(_) => Weak::default(),
            Unit::Moon(_) => Weak::default(),
            Unit::Meteoroid(_) => Weak::default(),
            Unit::Buoy(_) => Weak::default(),
            Unit::Planet(_) => Weak::default(),
            Unit::ClassicShipPlayerUnit(cs) => Arc::downgrade(&cs.as_player_unit().player().team()),
            Unit::Shot(shot) => shot.team(),
            Unit::Explosion(explosion) => explosion.team(),
        }
    }

    pub(crate) fn update_movement(&self, reader: &mut dyn PacketReader) {
        match self {
            Unit::Sun(_) => unreachable!(),
            Unit::BlackHole(_) => unreachable!(),
            Unit::Moon(_) => unreachable!(),
            Unit::Meteoroid(_) => unreachable!(),
            Unit::Buoy(_) => unreachable!(),
            Unit::Planet(_) => unreachable!(),
            Unit::ClassicShipPlayerUnit(cs) => cs.as_player_unit().update_movement(reader),
            Unit::Shot(shot) => shot.update_movement(reader),
            Unit::Explosion(explosion) => explosion.update_movement(reader),
        }
    }

    pub fn base(&self) -> &UnitBase {
        match self {
            Unit::Sun(sun) => sun.as_unit_base(),
            Unit::BlackHole(bh) => bh.as_unit_base(),
            Unit::Moon(moon) => moon.as_unit_base(),
            Unit::Meteoroid(meteoroid) => meteoroid.as_unit_base(),
            Unit::Buoy(buoy) => buoy.as_unit_base(),
            Unit::Planet(planet) => planet.as_unit_base(),
            Unit::ClassicShipPlayerUnit(cs) => cs.as_unit_base(),
            Unit::Shot(shot) => shot.as_unit_base(),
            Unit::Explosion(explosion) => explosion.as_unit_base(),
        }
    }

    pub fn as_steady_unit(&self) -> Option<&SteadyUnit> {
        match self {
            Unit::Sun(sun) => Some(sun.as_steady_unit()),
            Unit::BlackHole(bh) => Some(bh.as_steady_unit()),
            Unit::Moon(moon) => Some(moon.as_steady_unit()),
            Unit::Meteoroid(meteoroid) => Some(meteoroid.as_steady_unit()),
            Unit::Buoy(buoy) => Some(buoy.as_steady_unit()),
            Unit::Planet(planet) => Some(planet.as_steady_unit()),
            Unit::ClassicShipPlayerUnit(_) => None,
            Unit::Shot(_) => None,
            Unit::Explosion(_) => None,
        }
    }

    pub fn as_sun(&self) -> Option<&Sun> {
        if let Self::Sun(sun) = self {
            Some(sun)
        } else {
            None
        }
    }

    pub fn as_black_hole(&self) -> Option<&BlackHole> {
        if let Self::BlackHole(bh) = self {
            Some(bh)
        } else {
            None
        }
    }

    pub fn as_moon(&self) -> Option<&Moon> {
        if let Self::Moon(moon) = self {
            Some(moon)
        } else {
            None
        }
    }

    pub fn as_meteoroid(&self) -> Option<&Meteoroid> {
        if let Self::Meteoroid(meteoroid) = self {
            Some(meteoroid)
        } else {
            None
        }
    }

    pub fn as_buoy(&self) -> Option<&Buoy> {
        if let Self::Buoy(buoy) = self {
            Some(buoy)
        } else {
            None
        }
    }

    pub fn as_planet(&self) -> Option<&Planet> {
        if let Self::Planet(planet) = self {
            Some(planet)
        } else {
            None
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

    pub fn as_player_unit(&self) -> Option<&PlayerUnit> {
        match self {
            Unit::Sun(_) => None,
            Unit::BlackHole(_) => None,
            Unit::Moon(_) => None,
            Unit::Meteoroid(_) => None,
            Unit::Buoy(_) => None,
            Unit::Planet(_) => None,
            Unit::Shot(_) => None,
            Unit::Explosion(_) => None,
            Unit::ClassicShipPlayerUnit(cs) => Some(cs.as_player_unit()),
        }
    }

    pub fn as_classic_ship_player_unit(&self) -> Option<&ClassicShipPlayerUnit> {
        if let Self::ClassicShipPlayerUnit(cs) = self {
            Some(cs)
        } else {
            None
        }
    }
}

impl AsRef<UnitBase> for Unit {
    #[inline]
    fn as_ref(&self) -> &UnitBase {
        self.base()
    }
}
