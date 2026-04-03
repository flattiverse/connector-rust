use crate::network::{PacketReader, PacketWriter};
use crate::utils::{Readable, Writable};
use num_enum::FromPrimitive;

/// Direction used for railgun firing.
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
pub enum RailgunDirection {
    /// No direction was processed in the current tick.
    None = 0x00,
    /// Fire along the current ship angle.
    Front = 0x01,
    /// Fire opposite to the current ship angle.
    Back = 0x02,

    #[num_enum(catch_all)]
    Unknown(u8),
}

impl Default for RailgunDirection {
    #[inline]
    fn default() -> Self {
        Self::None
    }
}

impl Readable for RailgunDirection {
    #[inline]
    fn read(reader: &mut dyn PacketReader) -> Self {
        RailgunDirection::from_primitive(reader.read_byte())
    }
}

impl Writable for RailgunDirection {
    #[inline]
    fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_byte(u8::from(*self));
    }
}
