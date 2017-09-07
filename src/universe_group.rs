
use std::sync::Arc;
use std::sync::RwLock;

use Team;
use GameType;

pub struct UniverseGroup {
    id: u16
}

impl UniverseGroup {

    pub fn game_type(&self) -> Option<GameType> {
        None
    }

    pub fn team(&self, index: u8) -> &Option<Arc<RwLock<Team>>> {
        None
    }
}

impl PartialEq for UniverseGroup {
    fn eq(&self, other: &UniverseGroup) -> bool {
        self.id == other.id
    }
}