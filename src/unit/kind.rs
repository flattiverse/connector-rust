/// Specifies of which kind a unit is.
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
pub enum UnitKind {
    /// Represents a sun.
    Sun = 0x00,
    /// Represents a black hole.
    BlackHole = 0x01,
    /// Represents a planet.
    Planet = 0x08,
    /// Represents a moon.
    Moon = 0x09,
    /// Represents a meteoroid.
    Meteoroid = 0x0A,
    /// Represents a classical player ship.
    ClassicShipPlayerUnit = 0xF0,
    /// Represents a new style player ship.
    NewShipPlayerUnit = 0xF1,
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl UnitKind {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}
