use crate::hierarchy::ClusterId;
use crate::unit::{Mobility, UnitKind};
use crate::{NamedUnit, TeamId, Vector};
use std::fmt::Debug;

/// Represents an unit in Flattiverse. Each [`Unit`] in a [`crate::hierarchy::Cluster`] derives from
/// this type. The [`Unit`] declares methods which all units have in common. Derived types implement
/// those methods and might add futher propeties.
pub trait Unit: Debug + NamedUnit {
    /// The name of this [`Unit`]. The name can't be changed after it has been setup.
    fn name(&self) -> &str;

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
    #[inline]
    fn position(&self) -> Vector {
        Vector::default()
    }

    /// The gravity this [`Unit`] has on others.
    #[inline]
    fn gravity(&self) -> f64 {
        0.0
    }

    /// The radius of this [`Unit`].
    #[inline]
    fn radius(&self) -> f64 {
        1.0
    }

    /// This factor will be multiplied with the distance of the [`Unit`] to match, to determine
    /// whether you can see it. The vlaue `0.9` means you can see the unit 10% worse than with 100%.
    #[inline]
    fn visible_range_multiplier(&self) -> f64 {
        1.0
    }

    /// Specifies this movement kind this [`Unit`] is of.
    #[inline]
    fn mobility(&self) -> Mobility {
        Mobility::default()
    }

    /// Specifies the current [`crate::Team`] this [`Unit`] belongs to.
    #[inline]
    fn team(&self) -> Option<TeamId> {
        None
    }

    /// Specifies the [`UnitKind`] of this [`Unit`], which can be used to determin the [downcasting]
    /// target.
    ///
    /// [downcasting]: std::any::Any
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::default()
    }
}
