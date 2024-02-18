use crate::network::{PacketReader, PacketWriter};
use crate::utils::check_name_or_err;
use crate::GameError;

#[derive(Debug, Clone)]
pub struct UnitConfiguration {
    pub(crate) name: String,
}

impl Default for UnitConfiguration {
    fn default() -> Self {
        Self {
            name: "UnitName".to_string(),
        }
    }
}

impl UnitConfiguration {
    pub(crate) fn read(&mut self, reader: &mut dyn PacketReader) {
        self.name = reader.read_string();
    }

    pub(crate) fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_string(&self.name);
    }

    /// The name of the configured [`crate::unit::Unit`].
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The name of the configured [`crate::unit::Unit`].
    pub fn set_name(&mut self, name: impl Into<String>) -> Result<(), GameError> {
        self.name = check_name_or_err(name)?;
        Ok(())
    }
}
