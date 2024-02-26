use crate::hierarchy::ClusterId;
use crate::network::{ConnectionHandle, PacketReader};
use crate::unit::{BlackHole, Buoy, Meteoroid, PlayerUnit};
use crate::unit::{Mobility, Moon, Planet, Sun, UnitKind};
use crate::{GameError, NamedUnit, TeamId, Vector};
use std::any::Any;
use std::fmt::{Debug, Display, Formatter};

/// Represents a unit in Flattiverse. Each [`Unit`] in a [`crate::hierarchy::Cluster`] derives from
/// this type. The [`Unit`] declares methods which all units have in common. Derived types implement
/// those methods and might add futher propeties.
pub trait Unit: Any + Debug {
    /// The name of this [`Unit`]. The name can't be changed after it has been setup.
    fn name(&self) -> &str;

    //    /// Indicates whether this [`Unit`] is still active. A [`Unit`] is active as long as it is
    //    /// visible to the current player. If this [`Unit`] moves out of all scan areas, it is
    //    /// deactivated and [`Unit::active`] will return `false`.
    //    fn active(&self) -> bool;
    //
    //    /// For internal use only.
    //    fn deactivate(&mut self);

    /// The [`crate::hierarchy::Cluster`] this [`Unit`] is in.
    fn cluster(&self) -> ClusterId;

    /// Specifies whether this [`Unit`] can hide othe [`Unit`]s behind it. True means you can't see
    /// [`Unit`] behind this [`Unit`] when scanning.
    #[inline]
    fn is_masking(&self) -> bool {
        true
    }

    /// Speifies whether this [`Unit`] can collide with another [`Unit`].
    #[inline]
    fn is_solid(&self) -> bool {
        true
    }

    /// Specifies whether this [`Unit`] can be seen by othe runits (when scanning, etc.).
    #[inline]
    fn is_visible(&self) -> bool {
        true
    }

    /// Specifies wheter this [`Unit`] can be edited by the map editor.
    #[inline]
    fn can_be_edited(&self) -> bool {
        false
    }

    /// The speed limit of this [`Unit`]. If this limit is esceded, the [`Unit`] will be slowed down
    /// by 6% of its overshooting speed.
    #[inline]
    fn speed_imit(&self) -> f64 {
        0.0
    }

    /// The direction this [`Unit`] is facing to.
    #[inline]
    fn direction(&self) -> f64 {
        0.0
    }

    /// The movement of this [`Unit`].
    #[inline]
    fn movement(&self) -> Vector {
        Vector::default()
    }

    /// The position of this [`Unit`].
    fn position(&self) -> Vector;

    /// The gravity this [`Unit`] has on others.
    fn gravity(&self) -> f64;

    /// The radius of this [`Unit`].
    fn radius(&self) -> f64;

    /// This factor will be multiplied with the distance of the [`Unit`] to match, to determine
    /// whether you can see it. The vlaue `0.9` means you can see the unit 10% worse than with 100%.
    #[inline]
    fn visible_range_multiplier(&self) -> f64 {
        1.0
    }

    /// Specifies this movement kind this [`Unit`] is of.
    #[inline]
    fn mobility(&self) -> Mobility {
        Mobility::Still
    }

    /// Specifies the current [`crate::Team`] this [`Unit`] belongs to.
    #[inline]
    fn team(&self) -> Option<TeamId> {
        None
    }

    fn update(&mut self, reader: &mut dyn PacketReader);

    /// Specifies the [`UnitKind`] of this [`Unit`], which can be used to determin the [downcasting]
    /// target.
    ///
    /// [downcasting]: std::any::Any
    fn kind(&self) -> UnitKind;

    /// Workaround for as long as `trait_upcasting` is unstable.
    fn as_any(&self) -> &dyn Any;
}

impl<T: Unit> NamedUnit for T {
    #[inline]
    fn name(&self) -> &str {
        Unit::name(self)
    }
}

impl NamedUnit for dyn Unit {
    #[inline]
    fn name(&self) -> &str {
        Unit::name(self)
    }
}

impl Display for dyn Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.kind(), self.name())
    }
}

pub(crate) fn from_packet(
    cluster: ClusterId,
    kind: UnitKind,
    reader: &mut dyn PacketReader,
    connection: ConnectionHandle,
) -> Result<Box<dyn Unit>, GameError> {
    Ok(match kind {
        UnitKind::Sun => Box::new(Sun::new(cluster, reader, connection)) as Box<dyn Unit>,
        UnitKind::BlackHole => Box::new(BlackHole::new(cluster, reader, connection)),
        UnitKind::Planet => Box::new(Planet::new(cluster, reader, connection)),
        UnitKind::Moon => Box::new(Moon::new(cluster, reader, connection)),
        UnitKind::Meteoroid => Box::new(Meteoroid::new(cluster, reader, connection)),
        UnitKind::Buoy => Box::new(Buoy::new(cluster, reader, connection)),
        UnitKind::Ship => Box::new(PlayerUnit::new(cluster, reader)),
    })
}
