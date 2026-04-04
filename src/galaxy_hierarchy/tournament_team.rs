use crate::account::Account;
use crate::galaxy_hierarchy::Team;
use std::sync::Arc;

/// Snapshot of one team inside a configured tournament.
#[derive(Debug, Clone)]
pub struct TournamentTeam {
    team: Arc<Team>,
    participants: Vec<Arc<Account>>,
    wins: i32,
}

impl TournamentTeam {
    pub(crate) fn new(team: Arc<Team>, participants: Vec<Arc<Account>>, wins: i32) -> Self {
        Self {
            team,
            participants,
            wins,
        }
    }

    /// The normal galaxy team that participates in the tournament.
    #[inline]
    pub fn team(&self) -> &Arc<Team> {
        &self.team
    }

    /// Accounts assigned to this tournament team.
    #[inline]
    pub fn participants(&self) -> &[Arc<Account>] {
        &self.participants
    }

    /// Number of matches already won by this team in the mirrored match history.
    #[inline]
    pub fn wins(&self) -> i32 {
        self.wins
    }
}
