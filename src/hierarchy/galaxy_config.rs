use crate::network::{PacketReader, PacketWriter};
use crate::utils::check_name_or_err_32;
use crate::{GameError, GameType};
use num_enum::FromPrimitive;

#[derive(Debug, Clone, Default)]
pub struct GalaxyConfig {
    pub name: String,
    pub description: String,
    pub game_type: GameType,
    pub max_players: u8,

    pub max_platforms_galaxy: u16,
    pub max_probes_galaxy: u16,
    pub max_drones_galaxy: u16,
    pub max_ships_galaxy: u16,
    pub max_bases_galaxy: u16,

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

impl From<&mut dyn PacketReader> for GalaxyConfig {
    fn from(reader: &mut dyn PacketReader) -> Self {
        let mut this = Self::default();
        this.read(reader);
        this
    }
}

impl GalaxyConfig {
    pub(crate) fn read(&mut self, reader: &mut dyn PacketReader) {
        self.name = reader.read_string();
        self.description = reader.read_string();
        self.game_type = GameType::from_primitive(reader.read_byte());
        self.max_players = reader.read_byte();
        self.max_platforms_galaxy = reader.read_uint16();
        self.max_probes_galaxy = reader.read_uint16();
        self.max_drones_galaxy = reader.read_uint16();
        self.max_ships_galaxy = reader.read_uint16();
        self.max_bases_galaxy = reader.read_uint16();
        self.max_platforms_team = reader.read_uint16();
        self.max_probes_team = reader.read_uint16();
        self.max_drones_team = reader.read_uint16();
        self.max_ships_team = reader.read_uint16();
        self.max_bases_team = reader.read_uint16();
        self.max_platforms_player = reader.read_byte();
        self.max_probes_player = reader.read_byte();
        self.max_drones_player = reader.read_byte();
        self.max_ships_player = reader.read_byte();
        self.max_bases_player = reader.read_byte();
    }

    pub(crate) fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_string(&self.name);
        writer.write_string(&self.description);
        writer.write_byte(self.game_type as u8);
        writer.write_byte(self.max_players);
        writer.write_uint16(self.max_platforms_galaxy);
        writer.write_uint16(self.max_probes_galaxy);
        writer.write_uint16(self.max_drones_galaxy);
        writer.write_uint16(self.max_ships_galaxy);
        writer.write_uint16(self.max_bases_galaxy);
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

    /// The name of the configured [`crate::hierarchy::Galaxy`].
    pub fn set_name(&mut self, name: impl Into<String>) -> Result<(), GameError> {
        self.name = check_name_or_err_32(name)?;
        Ok(())
    }
}
