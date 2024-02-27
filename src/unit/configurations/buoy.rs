use crate::network::{PacketReader, PacketWriter};
use crate::unit::configurations::{CelestialBodyConfiguration, Configuration};
use crate::unit::UnitKind;
use crate::utils::check_name_or_err_64;
use crate::{GameError, Vector};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default)]
pub struct BuoyConfiguration {
    pub(crate) base: CelestialBodyConfiguration,
    message: String,
    beacons: Vec<Vector>,
}

impl BuoyConfiguration {
    /// The new message for the [`crate::unit::Buoy`].
    pub fn set_message(&mut self, message: impl Into<String>) -> Result<(), GameError> {
        let message = message.into();
        self.message = check_name_or_err_64(message)?;
        Ok(())
    }
}

impl Configuration for BuoyConfiguration {
    #[inline]
    fn read(&mut self, reader: &mut dyn PacketReader) {
        self.base.read(reader);

        self.message = reader.read_string();
        self.beacons = (0..reader.read_byte())
            .map(|_| Vector::default().with_read(reader))
            .collect();
    }

    #[inline]
    fn write(&self, writer: &mut dyn PacketWriter) {
        self.base.write(writer);

        writer.write_string(&self.name);
        writer.write_byte(self.beacons.len() as u8);
        self.beacons.iter().for_each(|b| b.write(writer));
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Buoy
    }
}

impl Deref for BuoyConfiguration {
    type Target = CelestialBodyConfiguration;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for BuoyConfiguration {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
