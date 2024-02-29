#[repr(u8)]
#[derive(
    Debug, Copy, Clone, Default, PartialEq, Eq, num_enum::FromPrimitive, num_enum::IntoPrimitive,
)]
pub enum DestructionReason {
    Shutdown,
    Suicide,
    Collision,
}
