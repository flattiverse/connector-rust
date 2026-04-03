use crate::galaxy_hierarchy::{
    Cluster, ControllableInfo, ControllableInfoId, Player, PlayerId, Team,
};
use crate::network::PacketReader;
use crate::unit::{AbstractUnit, Unit, UnitCastTable, UnitHierarchy, UnitInternal, UnitKind};
use crate::utils::{Also, Atomic};
use crate::{GameError, Vector};
use std::sync::{Arc, Weak};

pub(crate) trait ExplosionInternal {
    fn parent(&self) -> &dyn Explosion;
}

/// Visible explosion unit created by a projectile or another gameplay effect.
/// The current connector model exposes explosions as immediate damage-phase objects without a
/// separate shockwave phase.
#[allow(private_bounds)]
pub trait Explosion: ExplosionInternal + Unit {
    /// Player that caused the explosion, or yields `None` if no player-owned source is known.
    #[inline]
    fn player(&self) -> &Weak<Player> {
        ExplosionInternal::parent(self).player()
    }

    /// Controllable entry that caused the explosion, or yields `None` if no player-owned source is
    /// known.
    #[inline]
    fn controllable_info(&self) -> &Weak<ControllableInfo> {
        ExplosionInternal::parent(self).controllable_info()
    }

    /// Whether this explosion is currently in its damage phase.
    /// In the current connector model this is always `true`.
    #[inline]
    fn is_damage_phase(&self) -> bool {
        ExplosionInternal::parent(self).is_damage_phase()
    }

    /// Whether this explosion is currently in its shockwave phase.
    /// In the current connector model this is always `false`.
    #[inline]
    fn is_shock_wave_phase(&self) -> bool {
        ExplosionInternal::parent(self).is_shock_wave_phase()
    }

    /// The damage this explosion inflicts.
    #[inline]
    fn damage(&self) -> f32 {
        ExplosionInternal::parent(self).damage()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AbstractExplosion {
    parent: AbstractUnit,
    player: Weak<Player>,
    controllable_info: Weak<ControllableInfo>,
    position: Vector,
    size: f32,
    damage: f32,
    second_phase: Atomic<bool>,
}

impl AbstractExplosion {
    #[inline]
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Self::new_inner(cluster, name, reader).map(Arc::new)
    }

    pub(crate) fn new_inner(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Self, GameError> {
        let galaxy = cluster.upgrade().map(|c| c.galaxy()).unwrap();

        let player_id = PlayerId(reader.read_byte());
        let controllable_id = ControllableInfoId(reader.read_byte());

        let player = Some(player_id)
            .filter(|id| id.0 < 192)
            .and_then(|id| galaxy.get_player_opt(id));

        let controllable_info = player
            .as_ref()
            .and_then(|p| p.get_controllable_info_opt(controllable_id));

        Ok(Self {
            parent: AbstractUnit::new(cluster, name),
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
            it.mark_full_state_known();
        }))
    }
}

impl UnitInternal for AbstractExplosion {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    #[inline]
    fn update_movement(&self, reader: &mut dyn PacketReader) {
        self.parent.update_movement(reader);

        self.second_phase.store(true);
    }
}

impl UnitCastTable for AbstractExplosion {
    cast_fn!(explosion_cast_fn, AbstractExplosion, dyn Explosion);
}

impl UnitHierarchy for AbstractExplosion {
    #[inline]
    fn as_explosion(&self) -> Option<&dyn Explosion> {
        Some(self)
    }
}

impl Unit for AbstractExplosion {
    #[inline]
    fn radius(&self) -> f32 {
        self.size
    }

    #[inline]
    fn position(&self) -> Vector {
        self.position
    }

    #[inline]
    fn is_masking(&self) -> bool {
        false
    }

    #[inline]
    fn is_solid(&self) -> bool {
        false
    }

    #[inline]
    fn gravity(&self) -> f32 {
        0.0
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Explosion
    }

    #[inline]
    fn team(&self) -> Weak<Team> {
        self.player
            .upgrade()
            .map_or_else(Weak::default, |p| p.team_weak())
    }
}

#[forbid(clippy::missing_trait_methods)]
impl ExplosionInternal for AbstractExplosion {
    #[inline]
    fn parent(&self) -> &dyn Explosion {
        unreachable!()
    }
}

#[forbid(clippy::missing_trait_methods)]
impl Explosion for AbstractExplosion {
    #[inline]
    fn player(&self) -> &Weak<Player> {
        &self.player
    }

    #[inline]
    fn controllable_info(&self) -> &Weak<ControllableInfo> {
        &self.controllable_info
    }

    #[inline]
    fn is_damage_phase(&self) -> bool {
        true
    }

    #[inline]
    fn is_shock_wave_phase(&self) -> bool {
        false
    }

    #[inline]
    fn damage(&self) -> f32 {
        self.damage
    }
}
