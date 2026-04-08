use crate::galaxy_hierarchy::{Cluster, Team, TeamId};
use crate::network::PacketReader;
use crate::unit::{
    AbstractMobileUnit, MobileUnit, MobileUnitInternal, Unit, UnitCastTable, UnitHierarchy,
    UnitInternal,
};
use crate::utils::Atomic;
use crate::GameError;
use std::sync::{Arc, Weak};

pub(crate) trait MobileNpcUnitInternal {
    fn parent(&self) -> &dyn MobileNpcUnit;
}

/// Base type for visible mobile NPC units.
pub trait MobileNpcUnit: MobileNpcUnitInternal + MobileUnit {
    #[inline]
    fn hull(&self) -> f32 {
        MobileNpcUnitInternal::parent(self).hull()
    }

    #[inline]
    fn hull_max(&self) -> f32 {
        MobileNpcUnitInternal::parent(self).hull_max()
    }
}

#[derive(Debug)]
pub(crate) struct AbstractMobileNpcUnit {
    parent: AbstractMobileUnit,
    team: Weak<Team>,
    radius: Atomic<f32>,
    hull: Atomic<f32>,
    hull_maximum: Atomic<f32>,
}

impl AbstractMobileNpcUnit {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Self, GameError> {
        let galaxy = cluster.upgrade().unwrap().galaxy();
        Ok(Self {
            parent: AbstractMobileUnit::new(cluster, name),
            team: Arc::downgrade(&galaxy.get_team(TeamId(reader.read_byte()))),
            radius: Atomic::from_reader(reader),
            hull: Atomic::from(0.0),
            hull_maximum: Atomic::from(0.0),
        })
    }
}

impl UnitInternal for AbstractMobileNpcUnit {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.hull.read(reader);
        self.hull_maximum.read(reader);
    }
}

impl UnitCastTable for AbstractMobileNpcUnit {
    cast_fn!(mobile_unit_cast_fn, AbstractMobileNpcUnit, dyn MobileUnit);
    cast_fn!(
        mobile_npc_unit_cast_fn,
        AbstractMobileNpcUnit,
        dyn MobileNpcUnit
    );
}

impl UnitHierarchy for AbstractMobileNpcUnit {
    #[inline]
    fn as_mobile_unit(&self) -> Option<&dyn MobileUnit> {
        Some(self)
    }

    #[inline]
    fn as_mobile_npc_unit(&self) -> Option<&dyn MobileNpcUnit> {
        Some(self)
    }
}

impl Unit for AbstractMobileNpcUnit {
    #[inline]
    fn radius(&self) -> f32 {
        self.radius.load()
    }

    #[inline]
    fn can_be_edited(&self) -> bool {
        true
    }

    #[inline]
    fn team(&self) -> Weak<Team> {
        self.team.clone()
    }
}

impl MobileUnitInternal for AbstractMobileNpcUnit {
    #[inline]
    fn parent(&self) -> &dyn MobileUnit {
        &self.parent
    }
}

impl MobileUnit for AbstractMobileNpcUnit {}

#[forbid(clippy::missing_trait_methods)]
impl MobileNpcUnitInternal for AbstractMobileNpcUnit {
    fn parent(&self) -> &dyn MobileNpcUnit {
        unreachable!()
    }
}

#[forbid(clippy::missing_trait_methods)]
impl MobileNpcUnit for AbstractMobileNpcUnit {
    #[inline]
    fn hull(&self) -> f32 {
        self.hull.load()
    }

    #[inline]
    fn hull_max(&self) -> f32 {
        self.hull_maximum.load()
    }
}
