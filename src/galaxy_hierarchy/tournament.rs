use crate::galaxy_hierarchy::{
    TournamentMatchResult, TournamentMode, TournamentStage, TournamentTeam,
};

#[derive(Debug, Clone)]
pub struct Tournament {
    stage: TournamentStage,
    mode: TournamentMode,
    duration_ticks: u32,
    teams: Vec<TournamentTeam>,
    match_history: Vec<TournamentMatchResult>,
}

impl Tournament {
    pub(crate) fn new(
        stage: TournamentStage,
        mode: TournamentMode,
        duration_ticks: u32,
        teams: Vec<TournamentTeam>,
        match_history: Vec<TournamentMatchResult>,
    ) -> Self {
        Self {
            stage,
            mode,
            duration_ticks,
            teams,
            match_history,
        }
    }

    /// Current lifecycle stage of the tournament.
    #[inline]
    pub fn stage(&self) -> &TournamentStage {
        &self.stage
    }

    /// Match format used by the tournament, for example [`TournamentMode::BestOf5`].
    #[inline]
    pub fn mode(&self) -> &TournamentMode {
        &self.mode
    }

    /// Planned match duration in server ticks for each game of the series.
    #[inline]
    pub fn duration_ticks(&self) -> u32 {
        self.duration_ticks
    }

    /// Participating teams together with their configured account list and currently known win
    /// count.
    #[inline]
    pub fn teams(&self) -> &[TournamentTeam] {
        &self.teams
    }

    /// Ordered history of already finished matches.
    #[inline]
    pub fn match_history(&self) -> &[TournamentMatchResult] {
        &self.match_history
    }

    /// Number of the current or next live match, derived as `match_history().len() + 1`.
    #[inline]
    pub fn current_match_number(&self) -> i32 {
        self.match_history.len() as i32 + 1
    }
}
