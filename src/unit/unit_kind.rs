#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    Default,
    PartialEq,
    Eq,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::EnumIter,
    strum::AsRefStr,
)]
pub enum UnitKind {
    #[default]
    Sun = 0x00,
    BlackHole = 0x01,
    Planet = 0x04,
    Moon = 0x05,
    Meteoroid = 0x06,
    Buoy = 0x10,
    Ship = 0xF0,
}

impl UnitKind {
    #[inline]
    pub fn iter() -> impl Iterator<Item = UnitKind> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}
