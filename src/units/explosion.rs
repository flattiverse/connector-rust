use crate::units::orbits::Orbits;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Explosion {
    #[serde(default)]
    pub orbits: Vec<Orbits>,
    #[serde(default)]
    pub damage: f64,
}
