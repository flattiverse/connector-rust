use crate::network::PacketReader;
use crate::player_kind::PlayerKind;

#[derive(Debug)]
pub struct Player {
    id: u8,
    name: String,
    kind: PlayerKind,
    team: u8,
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

    #[inline]
    pub fn id(&self) -> u8 {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn kind(&self) -> PlayerKind {
        self.kind
    }

    #[inline]
    pub fn team(&self) -> u8 {
        self.team
    }
}
