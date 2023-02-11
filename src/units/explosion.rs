use crate::units::orbits::Orbit;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Explosion {
    #[serde(default)]
    pub orbits: Vec<Orbit>,
    #[serde(default)]
    pub damage: f64,
}
