use crate::galaxy_hierarchy::{Cluster, Team, TeamId};
use crate::network::PacketReader;
use crate::unit::{SteadyUnit, UnitBase, UnitExt, UnitExtSealed};
use arc_swap::ArcSwapWeak;
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub struct TargetUnit {
    team: ArcSwapWeak<Team>,
}

impl Clone for TargetUnit {
    fn clone(&self) -> Self {
        Self {
            team: ArcSwapWeak::new(self.team.load_full()),
        }
    }
}

impl TargetUnit {
    pub(crate) fn read(cluster: &Weak<Cluster>, reader: &mut dyn PacketReader) -> Self {
        let galaxy = cluster.upgrade().unwrap().galaxy();

        Self {
            team: ArcSwapWeak::new(Arc::downgrade(&galaxy.get_team(TeamId(reader.read_byte())))),
        }
    }

    pub(crate) fn update_target_team(&self, team: Weak<Team>) {
        self.team.store(team);
    }
}

impl<'a> UnitExtSealed<'a> for (&'a UnitBase, &'a SteadyUnit, &'a TargetUnit) {
    type Parent = (&'a UnitBase, &'a SteadyUnit);

    #[inline]
    fn parent(self) -> Self::Parent {
        (&self.0, &self.1)
    }
}

impl<'a> UnitExt<'a> for (&'a UnitBase, &'a SteadyUnit, &'a TargetUnit) {
    #[inline]
    fn is_masking(self) -> bool {
        false
    }

    #[inline]
    fn is_solid(self) -> bool {
        false
    }

    #[inline]
    fn team(self) -> Weak<Team> {
        self.2.team.load_full()
    }
}
