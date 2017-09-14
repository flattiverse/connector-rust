
#[repr(u8)]
#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum GameType {
    Mission = 0,
    ShootTheFlag = 1,
    Domination = 2
}

impl GameType {
    pub fn from_id(id: u8) -> Option<GameType> {
        match id {
            0 => Some(GameType::Mission),
            1 => Some(GameType::ShootTheFlag),
            2 => Some(GameType::Domination),
            _ => None
        }
    }
}