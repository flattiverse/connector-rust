use crate::galaxy_hierarchy::{
    Cluster, ControllableInfo, ControllableInfoId, Player, PlayerId, Team,
};
use crate::network::PacketReader;
use crate::unit::{UnitBase, UnitExt, UnitExtSealed, UnitKind};
use crate::utils::{Also, Atomic};
use crate::Vector;
use std::sync::{Arc, Weak};

/// Represents an explosion.
#[derive(Debug, Clone)]
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
            position: Vector::from_read(reader),
            second_phase: Atomic::from(false),
        }
        .also(|it| {
            it.base.mark_full_state_known();
        })
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

    /// Defines whether this explosion is in the damage phase or not.
    pub fn is_damage_phase(&self) -> bool {
        true
    }

    /// Defines whether this explosion is in the shockwave phase.
    pub fn is_shock_wave_phase(&self) -> bool {
        false
    }

    /// The damage this explosion inflicts.
    #[inline]
    pub fn damage(&self) -> f32 {
        self.damage
    }
}

impl AsRef<UnitBase> for Explosion {
    #[inline]
    fn as_ref(&self) -> &UnitBase {
        &self.base
    }
}

impl<'a> UnitExtSealed<'a> for &'a Explosion {
    type Parent = &'a UnitBase;

    #[inline]
    fn parent(self) -> Self::Parent {
        &self.base
    }

    fn update_movement(self, reader: &mut dyn PacketReader) {
        self.parent().update_movement(reader);

        self.second_phase.store(true);
    }
}

impl<'a> UnitExt<'a> for &'a Explosion {
    fn radius(self) -> f32 {
        self.size
    }

    fn position(self) -> Vector {
        self.position
    }

    #[inline]
    fn is_masking(self) -> bool {
        false
    }

    #[inline]
    fn is_solid(self) -> bool {
        false
    }

    #[inline]
    fn gravity(self) -> f32 {
        0.0
    }

    #[inline]
    fn kind(self) -> UnitKind {
        UnitKind::Explosion
    }

    #[inline]
    fn team(self) -> Weak<Team> {
        self.player
            .upgrade()
            .map_or_else(Weak::default, |p| p.team_weak())
    }
}
