use crate::events::ApplicableEvent;
use crate::players::Player;
use crate::universe_group::UniverseGroup;
use serde_derive::{Deserialize, Serialize};

/// This event contains all information about a [`Player`].
#[derive(Debug, Serialize, Deserialize)]
pub struct FullUpdatePlayerEvent {
    #[serde(flatten)]
    pub player: Player,
}

impl ApplicableEvent<UniverseGroup> for FullUpdatePlayerEvent {
    fn apply(self, group: &mut UniverseGroup) {
        let id = self.player.id;
        group.players[id.0] = Some(self.player);
    }
}
