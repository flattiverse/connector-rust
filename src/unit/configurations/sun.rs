use crate::network::{PacketReader, PacketWriter};
use crate::unit::configurations::CelestialBodyConfiguration;
use crate::unit::sub_components::SunSection;
use crate::{GameError, GameErrorKind};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default)]
pub struct SunConfiguration {
    pub(crate) base: CelestialBodyConfiguration,
    pub(crate) sections: Vec<SunSection>,
}

impl SunConfiguration {
    pub(crate) fn read(&mut self, reader: &mut dyn PacketReader) {
        self.base.read(reader);

        let sections = usize::from(reader.read_byte());
        self.sections = (0..sections)
            .map(|_| {
                let mut this = SunSection::default();
                this.read(reader);
                this
            })
            .collect();
    }

    pub(crate) fn write(&self, writer: &mut dyn PacketWriter) {
        self.base.write(writer);

        writer.write_byte(self.sections.len() as _);
        self.sections.iter().for_each(|s| s.write(writer));
    }

    #[inline]
    pub fn sections(&self) -> &[SunSection] {
        &self.sections
    }

    #[inline]
    pub fn sections_mut(&mut self) -> &mut [SunSection] {
        &mut self.sections
    }

    /// Returns the index of the new [`SunSection`] on success.
    pub fn add_section(&mut self) -> Result<usize, GameError> {
        if self.sections.len() >= 16 {
            Err(GameErrorKind::CannotAddAlreayFull.into())
        } else {
            let index = self.sections.len();
            self.sections.push(SunSection::default());
            Ok(index)
        }
    }

    /// Tries to remove the [`SunSection`] from this [`SunConfiguration`]. If successful, all
    /// [`SunSections`] after the given index will be moved forward. See [`Vec::remove`].
    #[inline]
    pub fn remove_section(&mut self, index: usize) -> Option<SunSection> {
        if index >= self.sections.len() {
            None
        } else {
            Some(self.sections.remove(index))
        }
    }
}

impl Deref for SunConfiguration {
    type Target = CelestialBodyConfiguration;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for SunConfiguration {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
