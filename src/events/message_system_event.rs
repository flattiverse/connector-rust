use serde_derive::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageSystemEvent {
    #[serde(skip_serializing, default = "SystemTime::now")]
    pub time: SystemTime,
    pub message: String,
}
