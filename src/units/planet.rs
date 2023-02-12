use crate::units::orbits::Orbit;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Planet {
    #[serde(default)]
    pub orbits: Vec<Orbit>,
}
