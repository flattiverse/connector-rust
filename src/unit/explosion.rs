use crate::galaxy_hierarchy::{
    Cluster, ControllableInfo, ControllableInfoId, Player, PlayerId, Team,
};
use crate::network::PacketReader;
use crate::runtime::Atomic;
use crate::unit::UnitBase;
use crate::Vector;
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub struct Explosion {
    base: UnitBase,
    player: Weak<Player>,
    controllable_info: Weak<ControllableInfo>,
    position: Vector,
    size: f32,
    damage: f32,
    second_phase: Atomic<bool>,
}

impl Explosion {
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
            .and_then(|id| galaxy.get_player_opt(id));

        let controllable_info = player
            .as_ref()
            .and_then(|p| p.get_controllable_info_opt(controllable_id));

        Self {
            base: UnitBase::new(cluster, name),
            player: player.as_ref().map(Arc::downgrade).unwrap_or_default(),
            controllable_info: controllable_info
                .as_ref()
                .map(Arc::downgrade)
                .unwrap_or_default(),
            size: reader.read_f32(),
            damage: reader.read_f32(),
            position: Vector::default().with_read(reader),
            second_phase: Atomic::from(false),
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
        self.position
    }

    /// The radius of the unit.
    #[inline]
    pub fn radius(&self) -> f32 {
        self.size
    }

    /// If true, other unis can hide behind this unit.
    #[inline]
    pub fn is_masking(&self) -> bool {
        false
    }

    /// If true, a crash with this unit is lethal.
    #[inline]
    pub fn is_solid(&self) -> bool {
        false
    }

    /// The gravity of this unit. This is how much this unit pulls others towards it.
    pub fn gravity(&self) -> f32 {
        if self.second_phase.load() {
            -0.5
        } else {
            0.0
        }
    }

    /// The team of the unit.
    pub fn team(&self) -> Weak<Team> {
        self.player
            .upgrade()
            .map(|p| Arc::downgrade(&p.team()))
            .unwrap_or_default()
    }

    /// Defines whether this explosion is in the damage phase or not.
    pub fn is_damage_phase(&self) -> bool {
        !self.second_phase.load()
    }

    /// Defines whether this explosion is in the shockwave phase.
    pub fn is_shock_wave_phase(&self) -> bool {
        self.second_phase.load()
    }

    /// The damage this explosion inflicts.
    #[inline]
    pub fn damage(&self) -> f32 {
        self.damage
    }

    pub(crate) fn update_movement(&self, reader: &mut dyn PacketReader) {
        self.second_phase.store(true);
        let _ = reader;
    }
}
