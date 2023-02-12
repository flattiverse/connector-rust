use crate::players::PlayerId;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatUnicastEvent {
    pub source: PlayerId,
    pub message: String,
    pub destination: PlayerId,
}