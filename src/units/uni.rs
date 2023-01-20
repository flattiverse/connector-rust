use crate::con::handle::{ConnectionHandle, ConnectionHandleError};
use crate::packet::{Command, Vector};
use serde_derive::{Deserialize, Serialize};
use std::future::Future;
use std::sync::Arc;

#[derive(Debug, PartialOrd, PartialEq, Hash, Ord, Eq, Copy, Clone)]
pub struct UniverseId(pub u16);

pub struct Universe {
    connection: Arc<ConnectionHandle>,
    id: UniverseId,
}

impl Universe {
    pub(crate) fn new(id: UniverseId, ch: Arc<ConnectionHandle>) -> Self {
        Self { id, connection: ch }
    }

    #[inline]
    pub fn set_unit(
        &self,
        unit: UnitData,
    ) -> Result<impl Future<Output=Result<(), ConnectionHandleError>>, ConnectionHandleError>
    {
        self.connection.send_block_command_mapped(UnitSetData {
            universe: self.id.0,
            unit,
        })
    }

    #[inline]
    pub fn delete_unit(
        &self,
        name: impl Into<String>,
    ) -> Result<impl Future<Output=Result<(), ConnectionHandleError>>, ConnectionHandleError>
    {
        self.connection
            .send_block_command_mapped(Command::DeleteUnit {
                universe: self.id.0,
                name: name.into(),
            })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitSetData {
    universe: u16,
    unit: UnitData,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct UnitData {
    pub name: String,
    pub position: Vector,
    pub radius: f64,
    pub gravity: f64,
    #[serde(flatten)]
    pub extension: UnitExtension,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::AsRefStr)]
#[serde(tag = "kind")]
pub enum UnitExtension {
    #[serde(rename = "Sun")]
    Sun { corona: f64 },
}

impl Default for UnitExtension {
    fn default() -> Self {
        Self::Sun {
            corona: f64::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum UniverseEvent {
    #[serde(rename = "newUnit")]
    NewUnit { universe: u16, unit: UnitData },
    #[serde(rename = "message")]
    BroadcastMessage { message: BroadcastMessage },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastMessage {
    sender: String,
    timestamp: String,
    text: String,
}
