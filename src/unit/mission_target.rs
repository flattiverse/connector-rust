use crate::galaxy_hierarchy::{Cluster, Team, TeamId};
use crate::network::PacketReader;
use crate::unit::{SteadyUnit, UnitBase, UnitExt, UnitExtSealed, UnitKind};
use crate::utils::Readable;
use crate::Vector;
use std::sync::{Arc, Weak};

/// A mission target with configurable waypoint vectors.
#[derive(Debug, Clone)]
pub struct MissionTarget {
    base: UnitBase,
    steady: SteadyUnit,
    team: Weak<Team>,
    vectors: Vec<Vector>,
}

impl MissionTarget {
    pub(crate) fn read(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Self {
        let galaxy = cluster.upgrade().unwrap().galaxy();

        Self {
            base: UnitBase::new(cluster, name),
            steady: SteadyUnit::read(reader),
            team: Arc::downgrade(&galaxy.get_team(TeamId(reader.read_byte()))),
            vectors: {
                let vector_count = reader.read_uint16() as usize;
                let mut vectors = Vec::with_capacity(vector_count);

                for _ in 0..vector_count {
                    vectors.push(Vector::from_read(reader));
                }

                vectors
            },
        }
    }

    /// Returns all configured waypoint vectors.
    #[inline]
    pub fn vectors(&self) -> &[Vector] {
        &self.vectors
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

impl<'a> UnitExtSealed<'a> for &'a MissionTarget {
    type Parent = (&'a UnitBase, &'a SteadyUnit);

    fn parent(self) -> Self::Parent {
        (&self.base, &self.steady)
    }
}

impl<'a> UnitExt<'a> for &'a MissionTarget {
    #[inline]
    fn is_masking(self) -> bool {
        false
    }

    #[inline]
    fn is_solid(self) -> bool {
        false
    }

    #[inline]
    fn kind(self) -> UnitKind {
        UnitKind::MissionTarget
    }

    #[inline]
    fn team(self) -> Weak<Team>
    where
        Self: Sized,
    {
        self.team.clone()
    }
}
