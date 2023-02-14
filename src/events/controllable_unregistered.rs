use crate::controllable::ControllableId;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ControllableUnregistered {
    pub universe: usize,
    #[serde(rename = "controllableID")]
    pub controllable: ControllableId,
}
