use crate::atomics::Atomic;
use crate::hierarchy::{ConnectionProvider, Galaxy, TeamConfig};
use crate::network::PacketReader;
use crate::{GameError, Identifiable, Indexer};
use arc_swap::ArcSwap;
use std::ops::Deref;
use std::sync::{Arc, Weak};

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
    galaxy: Weak<Galaxy>,
    active: Atomic<bool>,
    id: TeamId,
    config: ArcSwap<TeamConfig>,
}

impl Team {
    #[inline]
    pub fn new(galaxy: Weak<Galaxy>, id: impl Into<TeamId>, reader: &mut dyn PacketReader) -> Self {
        Self {
            galaxy,
            id: id.into(),
            active: Atomic::from(true),
            config: ArcSwap::from(Arc::new(TeamConfig::from(reader))),
        }
    }

    pub(crate) fn update(&self, reader: &mut dyn PacketReader) {
        self.config.swap(Arc::new(TeamConfig::from(reader)));
    }

    pub(crate) fn dynamic_update(&self, reader: &mut dyn PacketReader) {
        let _ = reader;
    }

    pub(crate) fn deactivate(&self) {
        self.active.store(false);
    }

    /// Sets the given values for this [`Team`]
    /// See also [`ConnectionHandle::configure_team`]
    #[inline]
    pub async fn configure(&self, config: &TeamConfig) -> Result<(), GameError> {
        self.galaxy
            .connection()?
            .configure_team(self.id, config)
            .await
    }

    /// Removes this [`Team`]
    /// See also [`ConnectionHandle::remove_team`]
    #[inline]
    pub async fn remove(&self) -> Result<(), GameError> {
        self.galaxy.connection()?.remove_team(self.id).await
    }

    /// Sends a chat message with a maximum of 512 characters to all players in this [`Team`].
    #[inline]
    pub async fn chat(&mut self, message: impl AsRef<str>) -> Result<(), GameError> {
        self.galaxy.connection()?.chat_team(self.id, message).await
    }

    #[inline]
    pub fn galaxy(&self) -> &Weak<Galaxy> {
        &self.galaxy
    }

    #[inline]
    pub fn id(&self) -> TeamId {
        self.id
    }

    #[inline]
    pub fn config(&self) -> impl Deref<Target = Arc<TeamConfig>> {
        self.config.load()
    }
}

impl Identifiable<TeamId> for Team {
    #[inline]
    fn id(&self) -> TeamId {
        self.id
    }
}
