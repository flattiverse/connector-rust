use crate::con::handle::{ConnectionHandle, ConnectionHandleError};
use crate::packet::Command;
use crate::units::uni::UniverseId;
use std::future::Future;
use std::sync::Arc;

pub struct Controllable {
    connection: Arc<ConnectionHandle>,
    universe: UniverseId,
    name: String,
}

impl Controllable {
    // TODO pub(crate)
    pub fn new(connection: Arc<ConnectionHandle>, universe: UniverseId, name: String) -> Self {
        Self {
            connection,
            universe,
            name,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn thrust(
        &self,
        value: f64,
    ) -> Result<
        impl Future<Output = Result<(), ConnectionHandleError>> + 'static,
        ConnectionHandleError,
    > {
        self.connection.send_block_command(Command::Thrust {
            universe: self.universe.0,
            name: self.name.clone(),
            value,
        })
    }
}
