use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GravityWell {
    pub radius: f64,
    pub force: Option<f64>,
}
