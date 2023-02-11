use crate::units::corona::Corona;
use crate::units::corona_section::CoronaSection;
use crate::units::orbits::Orbits;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sun {
    #[serde(default)]
    pub orbits: Vec<Orbits>,
    pub corona: Option<Corona>,
    #[serde(default)]
    pub sections: Vec<CoronaSection>,
}
