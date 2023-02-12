use crate::events::ApplicableEvent;
use crate::players::Player;
use crate::universe_group::UniverseGroup;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

/// This event contains all information about a [`Player`].
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullUpdatePlayerEvent {
    #[serde(flatten)]
    pub player: Player,
}

impl ApplicableEvent<UniverseGroup> for FullUpdatePlayerEvent {
    fn apply(mut self, group: &mut UniverseGroup) {
        let id = self.player.id;
        self.player.connection = Arc::downgrade(&group.connection);
        group.players[id.0] = Some(self.player);
    }
}
