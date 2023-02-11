use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Corona {
    pub radius: f64,
    pub energy: Option<f64>,
    pub particles: Option<f64>,
}
