use crate::galaxy_hierarchy::{
    Cluster, ControllableInfo, ControllableInfoId, Player, PlayerId, Team,
};
use crate::network::PacketReader;
use crate::unit::{Mobility, UnitBase};
use crate::utils::Atomic;
use crate::Vector;
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub struct Shot {
    base: UnitBase,
    player: Weak<Player>,
    controllable_info: Weak<ControllableInfo>,
    position: Atomic<Vector>,
    movement: Atomic<Vector>,
    ticks: Atomic<u16>,
    load: f32,
    damage: f32,
}

impl Shot {
    pub(crate) fn read(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Self {
        let galaxy = cluster.upgrade().map(|c| c.galaxy()).unwrap();

        let player_id = PlayerId(reader.read_byte());
        let controllable_id = ControllableInfoId(reader.read_byte());

        let player = Some(player_id)
            .filter(|id| id.0 < 192)
            .map(|id| galaxy.get_player(id));

        let controllable_info = player
            .as_ref()
            .map(|p| p.get_controllable_info(controllable_id));

        Self {
            base: UnitBase::new(cluster, name),
            player: player.as_ref().map(Arc::downgrade).unwrap_or_default(),
            controllable_info: controllable_info
                .as_ref()
                .map(Arc::downgrade)
                .unwrap_or_default(),
            ticks: Atomic::from(reader.read_uint16()),
            load: reader.read_f32(),
            damage: reader.read_f32(),
            position: Atomic::from_reader(reader),
            movement: Atomic::from_reader(reader),
        }
    }

    #[inline]
    pub fn base(&self) -> &UnitBase {
        &self.base
    }

    /// Represents the player which invoked the shot or null, if the shot hasn't been invoked by a
    /// player.
    #[inline]
    pub fn player(&self) -> &Weak<Player> {
        &self.player
    }

    /// Represents the ControllableInfo which invoked the shot or null, if the shot hasn't been
    /// invoked by a player.
    #[inline]
    pub fn controllable_info(&self) -> &Weak<ControllableInfo> {
        &self.controllable_info
    }

    /// The position of the unit.
    #[inline]
    pub fn position(&self) -> Vector {
        self.position.load()
    }

    /// The movement of the unit.
    #[inline]
    pub fn movement(&self) -> Vector {
        self.movement.load()
    }

    /// The direction the unit is looking into.
    #[inline]
    pub fn angle(&self) -> f32 {
        self.movement().angle()
    }

    /// The radius of the unit.
    #[inline]
    pub fn radius(&self) -> f32 {
        1.0
    }

    /// The mobility of this unit.
    #[inline]
    pub fn mobility(&self) -> Mobility {
        Mobility::Mobile
    }

    /// The team of the unit.
    pub fn team(&self) -> Weak<Team> {
        self.player
            .upgrade()
            .map(|p| Arc::downgrade(&p.team()))
            .unwrap_or_default()
    }

    /// The countdown of when the shot explodes.
    #[inline]
    pub fn ticks(&self) -> u16 {
        self.ticks.load()
    }

    #[inline]
    pub fn load(&self) -> f32 {
        self.load
    }

    #[inline]
    pub fn damage(&self) -> f32 {
        self.damage
    }

    pub(crate) fn update_movement(&self, reader: &mut dyn PacketReader) {
        self.ticks.store(reader.read_uint16());
        self.position.read(reader);
        self.movement.read(reader);
    }
}
