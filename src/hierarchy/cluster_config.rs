use crate::network::{PacketReader, PacketWriter};

#[derive(Debug, Clone, Default)]
pub struct ClusterConfig {
    pub name: String,
}

impl From<&mut dyn PacketReader> for ClusterConfig {
    #[inline]
    fn from(reader: &mut dyn PacketReader) -> Self {
        let mut this = Self::default();
        this.read(reader);
        this
    }
}

impl ClusterConfig {
    #[inline]
    pub(crate) fn read(&mut self, reader: &mut dyn PacketReader) {
        self.name = reader.read_string();
    }

    #[inline]
    pub(crate) fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_string(&self.name);
    }
}
