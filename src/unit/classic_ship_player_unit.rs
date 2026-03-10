use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{PlayerUnit, UnitBase};
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
