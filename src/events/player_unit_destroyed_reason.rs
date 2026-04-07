/// Describes why a public controllable-registration entry died.
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
    /// Destroyed by global server rules, for example disconnect cleanup or maintenance transitions.
    ByRules = 0x00,
    /// Destroyed because the owner explicitly called <c>Suicide()</c>.
    Suicided = 0x10,
    /// Destroyed because the containing cluster was removed.
    ByClusterRemoval = 0x01,
    /// Destroyed after leaving the activated map area and getting lost in deep space.
    LostInDeepSpace = 0x02,
    /// Destroyed by collision with a non-player unit.
    CollidedWithNeutralUnit = 0x20,
    /// Destroyed by collision with an enemy player-controlled unit.
    CollidedWithEnemyPlayerUnit = 0x28,
    /// Destroyed by collision with a friendly player-controlled unit.
    CollidedWithFriendlyPlayerUnit = 0x29,
    /// Destroyed by hostile player-originated weapon damage.
    ShotByEnemyPlayerUnit = 0x38,
    /// Destroyed by friendly-fire weapon damage.
    ShotByFriendlyPlayerUnit = 0x39,
    /// Temporarily taken offline because one subsystem is being rebuilt.
    Rebuilding = 0x40,

    #[num_enum(catch_all)]
    Unknown(u8),
}

impl PlayerUnitDestroyedReason {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }
}
