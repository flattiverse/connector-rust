use crate::galaxy_hierarchy::{
    Cluster, ControllableInfo, ControllableInfoId, Player, PlayerId, Team,
};
use crate::network::PacketReader;
use crate::unit::{
    AbstractMobileUnit, MobileUnit, MobileUnitInternal, Unit, UnitCastTable, UnitHierarchy,
    UnitInternal,
};
use crate::utils::{Atomic, Let};
use crate::GameError;
use std::sync::{Arc, Weak};

pub(crate) trait ProjectileInternal {
    fn parent(&self) -> &dyn Projectile;
}

/// Base type for visible projectile units such as shots and interceptors.
/// The owning player information can be absent for neutral or system-generated projectiles.
#[allow(private_bounds)]
pub trait Projectile: ProjectileInternal + Unit {
    /// Player that launched the projectile, or yields `None` if no player-owned source is known.
    #[inline]
    fn player(&self) -> &Weak<Player> {
        ProjectileInternal::parent(self).player()
    }

    /// Controllable entry that launched the projectile, or yields `None` if no player-owned source
    /// is known.
    #[inline]
    fn controllable_info(&self) -> &Weak<ControllableInfo> {
        ProjectileInternal::parent(self).controllable_info()
    }

    /// The remaining projectile lifetime in ticks.
    #[inline]
    fn ticks(&self) -> u16 {
        ProjectileInternal::parent(self).ticks()
    }

    /// Explosion load of the projectile.
    /// This becomes meaningful once the full projectile state is known.
    #[inline]
    fn load(&self) -> f32 {
        ProjectileInternal::parent(self).load()
    }

    /// Direct damage of the projectile.
    /// This becomes meaningful once the full projectile state is known.
    #[inline]
    fn damage(&self) -> f32 {
        ProjectileInternal::parent(self).damage()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AbstractProjectile {
    parent: AbstractMobileUnit,
    player: Weak<Player>,
    controllable_info: Weak<ControllableInfo>,
    ticks: Atomic<u16>,
    load: Atomic<f32>,
    damage: Atomic<f32>,
}

impl AbstractProjectile {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Self, GameError> {
        AbstractMobileUnit::new(cluster, name).r#let(|parent| {
            let galaxy = parent.cluster().galaxy();
            let player_id = PlayerId(reader.read_byte());
            let controllable_id = ControllableInfoId(reader.read_byte());
            let ticks = reader.read_uint16();

            let (player, controllable_info) = if player_id.0 < 192 {
                let player = galaxy.get_player(player_id);
                let controllable_info = player.get_controllable_info(controllable_id);
                (Arc::downgrade(&player), Arc::downgrade(&controllable_info))
            } else {
                (Weak::default(), Weak::default())
            };

            parent.read_position_and_movement(reader);

            Ok(Self {
                parent,
                player,
                controllable_info,
                ticks: Atomic::from(ticks),
                load: Atomic::from(0.0),
                damage: Atomic::from(0.0),
            })
        })
    }
}

impl UnitInternal for AbstractProjectile {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_movement(&self, reader: &mut dyn PacketReader) {
        self.ticks.read(reader);
        self.parent.read_position_and_movement(reader);
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);
        self.load.read(reader);
        self.damage.read(reader);
    }
}

impl UnitCastTable for AbstractProjectile {
    cast_fn!(mobile_unit_cast_fn, AbstractProjectile, dyn MobileUnit);
    cast_fn!(projectile_cast_fn, AbstractProjectile, dyn Projectile);
}

impl UnitHierarchy for AbstractProjectile {
    #[inline]
    fn as_mobile_unit(&self) -> Option<&dyn MobileUnit> {
        Some(self)
    }

    #[inline]
    fn as_projectile(&self) -> Option<&dyn Projectile> {
        Some(self)
    }
}

impl Unit for AbstractProjectile {
    #[inline]
    fn radius(&self) -> f32 {
        1.0
    }

    #[inline]
    fn is_masking(&self) -> bool {
        false
    }

    #[inline]
    fn team(&self) -> Weak<Team> {
        self.player
            .upgrade()
            .map(|p| p.team_weak())
            .unwrap_or_default()
    }
}

impl MobileUnitInternal for AbstractProjectile {
    #[inline]
    fn parent(&self) -> &dyn MobileUnit {
        &self.parent
    }
}

impl MobileUnit for AbstractProjectile {}

#[forbid(clippy::missing_trait_methods)]
impl ProjectileInternal for AbstractProjectile {
    fn parent(&self) -> &dyn Projectile {
        unreachable!()
    }
}

#[forbid(clippy::missing_trait_methods)]
impl Projectile for AbstractProjectile {
    #[inline]
    fn player(&self) -> &Weak<Player> {
        &self.player
    }

    #[inline]
    fn controllable_info(&self) -> &Weak<ControllableInfo> {
        &self.controllable_info
    }

    #[inline]
    fn ticks(&self) -> u16 {
        self.ticks.load()
    }

    #[inline]
    fn load(&self) -> f32 {
        self.load.load()
    }

    #[inline]
    fn damage(&self) -> f32 {
        self.damage.load()
    }
}
