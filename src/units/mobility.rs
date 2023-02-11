use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Mobility {
    Still,
    Steady,
    Mobile,
}
