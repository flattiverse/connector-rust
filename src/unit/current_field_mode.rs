use crate::network::{PacketReader, PacketWriter};
use crate::utils::{Readable, Writable};
use num_enum::FromPrimitive;

/// Determines how a current field induces movement.
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
pub enum CurrentFieldMode {
    /// Applies a fixed world-space movement vector.
    Directional = 0x00,
    /// Applies radial and tangential movement relative to the field center.
    Relative = 0x01,

    #[num_enum(catch_all)]
    Unknown(u8),
}

impl Readable for CurrentFieldMode {
    #[inline]
    fn read(reader: &mut dyn PacketReader) -> Self {
        CurrentFieldMode::from_primitive(reader.read_byte())
    }
}

impl Writable for CurrentFieldMode {
    #[inline]
    fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_byte(u8::from(*self))
    }
}
