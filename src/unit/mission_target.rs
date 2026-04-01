use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractTargetUnit, SteadyUnit, SteadyUnitInternal, TargetUnit, TargetUnitInternal, Unit,
    UnitHierarchy, UnitInternal, UnitKind,
};
use crate::utils::Atomic;
use crate::{GameError, Vector};
use arc_swap::{ArcSwap, Guard};
use std::sync::{Arc, Weak};

/// A mission target with a sequence number and configurable waypoint vectors.
#[derive(Debug)]
pub struct MissionTarget {
    parent: AbstractTargetUnit,
    sequence_number: Atomic<i32>,
    vectors: ArcSwap<Vec<Vector>>,
}

impl Clone for MissionTarget {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            sequence_number: Atomic::default(),
            vectors: ArcSwap::new(self.vectors.load_full()),
        }
    }
}

impl MissionTarget {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractTargetUnit::new(cluster.clone(), name, reader)?,
            sequence_number: Atomic::default(),
            vectors: ArcSwap::default(),
        }))
    }

    /// Sequence number of this mission target.
    #[inline]
    pub fn sequence_number(&self) -> i32 {
        self.sequence_number.load()
    }

    /// Returns all configured waypoint vectors.
    #[inline]
    pub fn vectors(&self) -> Guard<Arc<Vec<Vector>>> {
        self.vectors.load()
    }
}

impl UnitInternal for MissionTarget {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.sequence_number.read(reader);

        let vector_count = reader.read_uint16() as usize;
        let mut vectors = Vec::with_capacity(vector_count);

        for _ in 0..vector_count {
            vectors.push(Vector::from_read(reader));
        }

        self.vectors.store(Arc::new(vectors));
    }
}

impl UnitHierarchy for MissionTarget {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_target_unit(&self) -> Option<&dyn TargetUnit> {
        Some(self)
    }

    #[inline]
    fn as_mission_target(&self) -> Option<&MissionTarget> {
        Some(self)
    }
}

impl Unit for MissionTarget {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::MissionTarget
    }
}

impl SteadyUnitInternal for MissionTarget {}

impl SteadyUnit for MissionTarget {}

impl TargetUnitInternal for MissionTarget {}

impl TargetUnit for MissionTarget {}
