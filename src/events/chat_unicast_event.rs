use crate::players::PlayerId;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatUnicastEvent {
    pub source: PlayerId,
    pub message: String,
    pub destination: PlayerId,
}
