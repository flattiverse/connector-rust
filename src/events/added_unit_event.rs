use crate::units::unit::Unit;
use serde_derive::{Deserialize, Serialize};

/// This event informs of the addition of a unit to the [`UniverseGroup`].
///
/// [`UniverseGroup`]: crate::universe_group::UniverseGroup
#[derive(Debug, Serialize, Deserialize)]
pub struct AddedUnitEvent {
    pub universe: usize,
    pub unit: Unit,
}
