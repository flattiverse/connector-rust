/// Specifies why a PlayerUnit has been destroyed.
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
pub enum PlayerUnitDestroyedReason {
    /// PlayerUnit got destroyed due to server rules like when the player disconnects or the galaxy,
    /// switched to maintenance mode.
    ByRules = 0x00,
    /// The player called kill().
    Suicided = 0x10,
    /// The PlayerUnit collided with a neutral unit.
    CollidedWithNeutralUnit = 0x20,
    /// The PlayerUnit collided with an enemy PlayerUnit.
    CollidedWithEnemyPlayerUnit = 0x28,
    /// the PlayerUnit collided with a friendly PlayerUnit.
    CollidedWithFriendlyPlayerUnit = 0x29,
    /// The PlayerUnit has been shot by an enemy PlayerUnit.
    ShotByEnemyPlayerUnit = 0x38,
    /// The PlayerUnit has been shot by a friendly PlayerUnit.
    ShotByFriendlyPlayerUnit = 0x39,
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl PlayerUnitDestroyedReason {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}
