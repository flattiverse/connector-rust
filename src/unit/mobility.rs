/// Describes how a unit moves on the map.
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
pub enum Mobility {
    /// The unit does not move.
    Still = 0x01,
    /// The unit moves with a predefined steady movement.
    Steady = 0x02,
    /// The unit can actively change its movement at runtime.
    Mobile = 0x04,
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl Mobility {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}
