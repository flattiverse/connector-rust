use crate::hierarchy::Galaxy;
use crate::mission_selection::{PlayerInfo, TeamInfo, Universe};
use crate::{GameError, GameMode};

#[derive(Debug, Deserialize)]
pub struct GalaxyInfo {
    pub id: i32,
    pub name: String,
    #[serde(rename = "allowSpectating")]
    pub allow_spectating: bool,
    #[serde(rename = "gameType")]
    pub game_mode: GameMode,
    #[serde(rename = "maxPlayers")]
    pub max_players: i32,
    #[serde(rename = "maxPlatformsUniverse")]
    pub max_platforms_universe: i32,
    #[serde(rename = "maxProbesUniverse")]
    pub max_probes_universe: i32,
    #[serde(rename = "maxDronesUniverse")]
    pub max_drones_universe: i32,
    #[serde(rename = "maxShipsUniverse")]
    pub max_ships_universe: i32,
    #[serde(rename = "maxBasesUniverse")]
    pub max_bases_universe: i32,
    #[serde(rename = "maxPlatformsTeam")]
    pub max_platforms_team: i32,
    #[serde(rename = "maxProbesTeam")]
    pub max_probes_team: i32,
    #[serde(rename = "maxDronesTeam")]
    pub max_drones_team: i32,
    #[serde(rename = "maxShipsTeam")]
    pub max_ships_team: i32,
    #[serde(rename = "maxBasesTeam")]
    pub max_bases_team: i32,
    #[serde(rename = "maxPlatformsPlayer")]
    pub max_platforms_player: i32,
    #[serde(rename = "maxProbesPlayer")]
    pub max_probes_player: i32,
    #[serde(rename = "maxDronesPlayer")]
    pub max_drones_player: i32,
    #[serde(rename = "maxShipsPlayer")]
    pub max_ships_player: i32,
    #[serde(rename = "maxBasesPlayer")]
    pub max_bases_player: i32,
    #[serde(rename = "teams")]
    pub teams: Vec<TeamInfo>,
    #[serde(rename = "players")]
    pub players: Vec<PlayerInfo>,
}

impl GalaxyInfo {
    /// Joins the galaxy with the specified team.
    pub async fn join(&self, auth: &str, team: &TeamInfo) -> Result<Galaxy, GameError> {
        Universe::manual_join(
            &format!("wss://{}/game/galaxies/{}", Universe::URI_BASE, self.id),
            auth,
            team.id,
        )
        .await
    }
}
