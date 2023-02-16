use crate::team::TeamId;
use crate::units::orbits::Orbit;
use crate::vector::Vector;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MissionTarget {
    #[serde(default)]
    pub orbits: Vec<Orbit>,
    pub sequence: Option<i32>,
    #[serde(rename = "dominationRadius")]
    pub domination_radius: Option<f64>,
    #[serde(rename = "dominationProgress")]
    pub domination_progress: Option<f64>,
    #[serde(rename = "dominationTeam")]
    pub domination_team: Option<TeamId>,
    #[serde(default)]
    pub hints: Vec<Vector>,
}
