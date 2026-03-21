use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{PlayerUnit, UnitBase, UnitExt, UnitExtSealed, UnitKind};
use std::sync::Weak;

/// A classic ship for noobs.
#[derive(Debug, Clone)]
pub struct ClassicShipPlayerUnit {
    base: UnitBase,
    player_unit: PlayerUnit,
}

impl ClassicShipPlayerUnit {
    pub(crate) fn read(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Self {
        let galaxy = cluster.upgrade().unwrap().galaxy();

        Self {
            base: UnitBase::new(cluster, name),
            player_unit: PlayerUnit::read(&*galaxy, reader),
        }
    }

    #[inline]
    pub const fn gravity(&self) -> f32 {
        0.0012
    }

    #[inline]
    pub const fn radius(&self) -> f32 {
        14.0
    }
}

impl AsRef<UnitBase> for ClassicShipPlayerUnit {
    #[inline]
    fn as_ref(&self) -> &UnitBase {
        &self.base
    }
}

impl AsRef<PlayerUnit> for ClassicShipPlayerUnit {
    #[inline]
    fn as_ref(&self) -> &PlayerUnit {
        &self.player_unit
    }
}

impl<'a> UnitExtSealed<'a> for &'a ClassicShipPlayerUnit {
    type Parent = (&'a UnitBase, &'a PlayerUnit);

    #[inline]
    fn parent(self) -> (&'a UnitBase, &'a PlayerUnit) {
        (&self.base, &self.player_unit)
    }
}

impl<'a> UnitExt<'a> for &'a ClassicShipPlayerUnit {
    #[inline]
    fn radius(self) -> f32 {
        14.0
    }

    #[inline]
    fn gravity(self) -> f32 {
        0.0012
    }

    #[inline]
    fn kind(self) -> UnitKind {
        UnitKind::ClassicShipPlayerUnit
    }
}
