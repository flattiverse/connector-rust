use crate::con::{Connection, SendError};
use crate::packet::{Packet, Vector};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;

#[derive(Default)]
pub struct Unit {}

impl Unit {
    pub async fn create(
        &self,
        connection: &mut Connection,
    ) -> Result<impl Future<Output=()>, SendError> {
        let (id, receiver) = connection.block_manager.next_block();
        let packet = Packet {
            id,
            command: "CreateUnit".to_string(),
            parameters: {
                let mut map = HashMap::default();
                map.insert(
                    "data".to_string(),
                    serde_json::Value::String(serde_json::to_string(&UnitCreateData {
                        universe_group: 0,
                        universe: 0,
                        name: "".to_string(),
                        kind: UnitKind::Sun,
                        position: Vector { x: 0.0, y: 0.0 },
                        radius: 0.0,
                        corona: 0.0,
                        gravity: 0.0,
                    })?),
                );
                map
            },
        };
        connection.send_packet(&packet).await?;
        Ok(async move {
            let response = receiver.await;
            eprintln!("{response:?}")
        })
    }

    pub async fn change(
        &self,
        connection: &mut Connection,
    ) -> Result<impl Future<Output=()>, SendError> {
        let (id, receiver) = connection.block_manager.next_block();
        let packet = Packet {
            id,
            command: "ChangeUnit".to_string(),
            parameters: {
                let mut map = HashMap::default();
                map.insert(
                    "data".to_string(),
                    serde_json::Value::String(serde_json::to_string(&UnitUpdateData {
                        universe_group: 0,
                        universe: 0,
                        name: "".to_string(),
                        kind: UnitKind::Sun,
                        position: Vector { x: 0.0, y: 0.0 },
                        radius: 0.0,
                        corona: 0.0,
                        gravity: 0.0,
                    })?),
                );
                map
            },
        };
        connection.send_packet(&packet).await?;
        Ok(async move {
            let response = receiver.await;
            eprintln!("{response:?}")
        })
    }

    pub async fn delete(
        &self,
        connection: &mut Connection,
    ) -> Result<impl Future<Output=()>, SendError> {
        let (id, receiver) = connection.block_manager.next_block();
        let packet = Packet {
            id,
            command: "DeleteUnit".to_string(),
            parameters: {
                let mut map = HashMap::default();
                map.insert(
                    "data".to_string(),
                    serde_json::Value::String(serde_json::to_string(&UnitDeleteData {
                        universe_group: 0,
                        universe: 0,
                        name: "".to_string(),
                    })?),
                );
                map
            },
        };
        connection.send_packet(&packet).await?;
        Ok(async move {
            let response = receiver.await;
            eprintln!("{response:?}")
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitCreateData {
    #[serde(rename = "universegroup")]
    universe_group: u16,
    universe: u16,
    name: String,
    kind: UnitKind,
    position: Vector,
    radius: f64,
    gravity: f64,
    corona: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitUpdateData {
    #[serde(rename = "universegroup")]
    universe_group: u16,
    universe: u16,
    name: String,
    kind: UnitKind,
    position: Vector,
    radius: f64,
    gravity: f64,
    corona: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitDeleteData {
    #[serde(rename = "universegroup")]
    universe_group: u16,
    universe: u16,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UnitKind {
    Sun
}

