use crate::galaxy_hierarchy::{
    ControllableInfo, ControllableInfoId, Galaxy, Player, PlayerId, Team,
};
use crate::network::PacketReader;
use crate::unit::{Mobility, UnitBase, UnitExt, UnitExtSealed};
use crate::utils::Atomic;
use crate::Vector;
use std::sync::{Arc, Weak};

/// Represents a player unit.
#[derive(Debug, Clone)]
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

    pub(crate) fn update_movement(&self, reader: &mut dyn PacketReader) {
        self.position.read(reader);
        self.movement.read(reader);
    }
}

impl<'a> UnitExtSealed<'a> for (&'a UnitBase, &'a PlayerUnit)
where
    Self: 'a,
{
    type Parent = &'a UnitBase;

    #[inline]
    fn parent(self) -> Self::Parent {
        self.0
    }
}

impl<'b> UnitExt<'b> for (&'b UnitBase, &'b PlayerUnit) {
    #[inline]
    fn position(self) -> Vector {
        self.1.position.load()
    }

    #[inline]
    fn movement(self) -> Vector {
        self.1.movement.load()
    }

    #[inline]
    fn angle(self) -> f32 {
        self.1.movement.load().angle()
    }

    #[inline]
    fn mobility(self) -> Mobility {
        Mobility::Mobile
    }

    #[inline]
    fn team(self) -> Weak<Team> {
        self.1.player().team_weak()
    }
}
