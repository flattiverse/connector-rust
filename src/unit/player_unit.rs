use crate::galaxy_hierarchy::{ControllableInfo, ControllableInfoId, Galaxy, Player, PlayerId};
use crate::network::PacketReader;
use crate::runtime::Atomic;
use crate::unit::Mobility;
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

    /// Represents the player which controls the PlayerUnit.
    #[inline]
    pub fn player(&self) -> Arc<Player> {
        self.player.upgrade().unwrap()
    }

    /// Represents the ControllableInfo of this PlayerUnit.
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

    #[inline]
    pub fn angle(&self) -> f32 {
        self.movement().angle()
    }

    #[inline]
    pub fn mobility(&self) -> Mobility {
        Mobility::Mobile
    }

    pub(crate) fn update_movement(&self, reader: &mut dyn PacketReader) {
        self.position.read(reader);
        self.movement.read(reader);
    }
}
