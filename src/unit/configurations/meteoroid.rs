use crate::network::{PacketReader, PacketWriter};
use crate::unit::configurations::{Configuration, HarvestableConfiguration};
use crate::unit::UnitKind;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default)]
pub struct MeteoroidConfiguration {
    pub(crate) base: HarvestableConfiguration,
}

impl Configuration for MeteoroidConfiguration {
    #[inline]
    fn unit_name(&self) -> &str {
        &self.base.name
    }

    #[inline]
    fn read(&mut self, reader: &mut dyn PacketReader) {
        self.base.read(reader);
    }

    #[inline]
    fn write(&self, writer: &mut dyn PacketWriter) {
        self.base.write(writer);
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Meteoroid
    }
}

impl Deref for MeteoroidConfiguration {
    type Target = HarvestableConfiguration;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for MeteoroidConfiguration {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
