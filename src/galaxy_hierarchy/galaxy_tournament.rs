use crate::account::Account;
use crate::galaxy_hierarchy::{
    EventSink, Galaxy, TeamId, Tournament, TournamentConfiguration, TournamentMatchResult,
    TournamentMode, TournamentStage, TournamentTeam,
};
use crate::network::PacketReader;
use crate::{FlattiverseEventKind, GameError, GameErrorKind, ProgressState};
use num_enum::FromPrimitive;
use std::sync::Arc;

impl Galaxy {
    /// Current tournament snapshot mirrored from the server, or `None` if no tournament is
    /// configured. The reference changes through [`TournamentCreatedEvent`],
    /// [`TournamentUpdatedEvent`], and [`TournamentRemovedEvent`].
    #[inline]
    pub fn tournament(&self) -> Option<Arc<Tournament>> {
        self.tournament.load_full()
    }

    /// Configures a tournament from a typed connector-side description.
    #[inline]
    pub async fn configure_tournament(
        &self,
        configuration: &TournamentConfiguration,
    ) -> Result<(), GameError> {
        self.connection()
            .galaxy_configure_tournament(configuration)
            .await
    }

    /// Advances the configured tournament from preparation into the commencing stage.
    #[inline]
    pub async fn commence_tournament(&self) -> Result<(), GameError> {
        self.connection().galaxy_commence_tournament().await
    }

    /// Starts a previously commenced tournament so that it enters the running stage.
    #[inline]
    pub async fn start_tournament(&self) -> Result<(), GameError> {
        self.connection().galaxy_start_tournament().await
    }

    /// Removes the currently configured tournament from the galaxy.
    #[inline]
    pub async fn cancel_tournament(&self) -> Result<(), GameError> {
        self.connection().galaxy_cancel_tournament().await
    }

    /// Queries the account list that the server exposes for tournament tooling.
    pub async fn query_accounts(
        self: &Arc<Self>,
        progress_state: Option<Arc<ProgressState>>,
    ) -> Result<Vec<Arc<Account>>, GameError> {
        self.connection()
            .galaxy_query_accounts(self, progress_state)
            .await
    }

    #[instrument(level = "trace", skip(self, reader))]
    pub(crate) fn tournament_upsert(
        self: &Arc<Self>,
        events: &mut EventSink,
        reader: &mut dyn PacketReader,
    ) -> Result<(), GameError> {
        debug!("Upserting touranment");

        let tournament = Arc::new(self.read_tournament(reader)?);
        match self.tournament.load_full() {
            None => {
                self.tournament.store(Some(tournament.clone()));
                event!(events, TournamentCreated { tournament });
            }
            Some(previous) => {
                self.tournament.store(Some(tournament.clone()));
                event!(
                    events,
                    TournamentUpdated {
                        old_tournament: previous,
                        new_tournament: tournament,
                    }
                );
            }
        }

        Ok(())
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn tournament_removed(
        self: &Arc<Self>,
        events: &mut EventSink,
    ) -> Result<(), GameError> {
        match self.tournament.swap(None) {
            None => Err(GameErrorKind::InvalidData {
                message: Some("Server removed a tournament although none exists.".to_string()),
            }
            .into()),
            Some(tournament) => {
                event!(events, TournamentRemoved { tournament });
                Ok(())
            }
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub(crate) fn tournament_message(
        self: &Arc<Self>,
        events: &mut EventSink,
        message: String,
    ) -> Result<(), GameError> {
        event!(events, TournamentMessage { message });
        Ok(())
    }

    fn read_tournament(
        self: &Arc<Self>,
        reader: &mut dyn PacketReader,
    ) -> Result<Tournament, GameError> {
        let stage_value = reader.read_byte();
        let mode_value = reader.read_byte();
        let duration_ticks = reader.read_uint32();
        let team_count = reader.read_byte();

        let stage = TournamentStage::from_primitive(stage_value);
        let mode = TournamentMode::from_primitive(mode_value);

        let teams = (0..team_count)
            .map(|_| {
                let team_id = reader.read_byte();
                let participant_count = reader.read_byte();
                let team = self.get_team(TeamId(team_id));

                let participants = (0..participant_count)
                    .map(|_| Account::try_read(Arc::downgrade(self), reader).map(Arc::new))
                    .collect::<Result<Vec<Arc<Account>>, GameError>>()?;

                Ok(TournamentTeam::new(team, participants, 0))
            })
            .collect::<Result<Vec<TournamentTeam>, GameError>>()?;

        let history_count = reader.read_byte();
        let match_history = (0..history_count)
            .map(|index| {
                let winning_team_id = reader.read_byte();
                let winning_team = self.get_team(TeamId(winning_team_id));

                TournamentMatchResult::new(i32::from(index) + 1, winning_team)
            })
            .collect::<Vec<TournamentMatchResult>>();

        let teams_with_wins = teams
            .iter()
            .map(|team| {
                TournamentTeam::new(
                    team.team().clone(),
                    team.participants().to_vec(),
                    match_history
                        .iter()
                        .filter(|match_result| match_result.winning_team().id() == team.team().id())
                        .count() as i32,
                )
            })
            .collect::<Vec<TournamentTeam>>();

        Ok(Tournament::new(
            stage,
            mode,
            duration_ticks,
            teams_with_wins,
            match_history,
        ))
    }
}
