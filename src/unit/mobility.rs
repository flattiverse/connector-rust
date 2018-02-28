
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Mobility {
    Still = 0,
    Steady = 1,
    Mobile = 2
}

impl Mobility {
    pub fn from_id(id: u8) -> Option<Mobility> {
        match id {
            0 => Some(Mobility::Still),
            1 => Some(Mobility::Steady),
            2 => Some(Mobility::Mobile),
            _ => None,
        }
    }
}