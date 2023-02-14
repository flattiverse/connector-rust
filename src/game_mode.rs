use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum GameMode {
    Mission,
    STF,
    Domination,
    Race,
}
