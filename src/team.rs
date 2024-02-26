use crate::hierarchy::TeamConfig;
use crate::network::{ConnectionHandle, PacketReader};
use crate::{GameError, Indexer, NamedUnit};
use std::future::Future;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct TeamId(pub u8);

impl Indexer for TeamId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct Team {
    active: bool,
    id: TeamId,
    config: TeamConfig,
    connection: ConnectionHandle,
}

impl Team {
    #[inline]
    pub fn new(
        id: impl Into<TeamId>,
        connection: ConnectionHandle,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            active: true,
            id: id.into(),
            config: TeamConfig::from(reader),
            connection,
        }
    }

    pub(crate) fn update(&mut self, reader: &mut dyn PacketReader) {
        self.config = TeamConfig::from(reader);
    }

    pub(crate) fn dynamic_update(&mut self, reader: &mut dyn PacketReader) {}

    pub(crate) fn deactivate(&mut self) {
        self.active = false;
    }

    /// Sets the given values for this [`Team`]
    /// See also [`ConnectionHandle::configure_team`]
    #[inline]
    pub async fn configure(
        &self,
        config: &TeamConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.configure_team_split(self.id, config).await
    }

    /// Removes this [`Team`]
    /// See also [`ConnectionHandle::remove_team`]
    #[inline]
    pub async fn remove(&self) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.remove_team_split(self.id).await
    }

    #[inline]
    pub fn id(&self) -> TeamId {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        &&self.config.name
    }

    #[inline]
    pub fn config(&self) -> &TeamConfig {
        &self.config
    }
}

impl NamedUnit for Team {
    #[inline]
    fn name(&self) -> &str {
        Team::name(self)
    }
}
