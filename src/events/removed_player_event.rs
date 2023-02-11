use crate::events::ApplicableEvent;
use crate::players::PlayerId;
use crate::universe_group::UniverseGroup;
use serde_derive::{Deserialize, Serialize};

/// This event informs of the disconnect of a player from the [`UniverseGroup`].
#[derive(Debug, Serialize, Deserialize)]
pub struct RemovedPlayerEvent {
    pub id: PlayerId,
}

impl ApplicableEvent<UniverseGroup> for RemovedPlayerEvent {
    fn apply(self, group: &mut UniverseGroup) {
        group.players[self.id.0] = None;
    }
}
