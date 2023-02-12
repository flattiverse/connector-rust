use crate::units::player_unit_system_kind::PlayerUnitSystemKind;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerUnitSystemIdentifier {
    #[serde(rename = "system")]
    pub kind: PlayerUnitSystemKind,
    pub level: Option<i32>,
}
