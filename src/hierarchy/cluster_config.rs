use crate::network::{PacketReader, PacketWriter};
use crate::utils::check_name_or_err_32;
use crate::GameError;

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

    /// The name of the configured [`crate::hierarchy::Cluster`].
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
