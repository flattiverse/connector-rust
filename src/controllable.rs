use crate::error::GameError;
use crate::network::connection_handle::ConnectionHandle;
use crate::network::query::{QueryCommand, QueryResult};
use crate::team::TeamId;
use crate::units::player_unit::{PlayerUnit, PlayerUnitSystems};
use crate::vector::Vector;
use serde_derive::{Deserialize, Serialize};
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

#[repr(transparent)]
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct ControllableId(pub(crate) usize);

pub struct Controllable {
    pub(crate) connection: Arc<ConnectionHandle>,
    /// The name of your controllable.
    pub name: String,
    /// The id of your controllable.
    pub id: ControllableId,
    /// THe radius of your controllable.
    pub radius: f64,
    /// The position of your controllable.
    pub position: Vector,
    /// The movement of your controllable.
    pub movement: Vector,
    /// The direction of your controllable.
    pub direction: f64,
    /// If you have joined a team, the team of your controllable.
    pub team: Option<TeamId>,
    /// The gravity that your controllable is exercising on the other units.
    pub gravity: f64,
    /// The current energy output of your controllable.
    pub energy_output: f64,
    /// Whether your controllable is still alive.
    pub alive: bool,
    /// The rate at which your controllable is turning.
    pub turn_rate: f64,
    pub systems: Arc<Mutex<PlayerUnitSystems>>,
}

impl Controllable {
    pub(crate) async fn die(&self) {
        self.systems.lock().await.hull.value = 0.0;
    }

    pub(crate) async fn update(&self, unit: &PlayerUnit) {
        *self.systems.lock().await = unit.systems.clone();
    }

    pub async fn r#continue(&self) -> Result<impl Future<Output = QueryResult>, GameError> {
        if self.systems.lock().await.hull.value > 0.0 {
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
        if self.systems.lock().await.hull.value <= 0.0 {
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
        if self.systems.lock().await.hull.value <= 0.0 {
            Err(GameError::ControllableMustBeAlive)
        } else if !value.is_finite() {
            Err(GameError::FloatingPointNumberInvalid)
        } else if value.abs() > self.systems.lock().await.nozzle.value {
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
        if self.systems.lock().await.hull.value <= 0.0 {
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
