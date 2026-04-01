use crate::network::PacketReader;
use crate::utils::Readable;
use num_enum::FromPrimitive;

/// Defines how a switch affects linked gates.
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
pub enum SwitchMode {
    /// Inverts the current gate state.
    Toggle = 0x00,
    /// Opens linked gates.
    Open = 0x01,
    /// Closes linked gates.
    Close = 0x02,

    #[num_enum(catch_all)]
    Unknown(u8),
}

impl Readable for SwitchMode {
    #[inline]
    fn read(reader: &mut dyn PacketReader) -> Self {
        Self::from_primitive(reader.read_byte())
    }
}
