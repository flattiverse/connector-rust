use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractSteadyUnit, SteadyUnit, SteadyUnitInternal, Unit, UnitCastTable, UnitHierarchy,
    UnitInternal, UnitKind,
};
use crate::utils::Atomic;
use crate::GameError;
use std::sync::{Arc, Weak};

/// Visible nebula map unit that can be harvested by nebula collectors and acts as the source material for crystals.
#[derive(Debug, Clone)]
pub struct Nebula {
    parent: AbstractSteadyUnit,
    hue: Atomic<f32>,
}

impl Nebula {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractSteadyUnit::new(cluster, name, reader)?,
            hue: Atomic::from(0.0),
        }))
    }

    /// Hue value of the nebula material.
    #[inline]
    pub fn hue(&self) -> f32 {
        self.hue.load()
    }
}

impl UnitInternal for Nebula {
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);
        self.hue.read(reader);
    }
}

impl UnitCastTable for Nebula {
    cast_fn!(steady_unit_cast_fn, Nebula, dyn SteadyUnit);
}

impl UnitHierarchy for Nebula {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_nebula(&self) -> Option<&Nebula> {
        Some(self)
    }
}

impl Unit for Nebula {
    #[inline]
    fn is_masking(&self) -> bool {
        false
    }

    #[inline]
    fn is_solid(&self) -> bool {
        false
    }

    #[inline]
    fn can_be_edited(&self) -> bool {
        true
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Nebula
    }
}

impl SteadyUnitInternal for Nebula {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for Nebula {}
