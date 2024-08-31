
/// The status of an account - does it need to opt in, etc?
#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    Default,
    PartialEq,
    Eq,
    num_enum::FromPrimitive,
    num_enum::IntoPrimitive,
    strum::EnumIter,
    strum::AsRefStr,
)]
pub enum AccountStatus {
    /// The account needs to confirm its email address.
    OptIn = 0x00,
    /// The account needs to reconfirm its email address.
    ReOptIn = 0x01,
    /// The account is opted in and can use the game.
    User = 0x10,
    /// The account is banned.
    Banned = 0x80,
    /// The account is deleted.
    Deleted = 0xF0,
    /// The account is in an unknown state
    #[default]
    Unknown = 0xFF
}

impl AccountStatus {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}
