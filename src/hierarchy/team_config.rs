use crate::network::{PacketReader, PacketWriter};

#[derive(Debug, Clone, Default)]
pub struct TeamConfig {
    pub name: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<&mut dyn PacketReader> for TeamConfig {
    fn from(reader: &mut dyn PacketReader) -> Self {
        let mut this = Self::default();
        this.read(reader);
        this
    }
}

impl TeamConfig {
    pub(crate) fn read(&mut self, reader: &mut dyn PacketReader) {
        self.name = reader.read_string();
        self.red = reader.read_byte();
        self.green = reader.read_byte();
        self.blue = reader.read_byte();
    }

    pub(crate) fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_string(&self.name);
        writer.write_byte(self.red);
        writer.write_byte(self.green);
        writer.write_byte(self.blue);
    }
}
