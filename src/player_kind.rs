#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlayerKind {
    Player = 0x01,
    Spectator = 0x02,
    Admin = 0x04,
}
