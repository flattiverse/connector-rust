#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, num_enum::FromPrimitive, num_enum::IntoPrimitive)]
pub enum GameType {
    #[default]
    Mission,
    STF,
    Domination,
}
