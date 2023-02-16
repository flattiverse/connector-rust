use crate::controllable::ControllableId;
use serde_derive::{Deserialize, Serialize};

/// Notifies about the depletion and possible overuse of a resource of your [`Controllable`].
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepletedResourceEvent {
    pub universe: usize,
    #[serde(rename = "controllableID")]
    pub controllable: ControllableId,
    #[serde(default, rename = "energyOveruse")]
    pub energy_overuse: f64,
    #[serde(default, rename = "particleOveruse")]
    pub particle_overuse: f64,
}
