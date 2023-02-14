use crate::error::GameError;
use crate::network::connection_handle::ConnectionHandle;
use crate::network::query::{QueryCommand, QueryResponse};
use crate::team::TeamId;
use crate::units::player_unit::PlayerUnitSystems;
use crate::vector::Vector;
use serde_derive::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

#[repr(transparent)]
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct ControllableId(pub(crate) usize);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ControllableState {
    /// The movement of your controllable.
    pub movement: Vector,
    /// The position of your controllable.
    pub position: Vector,
    /// THe radius of your controllable.
    pub radius: f64,
    /// The gravity that your controllable is exercising on the other units.
    pub gravity: f64,
    /// The current energy output of your controllable.
    #[serde(rename = "energyOutput")]
    pub energy_output: f64,
    /// The rate at which your controllable is turning.
    #[serde(rename = "turnRate")]
    pub turn_rate: f64,
    #[serde(rename = "requestedScanDirection")]
    pub requested_scan_direction: f64,
    #[serde(rename = "requestedScanWidth")]
    pub requested_scan_width: f64,
    #[serde(rename = "requestedScanRange")]
    pub requested_scan_range: f64,
    #[serde(rename = "scanDirection")]
    pub scan_direction: f64,
    #[serde(rename = "scanWidth")]
    pub scan_width: f64,
    #[serde(rename = "scanRange")]
    pub scan_range: f64,
    pub systems: PlayerUnitSystems,
}

pub struct Controllable {
    pub(crate) connection: Arc<ConnectionHandle>,
    /// The name of your controllable.
    pub name: String,
    /// The id of your controllable.
    pub id: ControllableId,
    /// The direction of your controllable.
    pub direction: f64,
    /// If you have joined a team, the team of your controllable.
    pub team: Option<TeamId>,
    pub active: AtomicBool,
    pub state: Mutex<ControllableState>,
}

impl Controllable {
    pub async fn is_alive(&self) -> bool {
        self.state.lock().await.systems.hull.value > 0.0
    }

    pub(crate) async fn die(&self) {
        self.active.store(false, Ordering::Relaxed);
        self.state.lock().await.systems.hull.value = 0.0;
    }

    pub(crate) async fn update_state(&self, state: ControllableState) {
        *self.state.lock().await = state;
    }

    pub async fn r#continue(&self) -> Result<QueryResponse, GameError> {
        if self.state.lock().await.systems.hull.value > 0.0 {
            Err(GameError::ControllableMustBeDeadToContinue)
        } else {
            Ok(self
                .connection
                .send_query(QueryCommand::ContinueControllable {
                    controllable: self.id,
                })
                .await?
                .await?)
        }
    }

    pub async fn kill(&self) -> Result<QueryResponse, GameError> {
        if !self.is_alive().await {
            Err(GameError::ControllableMustBeAlive)
        } else {
            Ok(self
                .connection
                .send_query(QueryCommand::KillControllable {
                    controllable: self.id,
                })
                .await?
                .await?)
        }
    }

    pub async fn set_nozzle(&self, value: f64) -> Result<QueryResponse, GameError> {
        if !self.is_alive().await {
            Err(GameError::ControllableMustBeAlive)
        } else if !value.is_finite() {
            Err(GameError::FloatingPointNumberInvalid)
        } else {
            let max_nozzle = self
                .state
                .lock()
                .await
                .systems
                .nozzle
                .specialization
                .max_value;

            if value.abs() > max_nozzle * 1.05 {
                Err(GameError::FloatingPointNumberOutOfRange)
            } else {
                Ok(self
                    .connection
                    .send_query(QueryCommand::SetControllableNozzle {
                        controllable: self.id,
                        nozzle: {
                            let max_value = max_nozzle;
                            value.clamp(-max_value, max_value)
                        },
                    })
                    .await?
                    .await?)
            }
        }
    }

    pub async fn set_thruster(&self, value: f64) -> Result<QueryResponse, GameError> {
        if !self.is_alive().await {
            Err(GameError::ControllableMustBeAlive)
        } else if !value.is_finite() {
            Err(GameError::FloatingPointNumberInvalid)
        } else if value < 0.0
            || value > {
                self.state
                    .lock()
                    .await
                    .systems
                    .thruster
                    .specialization
                    .max_value
            } * 1.05
        {
            Err(GameError::FloatingPointNumberOutOfRange)
        } else {
            Ok(self
                .connection
                .send_query(QueryCommand::SetControllableThruster {
                    controllable: self.id,
                    thrust: value,
                })
                .await?
                .await?)
        }
    }

    pub async fn set_scanner(
        &self,
        direction: f64,
        length: f64,
        width: f64,
        enabled: bool,
    ) -> Result<QueryResponse, GameError> {
        if !self.is_alive().await {
            Err(GameError::ControllableMustBeAlive)
        } else if !direction.is_finite() || !length.is_finite() || !width.is_finite() {
            Err(GameError::FloatingPointNumberInvalid)
        } else {
            let direction = (direction + 3600.0) % 360.0;
            let (max_length, max_angle) = {
                let lock = self.state.lock().await;
                (
                    lock.systems.scanner.specialization.max_range,
                    lock.systems.scanner.specialization.max_angle,
                )
            };

            if direction < 0.0
                || !(59.9..360.1).contains(&length)
                || length > max_length * 1.05
                || width < 19.9
                || width > max_angle * 1.05
            {
                return Err(GameError::FloatingPointNumberOutOfRange);
            }

            Ok(self
                .connection
                .send_query(QueryCommand::SetControllableScanner {
                    controllable: self.id,
                    direction,
                    length: length.clamp(60.0, max_length),
                    width: width.clamp(20.0, max_angle),
                    enabled,
                })
                .await?
                .await?)
        }
    }
}
