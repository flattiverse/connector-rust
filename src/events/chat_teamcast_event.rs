use crate::players::PlayerId;
use crate::team::TeamId;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatTeamcastEvent {
    pub source: PlayerId,
    pub message: String,
    pub destination: TeamId,
}
