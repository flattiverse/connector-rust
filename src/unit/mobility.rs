/// Specifies the mobility of a unit.
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
    /// The unit doesn't move at all.
    Still = 0x01,
    /// The unit has a steady movement.
    Steady = 0x02,
    /// the unit is mobile.
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
