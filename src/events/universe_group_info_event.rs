use crate::events::ApplicableEvent;
use crate::game_mode::GameMode;
use crate::team::Team;
use crate::universe::Universe;
use crate::universe_group::UniverseGroup;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

/// This event notifies about the meta information a [`UniverseGroup`] has, like name,
/// description, teams, rules... You actually don't need to parse this event because it's also
/// parsed by the connector and the results are presented in fields on the [`UniverseGroup`].
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniverseGroupInfoEvent {
    /// The name of the [`UniverseGroup`].
    pub name: String,
    /// The description of the [`UniverseGroup`].
    pub description: String,
    /// The [`GameMode`] of the [`UniverseGroup`].
    pub mode: GameMode,
    pub metrics: Metrics,
    /// The [`Team`]s in the [`UniverseGroup`].
    pub teams: Vec<Team>,
    /// The [`Universe`]s in the [`UniverseGroup`].
    universes: Vec<Universe>,
    // /// The system upgrade paths in the [`UniverseGroup`].
    // systems: HashMap<PlayerUnitSystemIdentifier, PlayerUnitSystemUpgradepath>,
}

impl ApplicableEvent<UniverseGroup> for UniverseGroupInfoEvent {
    fn apply(self, group: &mut UniverseGroup) {
        group.name = self.name;
        group.description = self.description;
        group.mode = self.mode;

        self.metrics.apply(group);

        group.teams = Default::default();
        for mut team in self.teams {
            let id = team.id.0;
            team.connection = Arc::downgrade(&group.connection);
            group.teams[id] = Some(team);
        }

        group.universes = {
            const EMPTY: Option<Universe> = None;
            [EMPTY; 64]
        };
        for mut universe in self.universes {
            let id = universe.id;
            universe.connection = Arc::downgrade(&group.connection);
            group.universes[id.0] = Some(universe);
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metrics {
    /// The amount of players together in the [`UniverseGroup`].
    #[serde(rename = "maxPlayers")]
    pub max_players: u32,
    /// The amount of ships a player can have in the [`UniverseGroup`].
    #[serde(rename = "maxShipsPerPlayer")]
    pub max_ships_per_player: u32,
    /// The amount of ships a team can have in the [`UniverseGroup`].
    #[serde(rename = "maxShipsPerTeam")]
    pub max_ships_per_team: u32,
    /// The amount of bases a player can have in the [`UniverseGroup`].
    #[serde(rename = "maxBasesPerPlayer")]
    pub max_bases_per_player: u32,
    /// The amount of bases a team can have in the [`UniverseGroup`].
    #[serde(rename = "maxBasesPerTeam")]
    pub max_bases_per_team: u32,
    /// True, if joining this universe as a spectator is allowed.
    pub spectators: bool,
    /// The amount of ships that you can register in the [`UniverseGroup`].
    #[serde(rename = "registerShipLimit")]
    pub register_ship_limit: u32,
}

impl ApplicableEvent<UniverseGroup> for Metrics {
    fn apply(self, group: &mut UniverseGroup) {
        group.max_players = self.max_players;
        group.max_ships_per_player = self.max_ships_per_player;
        group.max_ships_per_team = self.max_ships_per_team;
        group.max_bases_per_player = self.max_bases_per_player;
        group.max_bases_per_team = self.max_bases_per_team;
        group.spectators = self.spectators;
        group.register_ship_limit = self.register_ship_limit;
    }
}
