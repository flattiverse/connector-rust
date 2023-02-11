use crate::units::corona::Corona;
use crate::units::corona_section::CoronaSection;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sun {
    pub corona: Option<Corona>,
    #[serde(default)]
    pub sections: Vec<CoronaSection>,
}
