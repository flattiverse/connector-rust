use crate::network::{PacketReader, PacketWriter};
use crate::TeamId;

#[derive(Debug, Clone)]
pub struct RegionConfig {
    pub name: String,
    pub start_propability: f64,
    pub respawn_propability: f64,
    pub protected: bool,
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub team: TeamId,
}

impl From<&mut dyn PacketReader> for RegionConfig {
    fn from(reader: &mut dyn PacketReader) -> Self {
        let mut this = Self {
            name: String::default(),
            start_propability: 0.0,
            respawn_propability: 0.0,
            protected: false,
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            team: TeamId(0),
        };
        this.read(reader);
        this
    }
}

impl RegionConfig {
    pub(crate) fn read(&mut self, reader: &mut dyn PacketReader) {
        self.name = reader.read_string();
        self.start_propability = reader.read_2u(100.0);
        self.respawn_propability = reader.read_2u(100.0);
        self.protected = reader.read_boolean();
        self.left = reader.read_2u(100.0);
        self.top = reader.read_2u(100.0);
        self.right = reader.read_2u(100.0);
        self.bottom = reader.read_2u(100.0);
        self.team = TeamId(
            reader
                .read_uint32() // TODO dont forget to fix the writer as well
                .try_into()
                .expect("TeamId is not within expected range"),
        );
    }

    pub(crate) fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_string(&self.name);
        writer.write_2u(self.start_propability, 100.0);
        writer.write_2u(self.respawn_propability, 100.0);
        writer.write_boolean(self.protected);
        writer.write_2u(self.left, 100.0);
        writer.write_2u(self.top, 100.0);
        writer.write_2u(self.right, 100.0);
        writer.write_2u(self.bottom, 100.0);
        writer.write_uint32(u32::from(self.team.0))
    }
}
