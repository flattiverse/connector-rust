/// The game mode of the galaxy.
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
pub enum GameMode {
    /// In this game mode players try to complete mission objectives.
    Mission = 0x00,
    /// In this game mode players try to shoot the enemy flag.
    ShootTheFlag = 0x01,
    /// In this game mode players fight over control points.
    Domination = 0x02,
    /// In this game mode players try to get the fastest time on a track.
    Race = 0x03,
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl GameMode {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}
