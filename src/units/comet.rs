use crate::units::orbits::Orbit;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comet {
    #[serde(default)]
    pub orbits: Vec<Orbit>,
}
