use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GravityWell {
    pub radius: f64,
    pub force: Option<f64>,
}
