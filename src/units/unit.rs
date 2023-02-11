use crate::team::TeamId;
use crate::units::mobility::Mobility;
use crate::units::unit_kind::UnitKind;
use crate::vector::Vector;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    /// The name of the unit.
    pub name: String,
    /// The radius of the unit
    pub radius: f64,
    /// The absolute position of the unit.
    pub position: Vector,
    /// The movement vector of the unit.
    pub movement: Vector,
    /// The direction this unit is facing towards.
    pub direction: f64,
    /// The [`Team`] this unit belongs to, if any. Referenced through its id.
    ///
    /// [`Team`]: crate::team::Team
    pub team: Option<TeamId>,
    /// The gravity exercised by this unit.
    pub gravity: f64,
    #[serde(flatten)]
    pub kind: UnitKind,
}

impl Unit {
    /// The [`Mobility`] status of this unit.
    pub fn mobility(&self) -> Mobility {
        todo!()
    }

    /// The energy output of this unit.
    pub fn energy_output(&self) -> f64 {
        todo!()
    }

    /// Whether this unit is masking.
    pub fn is_masking(&self) -> bool {
        todo!()
    }

    /// Whether this unit is solid
    pub fn is_solid(&self) -> bool {
        todo!()
    }

    /// Whether it is possible to edit this unit vai admin commands.
    pub fn is_map_editable(&self) -> bool {
        todo!()
    }
}
