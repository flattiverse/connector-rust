use crate::controllable::ControllableId;
use crate::units::unit_kind::UnitKind;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeathControllableEvent {
    pub universe: usize,
    #[serde(rename = "controllableID")]
    pub controllable: ControllableId,
    #[serde(rename = "causerKind")]
    pub causer_kind: Option<UnitKind>,
    #[serde(rename = "causerName")]
    pub causer_name: Option<String>,
    #[serde(rename = "reason")]
    pub death_reason: DeathReason,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DeathReason {
    Collision,
    Shelling,
    OutOfBounds,
}
