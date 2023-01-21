use crate::con::handle::{ConnectionHandle, ConnectionHandleError};
use crate::packet::{Command, Vector};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

#[derive(Debug, PartialOrd, PartialEq, Hash, Ord, Eq, Copy, Clone)]
pub struct UniverseId(pub u16);

pub struct Universe {
    connection: Arc<ConnectionHandle>,
    id: UniverseId,
    units: HashMap<String, UnitData>,
}

impl Universe {
    pub(crate) fn new(id: UniverseId, ch: Arc<ConnectionHandle>) -> Self {
        Self {
            id,
            connection: ch,
            units: HashMap::default(),
        }
    }

    pub fn id(&self) -> UniverseId {
        self.id
    }

    #[inline]
    pub fn set_unit(
        &self,
        unit: UnitData,
    ) -> Result<impl Future<Output = Result<(), ConnectionHandleError>>, ConnectionHandleError>
    {
        self.connection.send_block_command(UnitSetData {
            universe: self.id.0,
            unit,
        })
    }

    #[inline]
    pub fn delete_unit(
        &self,
        name: impl Into<String>,
    ) -> Result<impl Future<Output = Result<(), ConnectionHandleError>>, ConnectionHandleError>
    {
        self.connection.send_block_command(Command::DeleteUnit {
            universe: self.id.0,
            name: name.into(),
        })
    }

    #[inline]
    pub fn register_ship(
        &self,
        player_ship: UnitData,
    ) -> Result<impl Future<Output = Result<(), ConnectionHandleError>>, ConnectionHandleError>
    {
        if !matches!(player_ship.extension, UnitExtension::PlayerShip { .. }) {
            return Err(ConnectionHandleError::UnitCannotBeRegisteredAsShip(
                player_ship,
            ));
        }
        self.connection.send_block_command(Command::RegisterShip {
            universe: self.id.0,
            unit: player_ship,
        })
    }

    #[inline]
    pub(crate) fn on_new_unit(&mut self, unit: UnitData) {
        self.units.insert(unit.name.clone(), unit);
    }

    #[inline]
    pub(crate) fn on_update_unit(&mut self, unit: UnitData) {
        self.on_new_unit(unit);
    }

    #[inline]
    pub(crate) fn on_remove_unit(&mut self, name: &str) {
        self.units.remove(name);
    }

    #[inline]
    pub fn get_unit(&self, name: impl AsRef<str>) -> Option<&UnitData> {
        self.units.get(name.as_ref())
    }

    #[inline]
    pub fn iter_units(&self) -> impl Iterator<Item = &UnitData> {
        self.units.values()
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
    #[serde(rename = "sun")]
    Sun { corona: f64 },
    #[serde(rename = "playership")]
    PlayerShip {},
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
    #[serde(rename = "universeInfo")]
    UniverseInfo { universe: u16 },
    #[serde(rename = "newUser")]
    NewUser { name: String },
    #[serde(rename = "tickCompleted")]
    TickCompleted,
    #[serde(rename = "newUnit")]
    NewUnit { universe: u16, unit: UnitData },
    #[serde(rename = "removeUnit")]
    RemoveUnit { universe: u16, name: String },
    #[serde(rename = "updateUnit")]
    UpdateUnit { universe: u16, unit: UnitData },
    #[serde(rename = "broadcast")]
    BroadcastMessage { message: BroadcastMessage },
    //
    //
    //
    #[serde(rename = "universeUpdate")]
    UniverseUpdate { universe: u16 },
    #[serde(rename = "userUpdate")]
    UserUpdate { name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastMessage {
    sender: String,
    timestamp: String,
    text: String,
}
