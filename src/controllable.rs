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
        if self.systems.hull.value.unwrap_or_default() > 0.0 {
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
        if self.systems.hull.value.unwrap_or_default() <= 0.0 {
            Err(GameError::ControlalbleMustLiveToBeKilled)
        } else {
            Ok(self
                .connection
                .send_query(QueryCommand::KillControllable {
                    controllable: self.id,
                })
                .await?)
        }
    }
}
