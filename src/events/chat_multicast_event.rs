use crate::players::PlayerId;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMulticastEvent {
    pub source: PlayerId,
    pub message: String,
}