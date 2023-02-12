use crate::players::PlayerId;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerUnit {
    pub player: PlayerId,
    // pub controllable: ControllableId,
    #[serde(rename = "turnRate")]
    pub turn_rate: f64,
    // #[serde(default)]
    // pub systems: PlayerUnitSystems,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerUnitSystems {
    // #[serde(rename = "Hull")]
    // pub hull: PlayerUnitSystem,
}
