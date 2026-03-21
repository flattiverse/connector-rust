/// Runtime state of a subsystem for the current server tick.
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
pub enum SubsystemStatus {
    /// The subsystem was off and therefore did not act.
    Off = 0x00,
    /// The subsystem was enabled and successfully performed its work.
    Worked = 0x01,
    /// The subsystem was enabled but failed, typically because resources were missing.
    Failed = 0x02,
    /// The subsystem is currently upgrading and therefore unavailble.
    Upgrading = 0x03,

    /// The subsystem status is unknown.
    #[num_enum(catch_all)]
    Unknown(u8),
}
