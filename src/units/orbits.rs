use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Orbits {
    pub distance: f64,
    pub angle: f64,
    pub interval: i32,
}
