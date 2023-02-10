use crate::con::handle::ConnectionHandle;
use crate::con::{Connection, OpenError, UpdateEvent};
use crate::plr::User;
use crate::units::uni::{UniverseEvent, UniverseId};
use crate::units::uni_group::UniverseGroup;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;

pub struct Connector {
    handle: Arc<ConnectionHandle>,
    updates: mpsc::UnboundedReceiver<UpdateEvent>,
    universe_group: UniverseGroup,
}

impl Connector {
    pub const PING_INTERVAL: Duration = Duration::from_secs(1);

    #[inline]
    pub async fn new(api_key: impl AsRef<str>) -> Result<Self, ConnectorError> {
        Self::new_to(api_key, Connection::DEFAULT_URL).await
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
                group.on_add_universe(UniverseId(0));
                group
            },
            handle,
            updates,
        })
    }

    #[inline]
    pub async fn poll_next_update(&mut self) -> Option<Result<Vec<UpdateEvent>, UpdateError>> {
        match self.updates.try_recv() {
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => Some(Err(UpdateError::ConnectionClosed)),
            Ok(update) => match update {
                UpdateEvent::ServerEvents(events) => Some(Ok(events
                    .payload
                    .into_iter()
                    .flat_map(|event| {
                        match event {
                            UniverseEvent::UniverseInfo { universe } => {
                                self.universe_group.on_add_universe(UniverseId(universe));
                            }
                            UniverseEvent::NewUser { name } => {
                                self.universe_group.on_add_user(User::new(name));
                            }
                            UniverseEvent::TickCompleted => {
                                return Some(UpdateEvent::TickCompleted { tick: events.tick });
                            }
                            UniverseEvent::NewUnit { universe, unit } => {
                                if let Some(universe) =
                                    self.universe_group.get_universe_mut(UniverseId(universe))
                                {
                                    universe.on_new_unit(unit);
                                } else {
                                    error!(
                                        "Received NewUnit for unknown universe {:?}",
                                        UniverseId(universe)
                                    );
                                }
                            }
                            UniverseEvent::RemoveUnit { universe, name } => {
                                if let Some(universe) =
                                    self.universe_group.get_universe_mut(UniverseId(universe))
                                {
                                    universe.on_remove_unit(&name);
                                } else {
                                    error!(
                                        "Received RemoveUnit for unknown universe {:?}",
                                        UniverseId(universe)
                                    );
                                }
                            }
                            UniverseEvent::BroadcastMessage { message } => {
                                return Some(UpdateEvent::BroadcastMessage(message));
                            }
                            UniverseEvent::UniverseUpdate { universe } => {
                                self.universe_group.on_add_universe(UniverseId(universe))
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
                        }
                        None
                    })
                    .collect())),
                event => Some(Ok(vec![event])),
            },
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

#[derive(Debug, thiserror::Error)]
pub enum UpdateError {
    #[error("The connection has been closed")]
    ConnectionClosed,
}
