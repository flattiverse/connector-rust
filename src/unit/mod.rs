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

mod mission_target;
pub use mission_target::*;

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
    MissionTarget(MissionTarget),
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
            UnitKind::MissionTarget => Some(Unit::MissionTarget(MissionTarget::read(
                cluster, name, reader,
            ))),
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

    pub(crate) fn update_movement(&self, reader: &mut dyn PacketReader) {
        match self {
            Unit::Sun(_) => unreachable!(),
            Unit::BlackHole(_) => unreachable!(),
            Unit::Moon(_) => unreachable!(),
            Unit::Meteoroid(_) => unreachable!(),
            Unit::Buoy(_) => unreachable!(),
            Unit::MissionTarget(_) => unreachable!(),
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
            Unit::MissionTarget(mt) => mt.as_unit_base(),
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
            Unit::MissionTarget(mt) => Some(mt.as_steady_unit()),
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

    pub fn as_mission_target(&self) -> Option<&MissionTarget> {
        if let Self::MissionTarget(mt) = self {
            Some(mt)
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
            Unit::MissionTarget(_) => None,
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

pub(crate) trait UnitExtSealed<'a>
where
    Self: 'a,
{
    type Parent: UnitExt<'a>;

    fn parent(self) -> Self::Parent;
}

#[allow(private_bounds)]
pub trait UnitExt<'a>: UnitExtSealed<'a>
where
    Self: 'a,
{
    /// This is the name of the unit. A unit can't change her name after it has been set up.
    fn name(self) -> &'a str
    where
        Self: Sized,
    {
        self.parent().name()
    }

    /// The radius of the unit.
    #[inline]
    fn radius(self) -> f32
    where
        Self: Sized,
    {
        self.parent().radius()
    }

    /// The position of the unit.
    #[inline]
    fn position(self) -> Vector
    where
        Self: Sized,
    {
        self.parent().position()
    }

    /// The movement of the unit
    #[inline]
    fn movement(self) -> Vector
    where
        Self: Sized,
    {
        self.parent().movement()
    }

    /// The direction the unit is looking into.
    #[inline]
    fn angle(self) -> f32
    where
        Self: Sized,
    {
        self.parent().angle()
    }

    /// If true, other units can hide behind this unit.
    #[inline]
    fn is_masking(self) -> bool
    where
        Self: Sized,
    {
        self.parent().is_masking()
    }

    /// If true, a crash with this unit is lethal.
    #[inline]
    fn is_solid(self) -> bool
    where
        Self: Sized,
    {
        self.parent().is_solid()
    }

    /// If true, the unit can be edited via map editor calls.
    #[inline]
    fn can_be_edited(self) -> bool
    where
        Self: Sized,
    {
        self.parent().can_be_edited()
    }

    /// The gravity of this unit. This is how much this unit pulls others towards it.
    #[inline]
    fn gravity(self) -> f32
    where
        Self: Sized,
    {
        self.parent().gravity()
    }

    /// The mobility of the unit.
    #[inline]
    fn mobility(self) -> Mobility
    where
        Self: Sized,
    {
        self.parent().mobility()
    }

    /// The kind of the unit for a better match experience.
    #[inline]
    fn kind(self) -> UnitKind
    where
        Self: Sized,
    {
        self.parent().kind()
    }

    /// The cluster the unit is in.
    #[inline]
    fn cluster(self) -> Arc<Cluster>
    where
        Self: Sized,
    {
        self.parent().cluster()
    }

    /// The team of the unit.
    #[inline]
    fn team(self) -> Weak<Team>
    where
        Self: Sized,
    {
        self.parent().team()
    }
}

impl<'a> UnitExtSealed<'a> for &'a Unit {
    type Parent = &'a Unit;

    #[inline]
    fn parent(self) -> Self::Parent {
        unreachable!()
    }
}

impl<'a> UnitExt<'a> for &'a Unit {
    fn name(self) -> &'a str {
        match self {
            Unit::Sun(u) => u.name(),
            Unit::BlackHole(u) => u.name(),
            Unit::Moon(u) => u.name(),
            Unit::Meteoroid(u) => u.name(),
            Unit::Buoy(u) => u.name(),
            Unit::MissionTarget(u) => u.name(),
            Unit::Planet(u) => u.name(),
            Unit::ClassicShipPlayerUnit(u) => u.name(),
            Unit::Shot(u) => u.name(),
            Unit::Explosion(u) => u.name(),
        }
    }

    fn radius(self) -> f32 {
        match self {
            Unit::Sun(u) => u.radius(),
            Unit::BlackHole(u) => u.radius(),
            Unit::Moon(u) => u.radius(),
            Unit::Meteoroid(u) => u.radius(),
            Unit::Buoy(u) => u.radius(),
            Unit::MissionTarget(u) => u.radius(),
            Unit::Planet(u) => u.radius(),
            Unit::ClassicShipPlayerUnit(u) => u.radius(),
            Unit::Shot(u) => u.radius(),
            Unit::Explosion(u) => u.radius(),
        }
    }

    fn position(self) -> Vector {
        match self {
            Unit::Sun(u) => u.position(),
            Unit::BlackHole(u) => u.position(),
            Unit::Moon(u) => u.position(),
            Unit::Meteoroid(u) => u.position(),
            Unit::Buoy(u) => u.position(),
            Unit::MissionTarget(u) => u.position(),
            Unit::Planet(u) => u.position(),
            Unit::ClassicShipPlayerUnit(u) => u.position(),
            Unit::Shot(u) => u.position(),
            Unit::Explosion(u) => u.position(),
        }
    }

    fn movement(self) -> Vector {
        match self {
            Unit::Sun(u) => u.movement(),
            Unit::BlackHole(u) => u.movement(),
            Unit::Moon(u) => u.movement(),
            Unit::Meteoroid(u) => u.movement(),
            Unit::Buoy(u) => u.movement(),
            Unit::MissionTarget(u) => u.movement(),
            Unit::Planet(u) => u.movement(),
            Unit::ClassicShipPlayerUnit(u) => u.movement(),
            Unit::Shot(u) => u.movement(),
            Unit::Explosion(u) => u.movement(),
        }
    }

    fn angle(self) -> f32 {
        match self {
            Unit::Sun(u) => u.angle(),
            Unit::BlackHole(u) => u.angle(),
            Unit::Moon(u) => u.angle(),
            Unit::Meteoroid(u) => u.angle(),
            Unit::Buoy(u) => u.angle(),
            Unit::MissionTarget(u) => u.angle(),
            Unit::Planet(u) => u.angle(),
            Unit::ClassicShipPlayerUnit(u) => u.angle(),
            Unit::Shot(u) => u.angle(),
            Unit::Explosion(u) => u.angle(),
        }
    }

    fn is_masking(self) -> bool {
        match self {
            Unit::Sun(u) => u.is_masking(),
            Unit::BlackHole(u) => u.is_masking(),
            Unit::Moon(u) => u.is_masking(),
            Unit::Meteoroid(u) => u.is_masking(),
            Unit::Buoy(u) => u.is_masking(),
            Unit::MissionTarget(u) => u.is_masking(),
            Unit::Planet(u) => u.is_masking(),
            Unit::ClassicShipPlayerUnit(u) => u.is_masking(),
            Unit::Shot(u) => u.is_masking(),
            Unit::Explosion(u) => u.is_masking(),
        }
    }

    fn is_solid(self) -> bool {
        match self {
            Unit::Sun(u) => u.is_solid(),
            Unit::BlackHole(u) => u.is_solid(),
            Unit::Moon(u) => u.is_solid(),
            Unit::Meteoroid(u) => u.is_solid(),
            Unit::Buoy(u) => u.is_solid(),
            Unit::MissionTarget(u) => u.is_solid(),
            Unit::Planet(u) => u.is_solid(),
            Unit::ClassicShipPlayerUnit(u) => u.is_solid(),
            Unit::Shot(u) => u.is_solid(),
            Unit::Explosion(u) => u.is_solid(),
        }
    }

    fn can_be_edited(self) -> bool {
        match self {
            Unit::Sun(u) => u.can_be_edited(),
            Unit::BlackHole(u) => u.can_be_edited(),
            Unit::Moon(u) => u.can_be_edited(),
            Unit::Meteoroid(u) => u.can_be_edited(),
            Unit::Buoy(u) => u.can_be_edited(),
            Unit::MissionTarget(u) => u.can_be_edited(),
            Unit::Planet(u) => u.can_be_edited(),
            Unit::ClassicShipPlayerUnit(u) => u.can_be_edited(),
            Unit::Shot(u) => u.can_be_edited(),
            Unit::Explosion(u) => u.can_be_edited(),
        }
    }

    fn gravity(self) -> f32 {
        match self {
            Unit::Sun(u) => u.gravity(),
            Unit::BlackHole(u) => u.gravity(),
            Unit::Moon(u) => u.gravity(),
            Unit::Meteoroid(u) => u.gravity(),
            Unit::Buoy(u) => u.gravity(),
            Unit::MissionTarget(u) => u.gravity(),
            Unit::Planet(u) => u.gravity(),
            Unit::ClassicShipPlayerUnit(u) => u.gravity(),
            Unit::Shot(u) => u.gravity(),
            Unit::Explosion(u) => u.gravity(),
        }
    }

    fn mobility(self) -> Mobility {
        match self {
            Unit::Sun(u) => u.mobility(),
            Unit::BlackHole(u) => u.mobility(),
            Unit::Moon(u) => u.mobility(),
            Unit::Meteoroid(u) => u.mobility(),
            Unit::Buoy(u) => u.mobility(),
            Unit::MissionTarget(u) => u.mobility(),
            Unit::Planet(u) => u.mobility(),
            Unit::ClassicShipPlayerUnit(u) => u.mobility(),
            Unit::Shot(u) => u.mobility(),
            Unit::Explosion(u) => u.mobility(),
        }
    }

    fn kind(self) -> UnitKind {
        match self {
            Unit::Sun(u) => u.kind(),
            Unit::BlackHole(u) => u.kind(),
            Unit::Moon(u) => u.kind(),
            Unit::Meteoroid(u) => u.kind(),
            Unit::Buoy(u) => u.kind(),
            Unit::MissionTarget(u) => u.kind(),
            Unit::Planet(u) => u.kind(),
            Unit::ClassicShipPlayerUnit(u) => u.kind(),
            Unit::Shot(u) => u.kind(),
            Unit::Explosion(u) => u.kind(),
        }
    }

    fn cluster(self) -> Arc<Cluster> {
        match self {
            Unit::Sun(u) => u.cluster(),
            Unit::BlackHole(u) => u.cluster(),
            Unit::Moon(u) => u.cluster(),
            Unit::Meteoroid(u) => u.cluster(),
            Unit::Buoy(u) => u.cluster(),
            Unit::MissionTarget(u) => u.cluster(),
            Unit::Planet(u) => u.cluster(),
            Unit::ClassicShipPlayerUnit(u) => u.cluster(),
            Unit::Shot(u) => u.cluster(),
            Unit::Explosion(u) => u.cluster(),
        }
    }

    fn team(self) -> Weak<Team> {
        match self {
            Unit::Sun(u) => u.team(),
            Unit::BlackHole(u) => u.team(),
            Unit::Moon(u) => u.team(),
            Unit::Meteoroid(u) => u.team(),
            Unit::Buoy(u) => u.team(),
            Unit::MissionTarget(u) => u.team(),
            Unit::Planet(u) => u.team(),
            Unit::ClassicShipPlayerUnit(u) => u.team(),
            Unit::Shot(u) => u.team(),
            Unit::Explosion(u) => u.team(),
        }
    }
}
