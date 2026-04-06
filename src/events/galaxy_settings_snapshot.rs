use crate::galaxy_hierarchy::{Galaxy, GameMode};

/// Immutable snapshot of all server-driven galaxy setting values mirrored by the connector.
#[derive(Debug, Clone)]
pub struct GalaxySettingsSnapshot {
    /// Active game mode.
    pub game_mode: GameMode,
    /// Galaxy name.
    pub name: String,
    /// Galaxy description.
    pub description: String,
    /// Maximum connected players.
    pub max_players: u8,
    /// Maximum connected spectators.
    pub max_spectators: u16,
    /// Maximum total ships for the whole galaxy.
    pub galaxy_max_total_ships: u16,
    /// Maximum classic ships for the whole galaxy.
    pub galaxy_max_classic_ships: u16,
    /// Maximum new ships for the whole galaxy.
    pub galaxy_max_new_ships: u16,
    /// Maximum bases for the whole galaxy.
    pub galaxy_max_bases: u16,
    /// Maximum total ships per team.
    pub team_max_total_ships: u16,
    /// Maximum classic ships per team.
    pub team_max_classic_ships: u16,
    /// Maximum new ships per team.
    pub team_max_new_ships: u16,
    /// Maximum bases per team.
    pub team_max_bases: u16,
    /// Maximum total ships per player.
    pub player_max_total_ships: u8,
    /// Maximum classic ships per player.
    pub player_max_classic_ships: u8,
    /// Maximum new ships per player.
    pub player_max_new_ships: u8,
    /// Maximum bases per player.
    pub player_max_bases: u8,
    /// Maintenance mode flag.
    pub maintenance: bool,
    /// Whether regular player logins must provide runtime and build self-disclosure.
    pub requires_self_disclosure: bool,
}

impl From<&Galaxy> for GalaxySettingsSnapshot {
    fn from(galaxy: &Galaxy) -> Self {
        Self {
            game_mode: galaxy.game_mode(),
            name: galaxy.name().to_string(),
            description: galaxy.description().to_string(),
            max_players: galaxy.max_players(),
            max_spectators: galaxy.max_spectators(),
            galaxy_max_total_ships: galaxy.galaxy_max_total_ships(),
            galaxy_max_classic_ships: galaxy.galaxy_max_classic_ships(),
            galaxy_max_new_ships: galaxy.galaxy_max_new_ships(),
            galaxy_max_bases: galaxy.galaxy_max_bases(),
            team_max_total_ships: galaxy.team_max_total_ships(),
            team_max_classic_ships: galaxy.team_max_classic_ships(),
            team_max_new_ships: galaxy.team_max_new_ships(),
            team_max_bases: galaxy.team_max_bases(),
            player_max_total_ships: galaxy.player_max_total_ships(),
            player_max_classic_ships: galaxy.player_max_classic_ships(),
            player_max_new_ships: galaxy.player_max_new_ships(),
            player_max_bases: galaxy.player_max_bases(),
            maintenance: galaxy.maintenance(),
            requires_self_disclosure: galaxy.requires_self_disclosure(),
        }
    }
}
