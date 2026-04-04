use crate::account::AccountId;
use crate::galaxy_hierarchy::Team;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TournamentTeamConfiguration {
    team: Arc<Team>,
    account_ids: Vec<AccountId>,
}

impl TournamentTeamConfiguration {
    /// Creates one tournament-team configuration entry.
    ///
    /// * `team` Galaxy team that should participate in the tournament.
    /// * `account_ids` Persistent account ids assigned to that team.
    #[inline]
    pub fn new(team: Arc<Team>, account_ids: impl Into<Vec<AccountId>>) -> Self {
        Self {
            team,
            account_ids: account_ids.into(),
        }
    }

    /// Galaxy [`Team`] that should be used as tournament side.
    #[inline]
    pub fn team(&self) -> &Arc<Team> {
        &self.team
    }

    /// Persistent account ids assigned to the [`Team`] of this configuration.
    #[inline]
    pub fn account_ids(&self) -> impl Iterator<Item = AccountId> + '_ {
        self.account_ids.iter().copied()
    }
}
