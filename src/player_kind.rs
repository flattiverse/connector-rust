#[derive(Debug, Copy, Clone, PartialEq, Eq, num_enum::FromPrimitive, num_enum::IntoPrimitive)]
pub enum PlayerKind {
    Player = 0x01,
    Spectator = 0x02,
    Admin = 0x04,
}
