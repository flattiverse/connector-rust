use crate::controllable::ControllableId;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeathControllableEvent {
    // TODO unwrap, this is just a workaround
    pub universe: Option<usize>,
    #[serde(rename = "controllableID")]
    pub controllable: ControllableId,
    #[serde(rename = "causerKind")]
    pub causer_kind: Option<String>,
    #[serde(rename = "causerName")]
    pub causer_name: Option<String>,
    #[serde(rename = "reason")]
    pub death_reason: DeathReason,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum DeathReason {
    Collision,
    Shelling,
    OutOfBounds,
}
