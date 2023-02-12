use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GameMode {
    Mission,
    STF,
    Domination,
    Race,
}
