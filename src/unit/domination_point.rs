use crate::galaxy_hierarchy::{Cluster, TeamId};
use crate::network::PacketReader;
use crate::unit::{SteadyUnit, TargetUnit, UnitBase, UnitExt, UnitExtSealed, UnitKind};
use crate::utils::{Atomic, Readable};
use std::sync::{Arc, Weak};

/// A domination point target with live domination state.
#[derive(Debug, Clone)]
pub struct DominationPoint {
    base: UnitBase,
    steady: SteadyUnit,
    target: TargetUnit,
    domination_radius: f32,
    domination: Atomic<i32>,
    score_countdown: Atomic<i32>,
}

impl DominationPoint {
    pub(crate) fn read(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            base: UnitBase::new(cluster.clone(), name),
            steady: SteadyUnit::read(reader),
            target: TargetUnit::read(&cluster, reader),
            domination_radius: reader.read_f32(),
            domination: Atomic::default(),
            score_countdown: Atomic::default(),
        }
    }

    /// Radius in which ships influence domination.
    #[inline]
    pub fn domination_radius(&self) -> f32 {
        self.domination_radius
    }

    /// Current domination progress.
    #[inline]
    pub fn domination(&self) -> i32 {
        self.domination.load()
    }

    /// Current score countdown while fully controlled.
    #[inline]
    pub fn score_countdown(&self) -> i32 {
        self.score_countdown.load()
    }
}

impl AsRef<UnitBase> for DominationPoint {
    #[inline]
    fn as_ref(&self) -> &UnitBase {
        &self.base
    }
}

impl AsRef<SteadyUnit> for DominationPoint {
    #[inline]
    fn as_ref(&self) -> &SteadyUnit {
        &self.steady
    }
}
impl AsRef<TargetUnit> for DominationPoint {
    #[inline]
    fn as_ref(&self) -> &TargetUnit {
        &self.target
    }
}

impl<'a> UnitExtSealed<'a> for &'a DominationPoint {
    type Parent = (&'a UnitBase, &'a SteadyUnit, &'a TargetUnit);

    #[inline]
    fn parent(self) -> Self::Parent {
        (&self.base, &self.steady, &self.target)
    }

    fn update_state(self, reader: &mut dyn PacketReader) {
        self.parent().update_state(reader);

        let team_id = reader.read_byte();
        let team = self.cluster().galaxy().get_team_opt(TeamId(team_id));

        self.domination.read(reader);
        self.score_countdown.read(reader);

        self.target
            .update_target_team(team.as_ref().map(Arc::downgrade).unwrap_or_default());
    }
}

impl<'a> UnitExt<'a> for &'a DominationPoint {
    #[inline]
    fn kind(self) -> UnitKind {
        UnitKind::DominationPoint
    }
}
