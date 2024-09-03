use crate::galaxy_hierarchy::{ControllableInfo, ControllableInfoId, Galaxy, Player, PlayerId};
use crate::network::PacketReader;
use crate::runtime::Atomic;
use crate::Vector;
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub struct PlayerUnit {
    player: Weak<Player>,
    controllable_info: Weak<ControllableInfo>,
    position: Atomic<Vector>,
    movement: Atomic<Vector>,
}

impl PlayerUnit {
    pub(crate) fn read(galaxy: &Galaxy, reader: &mut dyn PacketReader) -> Self {
        let player_id = PlayerId(reader.read_byte());
        let controllable_info_id = ControllableInfoId(reader.read_byte());

        let player = galaxy.get_player(player_id);
        let controllable_info = player.get_controllable_info(controllable_info_id);

        Self {
            player: Arc::downgrade(&player),
            controllable_info: Arc::downgrade(&controllable_info),
            position: Atomic::from_reader(reader),
            movement: Atomic::from_reader(reader),
        }
    }

    #[inline]
    pub fn player(&self) -> Arc<Player> {
        self.player.upgrade().unwrap()
    }

    #[inline]
    pub fn controllable_info(&self) -> Arc<ControllableInfo> {
        self.controllable_info.upgrade().unwrap()
    }

    #[inline]
    pub fn position(&self) -> Vector {
        self.position.load()
    }

    #[inline]
    pub fn movement(&self) -> Vector {
        self.movement.load()
    }
}
