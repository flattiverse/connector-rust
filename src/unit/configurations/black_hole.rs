use crate::network::{PacketReader, PacketWriter};
use crate::unit::configurations::{CelestialBodyConfiguration, Configuration};
use crate::unit::sub_components::BlackHoleSection;
use crate::unit::UnitKind;
use crate::{GameError, GameErrorKind};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default)]
pub struct BlackHoleConfiguration {
    pub(crate) base: CelestialBodyConfiguration,
    pub sections: Vec<BlackHoleSection>,
}

impl BlackHoleConfiguration {
    /// Returns the index of the new [`BlackHoleSection`] on success.
    pub fn add_section(&mut self) -> Result<usize, GameError> {
        if self.sections.len() >= 16 {
            Err(GameErrorKind::CannotAddAlreayFull.into())
        } else {
            let index = self.sections.len();
            self.sections.push(BlackHoleSection::default());
            Ok(index)
        }
    }

    /// Tries to remove the [`BlackHoleSection`] from this [`BlackHoleConfiguration`]. If successful, all
    /// [`BlackHoleSections`] after the given index will be moved forward. See [`Vec::remove`].
    #[inline]
    pub fn remove_section(&mut self, index: usize) -> Option<BlackHoleSection> {
        if index >= self.sections.len() {
            None
        } else {
            Some(self.sections.remove(index))
        }
    }
}

impl Configuration for BlackHoleConfiguration {
    #[inline]
    fn read(&mut self, reader: &mut dyn PacketReader) {
        self.base.read(reader);

        let sections = usize::from(reader.read_byte());
        self.sections = (0..sections)
            .map(|_| {
                let mut this = BlackHoleSection::default();
                this.read(reader);
                this
            })
            .collect();
    }

    #[inline]
    fn write(&self, writer: &mut dyn PacketWriter) {
        self.base.write(writer);

        writer.write_byte(self.sections.len() as _);
        self.sections.iter().for_each(|s| s.write(writer));
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::BlackHole
    }
}

impl Deref for BlackHoleConfiguration {
    type Target = CelestialBodyConfiguration;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for BlackHoleConfiguration {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
