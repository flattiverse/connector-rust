use crate::network::{PacketReader, PacketWriter};
use crate::unit::configurations::{CelestialBodyConfiguration, Configuration};
use crate::unit::UnitKind;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default)]
pub struct BuoyConfiguration {
    pub(crate) base: CelestialBodyConfiguration,
}

impl Configuration for BuoyConfiguration {
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
