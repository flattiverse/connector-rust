use crate::network::PacketReader;
use crate::player_kind::PlayerKind;

#[derive(Debug)]
pub struct Player {
    pub id: u8,
    pub name: String,
    pub kind: PlayerKind,
    pub team: u8,
}

impl Player {
    pub fn new(id: u8, kind: PlayerKind, team: u8, reader: &mut dyn PacketReader) -> Self {
        Self {
            id,
            kind,
            team,
            name: reader.read_string(),
        }
    }
}
