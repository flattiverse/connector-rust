use crate::controllable::ControllableId;
use crate::players::PlayerId;
use serde_derive::{Deserialize, Serialize};

/// This event informs of the removal of a unit from the [`UniverseGroup`].
///
/// [`UniverseGroup`]: crate::universe_group::UniverseGroup
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemovedUnitEvent {
    pub name: String,
    /// The player that controls the unit, if applicable
    pub player: Option<PlayerId>,
    /// The controllable of the player, if applicable
    pub controllable: Option<ControllableId>,
}
