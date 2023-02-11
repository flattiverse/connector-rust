use serde_derive::{Deserialize, Serialize};

/// This event informs of the removal of a unit from the [`UniverseGroup`].
///
/// [`UniverseGroup`]: crate::universe_group::UniverseGroup
#[derive(Debug, Serialize, Deserialize)]
pub struct RemovedUnitEvent {
    pub name: String,
}
