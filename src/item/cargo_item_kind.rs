
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum CargoItemKind {
    Nebula = 0,
    Crystal = 1,
    MissionTarget = 2,
}

impl CargoItemKind {
    pub fn from_id(id: u8) -> Option<CargoItemKind> {
        match id {
            0 => Some(CargoItemKind::Nebula),
            1 => Some(CargoItemKind::Crystal),
            2 => Some(CargoItemKind::MissionTarget),
            _ => None
        }
    }
}