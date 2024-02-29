#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, num_enum::TryFromPrimitive, num_enum::IntoPrimitive,
)]
pub enum DestructionReason {
    Shutdown,
    SelfDestruction,
    Collision,
}
