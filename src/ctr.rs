use crate::con::handle::ConnectionHandle;
use crate::con::{Connection, OpenError, UpdateEvent};
use crate::units::uni::UniverseId;
use crate::units::uni_group::UniverseGroup;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

pub struct Connector {
    handle: Arc<ConnectionHandle>,
    updates: mpsc::UnboundedReceiver<UpdateEvent>,
    universe_groups: Vec<UniverseGroup>,
}

impl Connector {
    pub const PING_INTERVAL: Duration = Duration::from_secs(1);

    #[inline]
    pub async fn new(api_key: impl AsRef<str>) -> Result<Self, ConnectorError> {
        Self::new_to(api_key, Connection::DEFAULT_HOST).await
    }

    pub async fn new_to(
        api_key: impl AsRef<str>,
        host: impl AsRef<str>,
    ) -> Result<Self, ConnectorError> {
        let connection = Connection::connect_to(host.as_ref(), api_key.as_ref()).await?;
        let (handle, updates) = connection.spawn(Self::PING_INTERVAL);
        Ok(Self {
            universe_groups: vec![{
                let mut group = UniverseGroup::new(Arc::clone(&handle));
                group.add_universe(UniverseId(0));
                group
            }],
            handle,
            updates,
        })
    }

    #[inline]
    pub async fn update(&mut self) -> Option<UpdateEvent> {
        self.updates.recv().await
    }

    #[inline]
    pub fn iter_universe_groups(&self) -> impl Iterator<Item=&UniverseGroup> {
        self.universe_groups.iter()
    }

    #[inline]
    pub fn connection_handle(&self) -> &Arc<ConnectionHandle> {
        &self.handle
    }

    #[inline]
    pub fn clone_connection_handle(&self) -> Arc<ConnectionHandle> {
        Arc::clone(self.connection_handle())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConnectorError {
    #[error("Failed to open a connection to the flattiverse: {0}")]
    OpenError(#[from] OpenError),
}
