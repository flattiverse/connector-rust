use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractMobileUnit, MobileUnit, MobileUnitInternal, Unit, UnitHierarchy, UnitInternal,
};
use crate::utils::{Also, Atomic};
use crate::GameError;
use std::sync::Weak;

pub(crate) trait StormWhirlInternal {}

/// Base type for storm whirls.
#[allow(private_bounds)]
pub trait StormWhirl: StormWhirlInternal + Unit {}

#[derive(Debug, Clone)]
pub(crate) struct AbstractStormWhirl {
    parent: AbstractMobileUnit,
    radius: Atomic<f32>,
    gravity: Atomic<f32>,
    remaining_ticks: Atomic<u16>,
}

impl AbstractStormWhirl {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Self, GameError> {
        Ok(Self {
            parent: AbstractMobileUnit::new(cluster, name).also(|parent| {
                parent.position.read(reader);
                parent.movement.read(reader);
            }),
            radius: Atomic::from_reader(reader),
            gravity: Atomic::from_reader(reader),
            remaining_ticks: Atomic::default(),
        })
    }

    pub fn read_remaining_ticks(&self, reader: &mut dyn PacketReader) {
        self.remaining_ticks.read(reader);
    }
}

impl UnitInternal for AbstractStormWhirl {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitHierarchy for AbstractStormWhirl {
    #[inline]
    fn as_mobile_unit(&self) -> Option<&dyn MobileUnit> {
        Some(self)
    }

    #[inline]
    fn as_storm_whirl(&self) -> Option<&dyn StormWhirl> {
        Some(self)
    }
}

impl Unit for AbstractStormWhirl {
    fn radius(&self) -> f32 {
        self.radius.load()
    }

    fn is_solid(&self) -> bool {
        false
    }

    fn gravity(&self) -> f32 {
        self.gravity.load()
    }
}

impl MobileUnitInternal for AbstractStormWhirl {}

impl MobileUnit for AbstractStormWhirl {}

#[forbid(clippy::missing_trait_methods)]
impl StormWhirlInternal for AbstractStormWhirl {}

#[forbid(clippy::missing_trait_methods)]
impl StormWhirl for AbstractStormWhirl {}
