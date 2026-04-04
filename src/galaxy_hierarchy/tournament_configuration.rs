use crate::galaxy_hierarchy::{TeamId, TournamentMode, TournamentTeamConfiguration};

#[derive(Debug, Clone)]
pub struct TournamentConfiguration {
    mode: TournamentMode,
    duration_ticks: u32,
    teams: Vec<TournamentTeamConfiguration>,
    winning_team_ids: Vec<TeamId>,
}

impl TournamentConfiguration {
    /// Creates a new connector-side tournament configuration.
    ///
    /// * `mode` Series format of the tournament.
    /// * `duration_ticks` Planned duration of one match in server ticks.
    /// * `teams` Configured tournament teams with their participant account ids.
    /// * `winning_team_ids` Ordered winner history of already finished matches. This lets an admin
    ///   continue a partially played series.
    pub fn new(
        mode: TournamentMode,
        duration_ticks: u32,
        teams: Vec<TournamentTeamConfiguration>,
        winning_team_ids: Vec<TeamId>,
    ) -> Self {
        Self {
            mode,
            duration_ticks,
            teams,
            winning_team_ids,
        }
    }

    /// Series format that the server should use for the tournament.
    #[inline]
    pub fn mode(&self) -> TournamentMode {
        self.mode
    }

    /// Planned duration of each tournament match in server ticks.
    #[inline]
    pub fn duration_ticks(&self) -> u32 {
        self.duration_ticks
    }

    /// Tournament teams and their participant account ids in connector-side form.
    #[inline]
    pub fn teams(&self) -> &[TournamentTeamConfiguration] {
        &self.teams
    }

    /// Ordered winner-team ids for already completed matches.
    #[inline]
    pub fn winning_team_ids(&self) -> &[TeamId] {
        &self.winning_team_ids
    }
}
