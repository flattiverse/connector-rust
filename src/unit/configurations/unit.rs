use crate::network::{PacketReader, PacketWriter};
use crate::utils::check_name_or_err_32;
use crate::GameError;

#[derive(Debug, Clone)]
pub struct UnitConfiguration {
    pub name: String,
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
        let name = name.into();
        self.name = check_name_or_err_32(name)?;
        Ok(())
    }

    #[inline]
    pub fn name_valid(&self) -> bool {
        check_name_or_err_32(&self.name).is_ok()
    }
}
