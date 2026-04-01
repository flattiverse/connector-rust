use crate::galaxy_hierarchy::{Cluster, Team, TeamId};
use crate::network::PacketReader;
use crate::unit::{
    AbstractSteadyUnit, SteadyUnit, SteadyUnitInternal, Unit, UnitHierarchy, UnitInternal,
};
use crate::utils::Let;
use crate::GameError;
use arc_swap::ArcSwapWeak;
use std::sync::{Arc, Weak};

/// Shared base class for team-bound target units.
pub trait TargetUnitInternal {}
pub trait TargetUnit: TargetUnitInternal + SteadyUnit {}

#[derive(Debug)]
pub(crate) struct AbstractTargetUnit {
    pub(crate) parent: AbstractSteadyUnit,
    team: ArcSwapWeak<Team>,
}

impl Clone for AbstractTargetUnit {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            team: ArcSwapWeak::new(self.team.load_full()),
        }
    }
}

impl AbstractTargetUnit {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Self, GameError> {
        Ok(
            AbstractSteadyUnit::new(cluster, name, reader)?.r#let(|parent| Self {
                team: ArcSwapWeak::new(Arc::downgrade(
                    &parent
                        .cluster()
                        .galaxy()
                        .get_team(TeamId(reader.read_byte())),
                )),
                parent,
            }),
        )
    }

    /// Updates the owning team for this target during runtime sync.
    pub(crate) fn update_target_team(&self, team: Weak<Team>) {
        self.team.store(team);
    }
}

impl UnitInternal for AbstractTargetUnit {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }
}

impl UnitHierarchy for AbstractTargetUnit {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_target_unit(&self) -> Option<&dyn TargetUnit> {
        Some(self)
    }
}

impl Unit for AbstractTargetUnit {
    #[inline]
    fn is_masking(&self) -> bool {
        false
    }

    #[inline]
    fn is_solid(&self) -> bool {
        false
    }

    #[inline]
    fn team(&self) -> Weak<Team> {
        self.team.load_full()
    }
}

impl SteadyUnitInternal for AbstractTargetUnit {}

impl SteadyUnit for AbstractTargetUnit {}

impl TargetUnitInternal for AbstractTargetUnit {}

impl TargetUnit for AbstractTargetUnit {}
