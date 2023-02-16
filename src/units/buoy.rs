use crate::units::orbits::Orbit;
use crate::vector::Vector;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Buoy {
    #[serde(default)]
    pub orbits: Vec<Orbit>,
    pub message: String,
    #[serde(default)]
    pub hints: Vec<Vector>,
    #[serde(rename = "messageKind")]
    pub kind: MessageKind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageKind {
    #[serde(rename = "normal", alias = "Normal")]
    Normal,
    #[serde(rename = "warning", alias = "Warning")]
    Warning,
    #[serde(rename = "danger", alias = "Danger")]
    Danger,
}
