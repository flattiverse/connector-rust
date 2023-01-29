use crate::game_mode::GameMode;
use crate::team::Team;
use crate::universe_group::UniverseGroup;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UniverseGroupInfoEvent {
    /// The name of the [`crate::universe_group::UniverseGroup`].
    pub name: String,
    /// The description of the [`crate::universe_group::UniverseGroup`].
    pub description: String,
    /// The [`GameMode`] of the [`crate::universe_group::UniverseGroup`].
    pub mode: GameMode,
    /// The amount of players together in the [`crate::universe_group::UniverseGroup`].
    #[serde(alias = "metrics")]
    #[serde(rename = "maxPlayers")]
    pub max_players: u32,
    /// The amount of ships a player can have in the [`crate::universe_group::UniverseGroup`].
    #[serde(alias = "metrics")]
    #[serde(rename = "maxShipsPerPlayer")]
    pub max_ships_per_player: u32,
    /// The amount of ships a team can have in the [`crate::universe_group::UniverseGroup`].
    #[serde(alias = "metrics")]
    #[serde(rename = "maxShipsPerTeam")]
    pub max_ships_per_team: u32,
    /// The amount of bases a player can have in the [`crate::universe_group::UniverseGroup`].
    #[serde(alias = "metrics")]
    #[serde(rename = "maxBasePerPlayer")]
    pub max_base_per_player: u32,
    /// The amount of bases a team can have in the [`crate::universe_group::UniverseGroup`].
    #[serde(alias = "metrics")]
    #[serde(rename = "maxBasePerTeam")]
    pub max_base_per_team: u32,
    /// True, if joining this universe as a spectator is allowed.
    #[serde(alias = "metrics")]
    pub spectators: bool,
    /// The [`Team`]s in the [`crate::universe_group::UniverseGroup`].
    pub teams: Vec<Team>,
}

impl UniverseGroupInfoEvent {
    pub fn update(self, group: &mut UniverseGroup) {
        group.name = self.name;
        group.description = self.description;
        group.mode = self.mode;
        group.max_players = self.max_players;
        group.max_ships_per_player = self.max_ships_per_player;
        group.max_ships_per_team = self.max_ships_per_team;
        group.max_base_per_player = self.max_base_per_player;
        group.max_base_per_team = self.max_base_per_team;
        group.spectators = self.spectators;
        group.teams = self.teams;
    }
}
