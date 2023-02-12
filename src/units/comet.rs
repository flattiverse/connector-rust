use crate::units::orbits::Orbit;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Comet {
    #[serde(default)]
    pub orbits: Vec<Orbit>,
}