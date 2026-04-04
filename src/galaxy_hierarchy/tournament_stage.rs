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
pub enum TournamentStage {
    /// Tournament exists but has not been commenced yet.
    Preparation = 0x00,
    /// Tournament has been commenced and is in its pre-run stage.
    Commencing = 0x01,
    /// Tournament is currently running.
    Running = 0x02,

    #[num_enum(catch_all)]
    Unknown(u8),
}

impl TournamentStage {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}
