/// Series format of a Flattiverse tournament.
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
pub enum TournamentMode {
    /// One single decisive match.
    Solo = 0x00,
    /// First team to win two matches.
    BestOf3 = 0x01,
    /// First team to win three matches.
    BestOf5 = 0x02,
    /// First team to win four matches.
    BestOf7 = 0x03,
    /// First team to win five matches.
    BestOf9 = 0x04,
    /// First team to win six matches.
    BestOf11 = 0x05,
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl TournamentMode {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}
