/// Self-disclosed runtime automation level for one aspect.
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
pub enum RuntimeDisclosureLevel {
    /// This capability is not implemented.
    Unsupported = 0,
    /// A human issues the concrete action manually.
    Manual = 1,
    /// A human acts directly and software assists.
    Assisted = 2,
    /// Software executes a concrete user-selected target automatically.
    Automated = 3,
    /// Software acts autonomously within a broader mission or policy.
    Autonomous = 4,
    /// An AI system controls the aspect.
    AiControlled = 5,
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl RuntimeDisclosureLevel {
    pub(crate) fn validated(self) -> Option<Self> {
        if matches!(self, RuntimeDisclosureLevel::Unknown(_)) {
            None
        } else {
            Some(self)
        }
    }
}
