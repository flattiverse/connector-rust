use crate::con::handle::ConnectionHandle;
use crate::con::{Connection, OpenError, UpdateEvent};
use crate::units::uni::{UniverseEvent, UniverseId};
use crate::units::uni_group::UniverseGroup;
use log::error;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use crate::plr::User;

pub struct Connector {
    handle: Arc<ConnectionHandle>,
    updates: mpsc::UnboundedReceiver<UpdateEvent>,
    universe_group: UniverseGroup,
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
            universe_group: {
                let mut group = UniverseGroup::new(Arc::clone(&handle));
                group.add_universe(UniverseId(0));
                group
            },
            handle,
            updates,
        })
    }

    #[inline]
    pub async fn update(&mut self) -> Option<UpdateEvent> {
        loop {
            match self.updates.recv().await? {
                UpdateEvent::ServerEvents(events) => {
                    for event in events.payload {
                        match event {
                            UniverseEvent::UniverseUpdate { universe } => {
                                self.universe_group.add_universe(UniverseId(universe))
                            }
                            UniverseEvent::NewUnit { universe, unit } => {
                                if let Some(universe) =
                                    self.universe_group.get_universe_mut(UniverseId(universe))
                                {
                                    universe.on_new_unit(unit);
                                } else {
                                    error!(
                                        "Received update for unknown universe {:?}",
                                        UniverseId(universe)
                                    );
                                }
                            }
                            UniverseEvent::UpdateUnit { universe, unit } => {
                                if let Some(universe) =
                                    self.universe_group.get_universe_mut(UniverseId(universe))
                                {
                                    universe.on_update_unit(unit);
                                } else {
                                    error!(
                                        "Received update for unknown universe {:?}",
                                        UniverseId(universe)
                                    );
                                }
                            }
                            UniverseEvent::UserUpdate { name } => {
                                self.universe_group.on_add_user(User::new(name));
                            }
                            UniverseEvent::BroadcastMessage { message } => {
                                return Some(UpdateEvent::BroadcastMessage(message));
                            }
                        }
                    }
                }
                event => return Some(event),
            }
        }
    }

    #[inline]
    pub fn universe_group(&self) -> &UniverseGroup {
        &self.universe_group
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
