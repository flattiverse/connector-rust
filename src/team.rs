use crate::error::GameError;
use crate::network::connection_handle::{ConnectionHandle, SendQueryError};
use crate::network::query::{QueryCommand, QueryResponse};
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::sync::{Arc, Weak};

#[repr(transparent)]
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct TeamId(pub(crate) usize);

#[derive(Serialize, Deserialize, Clone)]
pub struct Team {
    #[serde(skip, default)]
    pub(crate) connection: Weak<ConnectionHandle>,
    /// The id of the team.
    pub id: TeamId,
    /// The name of the team.
    pub name: String,
    /// The red value of the team's color.
    pub r: f64,
    /// The green value of the team's color.
    pub g: f64,
    /// The blue value of the team's color.
    pub b: f64,
}

impl Debug for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Team")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("r", &self.r)
            .field("g", &self.g)
            .field("b", &self.b)
            .finish_non_exhaustive()
    }
}

impl Team {
    fn connection(&self) -> Result<Arc<ConnectionHandle>, GameError> {
        if let Some(connection) = self.connection.upgrade() {
            Ok(connection)
        } else {
            Err(GameError::SendQueryError(SendQueryError::ConnectionGone))
        }
    }

    /// The team's color in a three-dimensional color array (RGB)
    pub fn rgb(&self) -> [f64; 3] {
        [self.r, self.g, self.b]
    }

    pub fn chat(
        &self,
        message: impl Into<String>,
    ) -> impl Future<Output = Result<QueryResponse, GameError>> + 'static {
        let connection = self.connection();
        let message = GameError::checked_message(message.into());
        let team = self.id;
        async move {
            Ok(connection?
                .send_query(QueryCommand::ChatTeam {
                    team,
                    message: message?,
                })
                .await?
                .await?)
        }
    }
}
