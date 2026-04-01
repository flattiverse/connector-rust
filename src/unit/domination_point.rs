use crate::galaxy_hierarchy::{Cluster, TeamId};
use crate::network::PacketReader;
use crate::unit::{
    AbstractTargetUnit, SteadyUnit, SteadyUnitInternal, TargetUnit, TargetUnitInternal, Unit,
    UnitHierarchy, UnitInternal, UnitKind,
};
use crate::utils::Atomic;
use crate::GameError;
use std::sync::{Arc, Weak};

/// A domination point target with live domination state.
#[derive(Debug, Clone)]
pub struct DominationPoint {
    parent: AbstractTargetUnit,
    domination_radius: f32,
    domination: Atomic<i32>,
    score_countdown: Atomic<i32>,
}

impl DominationPoint {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractTargetUnit::new(cluster, name, reader)?,
            domination_radius: reader.read_f32(),
            domination: Atomic::default(),
            score_countdown: Atomic::default(),
        }))
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

impl UnitInternal for DominationPoint {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        let team_id = reader.read_byte();
        let team = self.cluster().galaxy().get_team_opt(TeamId(team_id));

        self.domination.read(reader);
        self.score_countdown.read(reader);

        self.parent
            .update_target_team(team.as_ref().map(Arc::downgrade).unwrap_or_default());
    }
}

impl UnitHierarchy for DominationPoint {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_target_unit(&self) -> Option<&dyn TargetUnit> {
        Some(self)
    }

    #[inline]
    fn as_domination_point(&self) -> Option<&DominationPoint> {
        Some(self)
    }
}

impl Unit for DominationPoint {
    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::DominationPoint
    }
}

impl SteadyUnitInternal for DominationPoint {}

impl SteadyUnit for DominationPoint {}

impl TargetUnitInternal for DominationPoint {}

impl TargetUnit for DominationPoint {}
