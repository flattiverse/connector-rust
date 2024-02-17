use crate::network::{PacketReader, PacketWriter};

#[derive(Debug, Clone, Default)]
pub struct RegionConfig {
    pub name: String,
    pub start_propability: f64,
    pub respawn_propability: f64,
    pub protected: bool,
}

impl From<&mut dyn PacketReader> for RegionConfig {
    fn from(reader: &mut dyn PacketReader) -> Self {
        let mut this = Self::default();
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
    }

    pub(crate) fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_string(&self.name);
        writer.write_2u(self.start_propability, 100.0);
        writer.write_2u(self.respawn_propability, 100.0);
        writer.write_boolean(self.protected);
    }
}
