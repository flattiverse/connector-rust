use crate::con::{Connection, SendError};
use crate::packet::{Command, Vector};
use serde_derive::{Deserialize, Serialize};
use std::future::Future;

#[derive(Debug, PartialOrd, PartialEq, Hash, Ord, Eq, Copy, Clone)]
pub struct UniverseId(pub u16);

pub struct Universe {
    id: UniverseId,
}

impl Universe {
    pub fn new(id: UniverseId) -> Self {
        Self { id }
    }

    pub async fn set_unit(
        &self,
        connection: &mut Connection,
        unit: UnitData,
    ) -> Result<impl Future<Output=()>, SendError> {
        let receiver = connection
            .send_block_command(UnitSetData {
                universe: self.id.0,
                unit,
            })
            .await?;
        Ok(async move {
            let response = receiver.await;
            eprintln!("SET UNIT RESPONSE: {response:?}")
        })
    }

    pub async fn delete_unit(
        &self,
        connection: &mut Connection,
        name: impl Into<String>,
    ) -> Result<impl Future<Output=()>, SendError> {
        let receiver = connection
            .send_block_command(Command::DeleteUnit {
                universe: self.id.0,
                name: name.into(),
            })
            .await?;

        Ok(async move {
            let response = receiver.await;
            eprintln!("DELETE UNIT RESPONSE: {response:?}")
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitSetData {
    universe: u16,
    unit: UnitData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitData {
    pub name: String,
    pub position: Vector,
    pub radius: f64,
    pub gravity: f64,
    #[serde(flatten)]
    pub extension: UnitExtension,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum UnitExtension {
    #[serde(rename = "Sun")]
    Sun { corona: f64 },
}
