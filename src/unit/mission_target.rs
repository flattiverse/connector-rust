use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{SteadyUnit, TargetUnit, UnitBase, UnitExt, UnitExtSealed, UnitKind};
use crate::utils::{Atomic, Readable};
use crate::Vector;
use arc_swap::{ArcSwap, Guard};
use std::sync::{Arc, Weak};

/// A mission target with a sequence number and configurable waypoint vectors.
#[derive(Debug)]
pub struct MissionTarget {
    base: UnitBase,
    steady: SteadyUnit,
    target: TargetUnit,
    sequence_number: Atomic<i32>,
    vectors: ArcSwap<Vec<Vector>>,
}

impl Clone for MissionTarget {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            steady: self.steady.clone(),
            target: self.target.clone(),
            sequence_number: Atomic::default(),
            vectors: ArcSwap::new(self.vectors.load_full()),
        }
    }
}

impl MissionTarget {
    pub(crate) fn read(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            base: UnitBase::new(cluster.clone(), name),
            steady: SteadyUnit::read(reader),
            target: TargetUnit::read(&cluster, reader),
            sequence_number: Atomic::default(),
            vectors: ArcSwap::default(),
        }
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

impl AsRef<UnitBase> for MissionTarget {
    #[inline]
    fn as_ref(&self) -> &UnitBase {
        &self.base
    }
}

impl AsRef<SteadyUnit> for MissionTarget {
    #[inline]
    fn as_ref(&self) -> &SteadyUnit {
        &self.steady
    }
}

impl AsRef<TargetUnit> for MissionTarget {
    #[inline]
    fn as_ref(&self) -> &TargetUnit {
        &self.target
    }
}

impl<'a> UnitExtSealed<'a> for &'a MissionTarget {
    type Parent = (&'a UnitBase, &'a SteadyUnit, &'a TargetUnit);

    #[inline]
    fn parent(self) -> Self::Parent {
        (&self.base, &self.steady, &self.target)
    }

    fn update_state(self, reader: &mut dyn PacketReader) {
        self.parent().update_state(reader);

        self.sequence_number.read(reader);

        let vector_count = reader.read_uint16() as usize;
        let mut vectors = Vec::with_capacity(vector_count);

        for _ in 0..vector_count {
            vectors.push(Vector::from_read(reader));
        }

        self.vectors.store(Arc::new(vectors));
    }
}

impl<'a> UnitExt<'a> for &'a MissionTarget {
    #[inline]
    fn kind(self) -> UnitKind {
        UnitKind::MissionTarget
    }
}
