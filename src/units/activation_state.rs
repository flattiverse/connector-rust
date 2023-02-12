use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ActivationState {
    Inactive,
    Foreshadowing,
    Rampup,
    Active,
    Fade,
}
