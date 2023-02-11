use crate::units::orbits::Orbits;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Moon {
    #[serde(default)]
    pub orbits: Vec<Orbits>,
}
