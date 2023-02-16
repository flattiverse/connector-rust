use crate::units::unit::Unit;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatedUnitEvent {
    pub unit: Unit,
}
