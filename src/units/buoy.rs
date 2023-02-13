use crate::units::orbits::Orbit;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Buoy {
    #[serde(default)]
    pub orbits: Vec<Orbit>,
    pub message: String,
}
