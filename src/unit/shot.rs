use crate::galaxy_hierarchy::{
    Cluster, ControllableInfo, ControllableInfoId, Player, PlayerId, Team,
};
use crate::network::PacketReader;
use crate::unit::{Mobility, UnitBase, UnitExt, UnitExtSealed, UnitKind};
use crate::utils::Atomic;
use crate::Vector;
use std::sync::{Arc, Weak};

/// Represents a shot.
#[derive(Debug, Clone)]
pub struct Shot {
    base: UnitBase,
    player: Weak<Player>,
    controllable_info: Weak<ControllableInfo>,
    position: Atomic<Vector>,
    movement: Atomic<Vector>,
    ticks: Atomic<u16>,
    load: Atomic<f32>,
    damage: Atomic<f32>,
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
            load: Atomic::default(),
            damage: Atomic::default(),
            position: Atomic::from_reader(reader),
            movement: Atomic::from_reader(reader),
        }
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

    /// The countdown of when the shot explodes.
    #[inline]
    pub fn ticks(&self) -> u16 {
        self.ticks.load()
    }

    #[inline]
    pub fn load(&self) -> f32 {
        self.load.load()
    }

    #[inline]
    pub fn damage(&self) -> f32 {
        self.damage.load()
    }
}

impl AsRef<UnitBase> for Shot {
    #[inline]
    fn as_ref(&self) -> &UnitBase {
        &self.base
    }
}

impl<'a> UnitExtSealed<'a> for &'a Shot {
    type Parent = &'a UnitBase;

    #[inline]
    fn parent(self) -> Self::Parent {
        &self.base
    }

    fn update_movement(self, reader: &mut dyn PacketReader) {
        self.parent().update_movement(reader);

        self.ticks.store(reader.read_uint16());
        self.position.read(reader);
        self.movement.read(reader);
    }

    fn update_state(self, reader: &mut dyn PacketReader) {
        self.parent().update_state(reader);

        self.load.read(reader);
        self.damage.read(reader);
    }
}

impl<'a> UnitExt<'a> for &'a Shot {
    #[inline]
    fn radius(self) -> f32 {
        1.0
    }

    #[inline]
    fn position(self) -> Vector {
        self.position.load()
    }

    #[inline]
    fn movement(self) -> Vector {
        self.movement.load()
    }

    #[inline]
    fn angle(self) -> f32 {
        self.movement().angle()
    }

    #[inline]
    fn is_masking(self) -> bool {
        false
    }

    #[inline]
    fn mobility(self) -> Mobility {
        Mobility::Mobile
    }

    #[inline]
    fn kind(self) -> UnitKind {
        UnitKind::Shot
    }

    #[inline]
    fn team(self) -> Weak<Team> {
        self.player
            .upgrade()
            .map_or_else(Weak::default, |p| p.team_weak())
    }
}
