use crate::galaxy_hierarchy::{Cluster, Team, TeamId};
use crate::network::PacketReader;
use crate::unit::{AbstractUnit, Unit, UnitCastTable, UnitHierarchy, UnitInternal};
use crate::utils::Atomic;
use crate::{GameError, Vector};
use std::sync::{Arc, Weak};

pub(crate) trait NpcUnitInternal {
    fn parent(&self) -> &dyn NpcUnit;
}

/// Base type for stationary visible NPC units.
pub trait NpcUnit: NpcUnitInternal + Unit {
    /// Current hull value.
    #[inline]
    fn hull(&self) -> f32 {
        NpcUnitInternal::parent(self).hull()
    }

    /// Maximum hull value.
    #[inline]
    fn hull_maximum(&self) -> f32 {
        NpcUnitInternal::parent(self).hull_maximum()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AbstractNpcUnit {
    parent: AbstractUnit,
    team: Weak<Team>,
    position: Atomic<Vector>,
    radius: Atomic<f32>,
    hull: Atomic<f32>,
    hull_maximum: Atomic<f32>,
}

impl AbstractNpcUnit {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Self, GameError> {
        let galaxy = cluster.upgrade().unwrap().galaxy();
        Ok(Self {
            parent: AbstractUnit::new(cluster, name),
            team: Arc::downgrade(&galaxy.get_team(TeamId(reader.read_byte()))),
            position: Atomic::from_reader(reader),
            radius: Atomic::from_reader(reader),
            hull: Atomic::from(0.0),
            hull_maximum: Atomic::from(0.0),
        })
    }
}

impl UnitInternal for AbstractNpcUnit {
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.hull.read(reader);
        self.hull_maximum.read(reader);
    }
}

impl UnitCastTable for AbstractNpcUnit {
    cast_fn!(npc_unit_cast_fn, AbstractNpcUnit, dyn NpcUnit);
}

impl UnitHierarchy for AbstractNpcUnit {
    #[inline]
    fn as_npc_unit(&self) -> Option<&dyn NpcUnit> {
        Some(self)
    }
}

impl Unit for AbstractNpcUnit {
    #[inline]
    fn radius(&self) -> f32 {
        self.radius.load()
    }

    #[inline]
    fn position(&self) -> Vector {
        self.position.load()
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

#[forbid(clippy::missing_trait_methods)]
impl NpcUnitInternal for AbstractNpcUnit {
    fn parent(&self) -> &dyn NpcUnit {
        unreachable!()
    }
}

#[forbid(clippy::missing_trait_methods)]
impl NpcUnit for AbstractNpcUnit {
    #[inline]
    fn hull(&self) -> f32 {
        self.hull.load()
    }

    #[inline]
    fn hull_maximum(&self) -> f32 {
        self.hull_maximum.load()
    }
}
