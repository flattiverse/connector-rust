use crate::events::ApplicableEvent;
use crate::players::{Player, PlayerId};
use crate::universe_group::UniverseGroup;
use serde_derive::{Deserialize, Serialize};

/// This event contains only mutable information about a [`Player`].
#[derive(Debug, Serialize, Deserialize)]
pub struct PartialUpdatePlayerEvent {
    pub id: PlayerId,
    #[serde(rename = "pvpScore")]
    pub pvp_score: f64,
    pub deaths: u64,
    pub collisions: u64,
    pub kills: u64,
    pub rank: i32,
}

impl ApplicableEvent<UniverseGroup> for PartialUpdatePlayerEvent {
    fn apply(self, group: &mut UniverseGroup) {
        let player = group.players[self.id.0].as_mut().unwrap();
        self.apply(player);
    }
}

impl ApplicableEvent<Player> for PartialUpdatePlayerEvent {
    fn apply(self, player: &mut Player) {
        player.pvp_score = self.pvp_score;
        player.deaths = self.deaths;
        player.collisions = self.collisions;
        player.kills = self.kills;
        player.rank = self.rank;
    }
}
