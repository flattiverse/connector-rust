use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum GameMode {
    Mission,
    STF,
    Domination,
    Race,
}
