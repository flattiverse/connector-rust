use crate::units::player_unit_system_identifier::PlayerUnitSystemIdentifier;
use crate::units::player_unit_system_kind::PlayerUnitSystemKind;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerUnitSystemUpgradePath {
    #[serde(rename = "requiredSystem")]
    pub required_component: Option<PlayerUnitSystemIdentifier>,
    #[serde(rename = "system")]
    pub kind: PlayerUnitSystemKind,
    pub level: u32,
    pub energy: f64,
    pub particles: f64,
    pub iron: f64,
    pub carbon: f64,
    pub silicon: f64,
    pub platinum: f64,
    pub gold: f64,
    pub time: i32,
    pub value0: f64,
    pub value1: f64,
    pub value2: f64,
    #[serde(rename = "areaIncrease")]
    pub area_increase: f64,
    #[serde(rename = "weightIncrease")]
    pub weight_increase: f64,
}
