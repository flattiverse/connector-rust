use crate::units::orbits::Orbit;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Shot {
    #[serde(default)]
    pub orbits: Vec<Orbit>,
    #[serde(rename = "explosionDamage")]
    pub explosion_damage: f64,
    #[serde(rename = "explosionRadius")]
    pub explosion_radius: f64,
    #[serde(rename = "lifetime")]
    pub life_time: i32,
}
