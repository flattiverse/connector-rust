use crate::network::{PacketReader, PacketWriter};
use crate::unit::configurations::{Configuration, HarvestableConfiguration};
use crate::unit::UnitKind;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default)]
pub struct PlanetConfiguration {
    pub(crate) base: HarvestableConfiguration,
}

impl Configuration for PlanetConfiguration {
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
        UnitKind::Planet
    }
}

impl Deref for PlanetConfiguration {
    type Target = HarvestableConfiguration;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for PlanetConfiguration {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
