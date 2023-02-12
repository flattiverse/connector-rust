use crate::units::gravity_well::GravityWell;
use crate::units::gravity_well_section::GravityWellSection;
use crate::units::orbits::Orbit;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blackhole {
    #[serde(default)]
    pub orbits: Vec<Orbit>,
    pub gravity_well: Option<GravityWell>,
    #[serde(default)]
    pub sections: Vec<GravityWellSection>,
}
