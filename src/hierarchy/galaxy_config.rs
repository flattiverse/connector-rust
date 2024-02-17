use crate::hierarchy::Galaxy;
use crate::network::PacketWriter;
use crate::GameType;

#[derive(Debug, Clone, Default)]
pub struct GalaxyConfig {
    pub name: String,
    pub description: String,
    pub game_type: GameType,
    pub max_players: u8,

    pub max_platforms_universe: u16,
    pub max_probes_universe: u16,
    pub max_drones_universe: u16,
    pub max_ships_universe: u16,
    pub max_bases_universe: u16,

    pub max_platforms_team: u16,
    pub max_probes_team: u16,
    pub max_drones_team: u16,
    pub max_ships_team: u16,
    pub max_bases_team: u16,

    pub max_platforms_player: u8,
    pub max_probes_player: u8,
    pub max_drones_player: u8,
    pub max_ships_player: u8,
    pub max_bases_player: u8,
}

impl From<&Galaxy> for GalaxyConfig {
    fn from(galaxy: &Galaxy) -> Self {
        Self {
            name: galaxy.name().to_string(),
            description: galaxy.description().to_string(),
            game_type: galaxy.game_type(),
            max_players: galaxy.max_players(),
            max_platforms_universe: galaxy.max_platforms_universe(),
            max_probes_universe: galaxy.max_probes_universe(),
            max_drones_universe: galaxy.max_drones_universe(),
            max_ships_universe: galaxy.max_ships_universe(),
            max_bases_universe: galaxy.max_bases_universe(),
            max_platforms_team: galaxy.max_platforms_team(),
            max_probes_team: galaxy.max_probes_team(),
            max_drones_team: galaxy.max_drones_team(),
            max_ships_team: galaxy.max_ships_team(),
            max_bases_team: galaxy.max_bases_team(),
            max_platforms_player: galaxy.max_platforms_player(),
            max_probes_player: galaxy.max_probes_player(),
            max_drones_player: galaxy.max_drones_player(),
            max_ships_player: galaxy.max_ships_player(),
            max_bases_player: galaxy.max_bases_player(),
        }
    }
}

impl GalaxyConfig {
    pub(crate) fn write_to(&self, writer: &mut dyn PacketWriter) {
        writer.write_string(&self.name);
        writer.write_string(&self.description);
        writer.write_byte(self.game_type as u8);
        writer.write_byte(self.max_players);
        writer.write_uint16(self.max_platforms_universe);
        writer.write_uint16(self.max_probes_universe);
        writer.write_uint16(self.max_drones_universe);
        writer.write_uint16(self.max_ships_universe);
        writer.write_uint16(self.max_bases_universe);
        writer.write_uint16(self.max_platforms_team);
        writer.write_uint16(self.max_probes_team);
        writer.write_uint16(self.max_drones_team);
        writer.write_uint16(self.max_ships_team);
        writer.write_uint16(self.max_bases_team);
        writer.write_byte(self.max_platforms_player);
        writer.write_byte(self.max_probes_player);
        writer.write_byte(self.max_drones_player);
        writer.write_byte(self.max_ships_player);
        writer.write_byte(self.max_bases_player);
    }
}
