use crate::network::{PacketReader, PacketWriter};
use crate::utils::check_name_or_err_32;
use crate::GameError;

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

    /// The name of the configured [`crate::Team`].
    pub fn set_name(&mut self, name: impl Into<String>) -> Result<(), GameError> {
        let name = name.into();
        self.name = check_name_or_err_32(name)?;
        Ok(())
    }

    #[inline]
    pub fn name_valid(&self) -> bool {
        check_name_or_err_32(&self.name).is_ok()
    }
}
