use crate::units::corona::Corona;
use crate::units::corona_section::CoronaSection;
use crate::units::orbits::Orbit;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sun {
    #[serde(default)]
    pub orbits: Vec<Orbit>,
    pub corona: Option<Corona>,
    #[serde(default)]
    pub sections: Vec<CoronaSection>,
}
