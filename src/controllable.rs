use crate::error::GameError;
use crate::network::connection_handle::ConnectionHandle;
use crate::network::query::{QueryCommand, QueryResult};
use crate::team::TeamId;
use crate::units::player_unit::{PlayerUnit, PlayerUnitSystems};
use crate::vector::Vector;
use serde_derive::{Deserialize, Serialize};
use std::future::Future;
use std::sync::Arc;

#[repr(transparent)]
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct ControllableId(pub(crate) usize);

pub struct Controllable {
    pub(crate) connection: Arc<ConnectionHandle>,
    pub name: String,
    pub id: ControllableId,
    pub radius: f64,
    pub position: Vector,
    pub movement: Vector,
    pub direction: f64,
    pub team: Option<TeamId>,
    pub gravity: f64,
    pub energy_output: f64,
    pub alive: bool,
    pub turn_rate: f64,
    pub systems: PlayerUnitSystems,
}

impl Controllable {
    pub fn update(&mut self, unit: PlayerUnit) {
        let _ = unit;
        // self.systems = unit.systems.clone();
    }

    pub async fn r#continue(&self) -> Result<impl Future<Output = QueryResult>, GameError> {
        if self.systems.hull.value > 0.0 {
            Err(GameError::ControllableMustBeDeadToContinue)
        } else {
            Ok(self
                .connection
                .send_query(QueryCommand::ContinueControllable {
                    controllable: self.id,
                })
                .await?)
        }
    }

    pub async fn kill(&self) -> Result<impl Future<Output = QueryResult>, GameError> {
        if self.systems.hull.value <= 0.0 {
            Err(GameError::ControllableMustBeAlive)
        } else {
            Ok(self
                .connection
                .send_query(QueryCommand::KillControllable {
                    controllable: self.id,
                })
                .await?)
        }
    }

    pub async fn set_nozzle(
        &self,
        value: f64,
    ) -> Result<impl Future<Output = QueryResult>, GameError> {
        if self.systems.hull.value <= 0.0 {
            Err(GameError::ControllableMustBeAlive)
        } else if !value.is_finite() {
            Err(GameError::FloatingPointNumberInvalid)
        } else if value.abs() > self.systems.nozzle.value {
            Err(GameError::FloatingPointNumberOutOfRange)
        } else {
            Ok(self
                .connection
                .send_query(QueryCommand::SetControllableNozzle {
                    controllable: self.id,
                    nozzle: value.clamp(-5.0, 5.0),
                })
                .await?)
        }
    }

    pub async fn set_scanner(
        &self,
        direction: f64,
        length: f64,
        width: f64,
        enabled: bool,
    ) -> Result<impl Future<Output = QueryResult>, GameError> {
        if self.systems.hull.value <= 0.0 {
            Err(GameError::ControllableMustBeAlive)
        } else if !direction.is_finite() || !length.is_finite() || !width.is_finite() {
            Err(GameError::FloatingPointNumberInvalid)
        } else {
            let direction = (direction + 3600.0) % 360.0;

            if direction < 0.0 || length > 300.1 || length < 59.9 || width < 19.9 || width > 60.1 {
                return Err(GameError::FloatingPointNumberOutOfRange);
            }

            Ok(self
                .connection
                .send_query(QueryCommand::SetControllableScanner {
                    controllable: self.id,
                    direction,
                    length: length.clamp(60.0, 300.0),
                    width: width.clamp(20.0, 60.0),
                    enabled,
                })
                .await?)
        }
    }
}
