#[repr(u8)]
#[derive(
    Debug, Copy, Clone, Default, PartialEq, Eq, num_enum::FromPrimitive, num_enum::IntoPrimitive,
)]
pub enum Mobility {
    #[default]
    Still = 0,
    Steady,
    Mobile,
}
