use crate::players::PlayerId;
use serde_derive::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMulticastEvent {
    #[serde(skip_serializing, default = "SystemTime::now")]
    pub time: SystemTime,
    pub source: PlayerId,
    pub message: String,
}
