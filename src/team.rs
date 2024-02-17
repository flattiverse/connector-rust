use crate::hierarchy::TeamConfig;
use crate::network::{ConnectionHandle, PacketReader};
use crate::{GameError, Indexer, NamedUnit};
use std::future::Future;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, derive_more::From)]
pub struct TeamId(pub(crate) u8);

impl Indexer for TeamId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct Team {
    id: TeamId,
    name: String,
    red: u8,
    green: u8,
    blue: u8,
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
            id: id.into(),
            name: reader.read_string(),
            red: reader.read_byte(),
            green: reader.read_byte(),
            blue: reader.read_byte(),
            connection,
        }
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
        &&self.name
    }

    #[inline]
    pub fn red(&self) -> u8 {
        self.red
    }

    #[inline]
    pub fn green(&self) -> u8 {
        self.green
    }

    #[inline]
    pub fn blue(&self) -> u8 {
        self.blue
    }
}

impl NamedUnit for Team {
    #[inline]
    fn name(&self) -> &str {
        Team::name(self)
    }
}
