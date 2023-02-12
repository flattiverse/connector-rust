use crate::units::player_unit_system_kind::PlayerUnitSystemKind;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct PlayerUnitSystemIdentifier {
    pub system: PlayerUnitSystemKind,
    pub level: Option<u32>,
}
