use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum ActivationState {
    Inactive,
    Foreshadowing,
    Rampup,
    Active,
    Fade,
}
