use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Activation {
    pub probability: f64,
    pub foreshadowing: i32,
    pub upramp: i32,
    pub time: i32,
    pub fade: i32,
}
