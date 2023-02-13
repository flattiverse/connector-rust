use crate::controllable::{ControllableId, ControllableState};
use crate::universe_group::UniverseGroup;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatedControllableEvent {
    pub universe: usize,
    #[serde(rename = "controllableID")]
    pub controllable: ControllableId,
    #[serde(rename = "controllable")]
    pub controllable_state: ControllableState,
}

impl UpdatedControllableEvent {
    pub(crate) async fn apply(self, group: &mut UniverseGroup) {
        let id = self.controllable;
        group[id].update_state(self.controllable_state).await;
    }
}
