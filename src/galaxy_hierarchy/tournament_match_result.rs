use crate::galaxy_hierarchy::Team;
use std::sync::Arc;

/// Result entry for one already finished tournament match.
#[derive(Debug, Clone)]
pub struct TournamentMatchResult {
    match_number: i32,
    winning_team: Arc<Team>,
}

impl TournamentMatchResult {
    pub(crate) fn new(match_number: i32, winning_team: Arc<Team>) -> Self {
        Self {
            match_number,
            winning_team,
        }
    }

    /// One-based match number within the tournament series.
    #[inline]
    pub fn match_number(&self) -> i32 {
        self.match_number
    }

    /// Team that won the referenced match.
    #[inline]
    pub fn winning_team(&self) -> &Arc<Team> {
        &self.winning_team
    }
}
