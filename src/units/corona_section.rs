use crate::units::activation::Activation;
use crate::units::activation_state::ActivationState;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoronaSection {
    #[serde(rename = "angleStart")]
    pub angle_start: f64,
    #[serde(rename = "angleEnd")]
    pub angle_end: f64,
    #[serde(rename = "distanceStart")]
    pub distance_start: f64,
    #[serde(rename = "distanceEnd")]
    pub distance_end: f64,
    pub energy: Option<f64>,
    pub particles: Option<f64>,
    pub activation: Option<Activation>,
    #[serde(rename = "activationState")]
    pub activation_state: Option<ActivationStateFrame>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivationStateFrame {
    pub state: ActivationState,
    pub frame: i32,
}
