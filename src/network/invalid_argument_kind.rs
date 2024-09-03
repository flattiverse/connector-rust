/// Specifies how much an argument is wrong.
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
pub enum InvalidArgumentKind {
    /// The argument was too small.
    TooSmall = 0x01,
    /// The argument was too large.
    TooLarge = 0x02,
    /// The arguments value doesn't match the name constraint.
    NameConstraint = 0x03,
    /// The arguments value doesn't match the chat message constraints
    ChatConstraint = 0x04,
    /// The specified entity has not been found.
    EntityNotFound = 0xFB,
    /// The specified name is already taken.
    NameInUse = 0xFC,
    /// The arguments value did contain a Not a Number value.
    ConstrainedNaN = 0xFD,
    /// The argument contains an infinite value.
    ConstrainedInfinity = 0xFE,

    /// It's not specified how the parameter is invalid.
    #[num_enum(catch_all)]
    Unknown(u8),
}
