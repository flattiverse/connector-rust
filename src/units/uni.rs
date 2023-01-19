use crate::con::{Connection, SendError};
use crate::packet::Vector;
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
        unit_json: String,
    ) -> Result<impl Future<Output=()>, SendError> {
        let receiver = connection
            .send_block_command(|payload| {
                Ok(payload
                    .with_command("setunit")
                    .with_parameter(
                        "data",
                        serde_json::to_string(&UnitSetData {
                            universe: self.id.0,
                            unit: unit_json,
                        })?,
                    ))
            })
            .await?;
        Ok(async move {
            let response = receiver.await;
            eprintln!("{response:?}")
        })
    }

    pub async fn delete(
        &self,
        connection: &mut Connection,
        name: impl Into<String>,
    ) -> Result<impl Future<Output=()>, SendError> {
        let receiver = connection
            .send_block_command(|payload| {
                Ok(payload
                    .with_command("DeleteUnit")
                    .with_parameter("name", name.into()))
            })
            .await?;

        Ok(async move {
            let response = receiver.await;
            eprintln!("{response:?}")
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UnitSetData {
    universe: u16,
    unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitData {
    #[serde(rename = "universegroup")]
    universe_group: u16,
    universe: u16,
    name: String,
    position: Vector,
    radius: f64,
    gravity: f64,
    corona: f64,
}
