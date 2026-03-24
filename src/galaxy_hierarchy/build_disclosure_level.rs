/// Highest disclosed build-assistance level for one aspect.
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
pub enum BuildDisclosureLevel {
    /// No search engine or LLM help was used.
    None = 0,
    /// Search engines or documentation were used without LLMs.
    SearchOnly = 1,
    /// Free-tier LLMs were used.
    FreeLlm = 2,
    /// Paid chat-grade LLMs were used.
    PaidLlm = 3,
    /// Editor-integrated LLM tooling was used.
    IntegratedLlm = 4,
    /// Agentic coding tools such as Codex or Claude Code were used.
    AgenticTool = 5,
    #[num_enum(catch_all)]
    Unknown(u8),
}

impl BuildDisclosureLevel {
    pub(crate) fn validated(self) -> Option<Self> {
        if matches!(self, BuildDisclosureLevel::Unknown(_)) {
            None
        } else {
            Some(self)
        }
    }
}
