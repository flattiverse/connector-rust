use crate::error::GameError;
use crate::network::connection_handle::{ConnectionHandle, SendQueryError};
use crate::network::query::{QueryCommand, QueryError, QueryResult};
use crate::region::{GameRegion, GameRegionId};
use crate::team::TeamId;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::ops::Deref;
use std::sync::Weak;

#[repr(transparent)]
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct UniverseId(pub(crate) usize);

#[derive(Serialize, Deserialize, Clone)]
pub struct Universe {
    #[serde(skip, default)]
    pub(crate) connection: Weak<ConnectionHandle>,
    pub id: UniverseId,
    pub name: String,
}

impl Debug for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Universe")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl Universe {
    fn connection(&self) -> Result<impl Deref<Target = ConnectionHandle>, GameError> {
        if let Some(connection) = self.connection.upgrade() {
            Ok(connection)
        } else {
            Err(GameError::SendQueryError(SendQueryError::ConnectionGone))
        }
    }

    /// Creates or updates a unit in this [`Universe`]. The unit will be overwritten if the it
    /// already exists (same name) and the colliding unit isn't a non editable unit like a player
    /// unit, shot or explosion, ...
    ///
    /// The definition is expected to be a well formatted JSON definition of a [`Unit`].
    ///
    /// # Access Restricted
    ///
    /// This is only accessible if you are an administrator.
    pub async fn set_unit(
        &self,
        definition: impl Into<String>,
    ) -> Result<impl Future<Output = QueryResult>, GameError> {
        let definition = definition.into();
        if definition.is_empty() {
            Err(GameError::UnitDefinitionEmpty)
        } else if definition.len() > 2048 {
            Err(GameError::UnitDefinitionTooLong)
        } else {
            Ok(self
                .connection()?
                .send_query(QueryCommand::SetUnit {
                    universe: self.id,
                    unit: definition,
                })
                .await?)
        }
    }

    /// Removes a unit from this [`Universe`].
    ///
    /// # Access Restricted
    ///
    /// This is only accessible if you are an administrator.
    pub async fn remove_unit(
        &self,
        name: impl Into<String>,
    ) -> Result<impl Future<Output = QueryResult>, GameError> {
        let name = GameError::checked_name(name.into())?;
        Ok(self
            .connection()?
            .send_query(QueryCommand::RemoveUnit {
                universe: self.id,
                unit: name,
            })
            .await?)
    }

    /// Queries the server for the [`Region`] definitions
    pub async fn query_regions(
        &self,
    ) -> Result<impl Future<Output = Result<Vec<GameRegion>, QueryError>>, GameError> {
        let query = self
            .connection()?
            .send_query(QueryCommand::ListRegion { universe: self.id })
            .await?;

        Ok(async move {
            let response = query.await?;
            if let Some(str) = response.get_str() {
                Ok(serde_json::from_str::<Vec<GameRegion>>(str)?)
            } else {
                Ok(Vec::default())
            }
        })
    }

    /// Creates or updates a region in this [`Universe`]. The region will be overwritten if it
    /// already exists (same id).
    ///
    /// # Access Restricted
    ///
    /// This is only accessible if you are an administrator.
    #[allow(clippy::too_many_arguments)]
    pub async fn set_region(
        &self,
        id: GameRegionId,
        team: TeamId,
        name: impl Into<Option<String>>,
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
        start_location: bool,
        safe_zone: bool,
        slow_restore: bool,
    ) -> Result<impl Future<Output = QueryResult>, GameError> {
        let connection = self.connection()?;
        if let Some(name) = name.into() {
            Ok(connection
                .send_query(QueryCommand::SetRegion {
                    universe: self.id,
                    region: id,
                    teams: team.0 as _,
                    name,
                    left,
                    top,
                    right,
                    bottom,
                    start_location,
                    safe_zone,
                    slow_restore,
                })
                .await?)
        } else {
            Ok(connection
                .send_query(QueryCommand::SetRegionUnnamed {
                    universe: self.id,
                    region: id,
                    teams: team.0 as _,
                    left,
                    top,
                    right,
                    bottom,
                    start_location,
                    safe_zone,
                    slow_restore,
                })
                .await?)
        }
    }

    /// Removes the region from this [`Universe`].
    ///
    /// # Access Restricted
    ///
    /// This is only accessible if you are an administrator.
    pub async fn remove_region(
        &self,
        id: GameRegionId,
    ) -> Result<impl Future<Output = QueryResult>, GameError> {
        Ok(self
            .connection()?
            .send_query(QueryCommand::RemoveRegion {
                universe: self.id,
                region: id,
            })
            .await?)
    }

    /// Retrieves the raw JSON definition for a unit.
    ///
    /// # Access Restricted
    ///
    /// This is only accessible if you are an administrator.
    pub async fn query_unit_map_edit_json(
        &self,
        name: String,
    ) -> Result<impl Future<Output = QueryResult>, GameError> {
        let name = GameError::checked_name(name)?;
        Ok(self
            .connection()?
            .send_query(QueryCommand::GetUnit {
                universe: self.id,
                unit: name,
            })
            .await?)
    }
}
