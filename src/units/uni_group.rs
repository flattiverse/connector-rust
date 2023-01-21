use crate::con::handle::{ConnectionHandle, ConnectionHandleError};
use crate::packet::{Command, Message, MessageKind};
use crate::plr::User;
use crate::units::uni::{Universe, UniverseId};
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

pub struct UniverseGroup {
    connection: Arc<ConnectionHandle>,
    universes: HashMap<u16, Universe, nohash_hasher::BuildNoHashHasher<u16>>,
    users: HashMap<String, User>,
}

impl UniverseGroup {
    pub fn new(connection: Arc<ConnectionHandle>) -> Self {
        Self {
            connection,
            universes: HashMap::default(),
            users: HashMap::default(),
        }
    }

    #[inline]
    pub(crate) fn on_add_universe(&mut self, id: UniverseId) {
        self.universes
            .insert(id.0, Universe::new(id, Arc::clone(&self.connection)));
    }

    #[inline]
    pub(crate) fn on_add_user(&mut self, user: User) {
        self.users.insert(user.name().to_string(), user);
    }

    #[inline]
    pub fn send_broadcast_message(
        &self,
        message: impl Into<String>,
    ) -> Result<
        impl Future<Output = Result<(), ConnectionHandleError>> + 'static,
        ConnectionHandleError,
    > {
        self.connection.send_block_command(Command::Message {
            kind: MessageKind::Broadcast,
            message: Message::from(message.into()),
        })
    }

    #[inline]
    pub fn create_universe(
        &self,
        name: impl Into<String>,
        x_bounds: f64,
        y_bounds: f64,
    ) -> Result<
        impl Future<Output = Result<(), ConnectionHandleError>> + 'static,
        ConnectionHandleError,
    > {
        self.connection.send_block_command(Command::CreateUniverse {
            name: name.into(),
            x_bounds,
            y_bounds,
        })
    }

    #[inline]
    pub fn iter_universes(&self) -> impl Iterator<Item = &Universe> {
        self.universes.values()
    }

    #[inline]
    pub fn get_universe(&self, id: UniverseId) -> Option<&Universe> {
        self.universes.get(&id.0)
    }

    #[inline]
    pub(crate) fn get_universe_mut(&mut self, id: UniverseId) -> Option<&mut Universe> {
        self.universes.get_mut(&id.0)
    }

    #[inline]
    pub fn iter_users(&self) -> impl Iterator<Item = &User> {
        self.users.values()
    }

    #[inline]
    pub fn get_user(&self, name: &str) -> Option<&User> {
        self.users.get(name)
    }
}
