use crate::network::{PacketReader, PacketWriter};
use crate::utils::{Readable, Writable};
use num_enum::FromPrimitive;

/// Quality grade of a crystal.
#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    num_enum::FromPrimitive,
    num_enum::IntoPrimitive,
    strum::EnumIter,
    strum::AsRefStr,
)]
pub enum CrystalGrade {
    /// Weak crystal with side effects.
    LowGrade = 0x00,
    /// Common crystal with small side effects.
    Regular = 0x01,
    /// Pure crystal without side effects.
    Pure = 0x02,
    /// High-grade crystal.
    Mastery = 0x03,
    /// Exceptional crystal with adjacent positive effects.
    Divine = 0x04,

    #[num_enum(catch_all)]
    Unknown(u8),
}

impl CrystalGrade {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}

impl Readable for CrystalGrade {
    #[inline]
    fn read(reader: &mut dyn PacketReader) -> Self {
        Self::from_primitive(reader.read_byte())
    }
}

impl Writable for CrystalGrade {
    #[inline]
    fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_byte(u8::from(*self));
    }
}
