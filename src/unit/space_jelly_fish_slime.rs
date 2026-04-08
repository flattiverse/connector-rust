use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractProjectile, MobileUnit, MobileUnitInternal, Projectile, ProjectileInternal, Unit,
    UnitCastTable, UnitHierarchy, UnitInternal, UnitKind,
};
use crate::utils::Atomic;
use crate::GameError;
use arc_swap::ArcSwap;
use num_enum::FromPrimitive;
use std::sync::{Arc, Weak};

/// Homing biological projectile spawned by a space jellyfish.
#[derive(Debug)]
pub struct SpaceJellyFishSlime {
    parent: AbstractProjectile,
    target_cluster_id: Atomic<u8>,
    target_unit_name: ArcSwap<String>,
    target_unit_kind: Atomic<Option<UnitKind>>,
}

impl Clone for SpaceJellyFishSlime {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            target_cluster_id: self.target_cluster_id.clone(),
            target_unit_name: ArcSwap::from(self.target_unit_name.load_full()),
            target_unit_kind: self.target_unit_kind.clone(),
        }
    }
}

impl SpaceJellyFishSlime {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractProjectile::new(cluster, name, reader)?,
            target_cluster_id: Atomic::from(u8::MAX),
            target_unit_name: ArcSwap::new(String::new().into()),
            target_unit_kind: Atomic::from(None),
        }))
    }

    /// Target cluster id, or 255 if no target is currently known.
    #[inline]
    pub fn target_cluster_id(&self) -> u8 {
        self.target_cluster_id.load()
    }

    /// Target unit name, or empty if no target is currently known.
    #[inline]
    pub fn target_unit_name(&self) -> Arc<String> {
        self.target_unit_name.load_full()
    }

    /// Target unit kind, or `None` if no target is currently known.
    #[inline]
    pub fn target_unit_kind(&self) -> Option<UnitKind> {
        self.target_unit_kind.load()
    }
}

impl UnitInternal for SpaceJellyFishSlime {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        let target_cluster_id = reader.read_byte();
        let target_unit_name = reader.read_string();
        let target_kind_value = reader.read_byte();

        if target_cluster_id == u8::MAX
            || target_kind_value == u8::MAX
            || target_unit_name.is_empty()
        {
            self.target_cluster_id.store(u8::MAX);
            self.target_unit_name.store(Arc::new(String::new()));
            self.target_unit_kind.store(None);
        } else {
            self.target_cluster_id.store(target_cluster_id);
            self.target_unit_name.store(Arc::new(target_unit_name));
            self.target_unit_kind
                .store(Some(UnitKind::from_primitive(target_kind_value)));
        }
    }
}

impl UnitCastTable for SpaceJellyFishSlime {
    cast_fn!(mobile_unit_cast_fn, SpaceJellyFishSlime, dyn MobileUnit);
    cast_fn!(projectile_cast_fn, SpaceJellyFishSlime, dyn Projectile);
}

impl UnitHierarchy for SpaceJellyFishSlime {
    #[inline]
    fn as_mobile_unit(&self) -> Option<&dyn MobileUnit> {
        Some(self)
    }

    #[inline]
    fn as_projectile(&self) -> Option<&dyn Projectile> {
        Some(self)
    }

    #[inline]
    fn as_space_jelly_fish_slime(&self) -> Option<&SpaceJellyFishSlime> {
        Some(self)
    }
}

impl Unit for SpaceJellyFishSlime {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::SpaceJellyFishSlime
    }
}

impl MobileUnitInternal for SpaceJellyFishSlime {
    #[inline]
    fn parent(&self) -> &dyn MobileUnit {
        &self.parent
    }
}

impl MobileUnit for SpaceJellyFishSlime {}

impl ProjectileInternal for SpaceJellyFishSlime {
    #[inline]
    fn parent(&self) -> &dyn Projectile {
        &self.parent
    }
}

impl Projectile for SpaceJellyFishSlime {}
