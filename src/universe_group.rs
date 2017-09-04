

use GameType;

pub struct UniverseGroup {
    id: u16
}

impl UniverseGroup {

    pub fn game_type(&self) -> Option<GameType> {
        None
    }
}

impl PartialEq for UniverseGroup {
    fn eq(&self, other: &UniverseGroup) -> bool {
        self.id == other.id
    }
}