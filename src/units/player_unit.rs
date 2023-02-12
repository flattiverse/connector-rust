use crate::players::PlayerId;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerUnit {
    pub player: PlayerId,
    // pub controllable: ControllableId,
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
    // #[serde(default)]
    // pub systems: PlayerUnitSystems,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerUnitSystems {
    // #[serde(rename = "Hull")]
    // pub hull: PlayerUnitSystem,
}
